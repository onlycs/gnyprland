use crate::prelude::*;
use glib::{value::FromValue, Value};
use gtk::{glib::IsA, traits::CssProviderExt, CssProvider};
use std::{
    any::Any,
    backtrace::Backtrace,
    error::Error,
    fmt::Debug,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    panic::Location,
    sync::Arc,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("At {location}: Error in transform function:\n{source}")]
    Transform {
        #[from]
        source: Box<dyn Error>,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },

    #[error("FromValue failed:\n{0}")]
    FromValue(Box<dyn Error>),

    #[error("At {location}: GLib error:\n{source}")]
    Glib {
        #[from]
        source: glib::Error,
        location: &'static Location<'static>,
        backtrace: Backtrace,
    },
}

type Result<T, E = TransformError> = std::result::Result<T, E>;
pub trait ThreadSafe = Send + Sync + 'static;
trait Transformer = for<'v> Fn(&'v Value) -> Result<Box<dyn Any>> + ThreadSafe;
pub trait ValueTransformer = for<'v> Fn(&'v Value) -> Result<Value> + ThreadSafe;
pub trait ValueConsumer = for<'v> Fn(&'v Value) -> Result<()> + 'static;
trait OrElse<T> = Fn() -> Option<T> + ThreadSafe;

mod private {
    use gtk::glib::ObjectExt;
    use gtk::glib::ObjectType;

    use super::*;

    pub trait BindDest {
        fn set_notify(&self, prop: &str, value: Value);
        fn as_object(&self) -> &impl ObjectType;
    }

    impl<O: ObjectType> BindDest for O {
        fn set_notify(&self, prop: &str, value: Value) {
            self.set_property(prop, value);
            self.notify(prop);
        }

        fn as_object(&self) -> &impl ObjectType {
            self
        }
    }

    pub trait BindSource {
        fn connect(&self, prop: &str, cb: impl ValueConsumer);
        fn raw_bind(
            &self,
            prop: &str,
            dst: &impl BindDest,
            dstprop: &str,
            transform: impl ValueTransformer,
        );
    }

    impl<O: ObjectType> BindSource for O {
        fn connect(&self, prop: &str, cb: impl ValueConsumer) {
            unsafe {
                self.connect_notify_unsafe(Some(prop), move |obj, param| {
                    let param = param.name();
                    let value = obj.property_value(&param);
                    cb(&value).unwrap();
                });
            }
        }

        fn raw_bind(
            &self,
            prop: &str,
            dst: &impl BindDest,
            dstprop: &str,
            transform: impl ValueTransformer,
        ) {
            self.bind_property(prop, dst.as_object(), dstprop)
                .transform_to(move |_, value| transform(value).ok())
                .build();
        }
    }
}

pub use private::*;

pub trait Bind: BindSource + Sized {
    fn bind<'a, T: for<'v> FromValue<'v> + 'static>(
        &'a self,
        prop: &'a str,
    ) -> Binding<'a, Self, T> {
        Binding::new(self, prop)
    }
}

impl<S: BindSource> Bind for S {}

pub struct Binding<'a, S, T> {
    object: &'a S,
    prop: &'a str,

    transform: Arc<dyn Transformer>,
    or_else: Arc<dyn OrElse<T>>,

    _phantom: PhantomData<T>,
}

impl<'a, S, T> Clone for Binding<'a, S, T> {
    fn clone(&self) -> Self {
        Self {
            object: self.object,
            prop: self.prop,
            transform: Arc::clone(&self.transform),
            or_else: Arc::clone(&self.or_else),
            _phantom: PhantomData,
        }
    }
}

impl<'a, S: BindSource, T> Binding<'a, S, T> {
    fn new(object: &'a S, prop: &'a str) -> Self
    where
        T: for<'v> FromValue<'v> + 'static,
    {
        Self {
            object,
            prop,
            transform: Arc::new(|val| {
                Ok(val
                    .get::<T>()
                    .map(Box::new)
                    .map_err(Box::new)
                    .map_err(|src| TransformError::FromValue(src))?)
            }),
            or_else: Arc::new(|| None),
            _phantom: PhantomData,
        }
    }

    pub fn transform<K, F>(&self, func: F) -> Binding<'a, S, K>
    where
        T: 'static,
        K: 'static,
        F: Fn(T) -> K + Send + Sync + 'static,
    {
        self.transform_err(move |val| Ok(func(val)))
    }

    pub fn transform_err<K, F>(&self, func: F) -> Binding<'a, S, K>
    where
        T: 'static,
        K: 'static,
        F: Fn(T) -> Result<K, Box<dyn Error>> + Send + Sync + 'static,
    {
        let func = Arc::new(func);
        let func_clone = Arc::clone(&func);

        let or_else = Arc::clone(&self.or_else);
        let or_else = Arc::new(move || {
            let val = or_else()?;
            Some(func_clone(val).ok()?)
        });

        let transform = Arc::clone(&self.transform);
        let transform: Arc<dyn Transformer> = Arc::new(move |val| {
            let val = transform(val)?;
            let val = val.downcast::<T>().unwrap();
            let val = func(*val)?;

            Ok(Box::new(val))
        });

        Binding {
            object: self.object,
            prop: self.prop,
            transform,
            or_else,
            _phantom: PhantomData,
        }
    }

    pub fn connect<F>(&self, f: F)
    where
        T: 'static,
        F: Fn(T) + Send + Sync + 'static,
    {
        unsafe {
            self.connect_unsafe(f);
        }
    }

    pub unsafe fn connect_unsafe<F>(&self, f: F)
    where
        T: 'static,
        F: Fn(T) + 'static,
    {
        let transform = Arc::clone(&self.transform);
        let or_else = Arc::clone(&self.or_else);

        self.object
            .connect(self.prop.replace("_", "-").as_str(), move |value| {
                let value = transform(&value);
                let downcast = value.map(|t| t.downcast::<T>().unwrap());

                match downcast {
                    Ok(value) => f(*value),
                    _ if let Some(value) = or_else() => f(value),
                    Err(err) => eprintln!("Error in binding: {}", err),
                }

                Ok(())
            });
    }

    pub fn or(&self, data: T) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        Self {
            or_else: Arc::new(move || Some(data.clone())),
            ..self.clone()
        }
    }

    pub fn or_else(&self, f: impl Fn() -> T + Send + Sync + 'static) -> Self
    where
        T: Send + Sync + Clone + 'static,
    {
        Self {
            or_else: Arc::new(move || Some(f())),
            ..self.clone()
        }
    }

    pub fn bind(&self, other: &'a impl BindDest, other_prop: &'a str)
    where
        T: Into<Value> + 'static,
    {
        let transform = Arc::clone(&self.transform);
        let or_else = Arc::clone(&self.or_else);

        self.object
            .raw_bind(self.prop, other, other_prop, move |val| {
                match transform(val) {
                    Ok(val) => Ok((*val.downcast::<T>().unwrap()).into()),
                    _ if let Some(val) = or_else() => Ok(val.into()),
                    Err(e) => Err(e),
                }
            });
    }
}

impl<'a, S: BindSource> Binding<'a, S, String> {
    pub fn bind_css(self, widget: &'a impl IsA<gtk::Widget>) {
        let provider = forever(CssProvider::new());

        widget
            .style_context()
            .add_provider(provider, gtk::STYLE_PROVIDER_PRIORITY_USER);

        // SAFETY: provider lives for 'static, so does widget
        unsafe {
            self.connect_unsafe(move |css| {
                provider
                    .load_from_data(format!("* {{ {} }}", css).as_bytes())
                    .unwrap();
            });
        }
    }
}

impl<'a, S: BindSource> Binding<'a, S, Vec<String>> {
    pub fn bind_class_name(self, widget: &'a impl IsA<gtk::Widget>) {
        let context = widget.style_context();

        // SAFETY: context lives for the same length as widget, which is forever
        unsafe {
            self.connect_unsafe(move |classes| {
                let old_classes = context.list_classes();

                for class in old_classes {
                    context.remove_class(&class);
                }

                for class in classes {
                    context.add_class(class.as_str());
                }
            });
        }
    }
}
