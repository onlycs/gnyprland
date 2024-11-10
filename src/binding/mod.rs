use glib::{value::FromValue, Binding, Value};
use gtk::glib::{ObjectExt, ObjectType};
use std::{marker::PhantomData, sync::Arc};

pub struct BindData<'a, O, T>
where
    O: ObjectType,
{
    object: &'a O,
    prop: &'a str,
    transform: Arc<dyn for<'v> Fn(&'v Value) -> Value + Send + Sync + 'static>,
    _phantom: PhantomData<T>,
}

impl<'a, O, T> Clone for BindData<'a, O, T>
where
    O: ObjectType,
{
    fn clone(&self) -> Self {
        Self {
            object: self.object,
            prop: self.prop,
            transform: Arc::clone(&self.transform),
            _phantom: PhantomData,
        }
    }
}

impl<'a, O, T> BindData<'a, O, T>
where
    O: ObjectType,
{
    fn new(object: &'a O, prop: &'a str) -> Self {
        Self {
            object,
            prop,
            transform: Arc::new(|val| unsafe { Value::from_value(val) }),
            _phantom: PhantomData,
        }
    }

    pub fn transform<K, F>(&self, func: F) -> BindData<'a, O, K>
    where
        T: for<'v> FromValue<'v> + 'static,
        K: Into<Value> + 'static,
        F: Fn(T) -> K + Send + Sync + 'static,
    {
        let transform = Arc::clone(&self.transform);

        BindData {
            object: self.object,
            prop: self.prop,
            transform: Arc::new(move |value| {
                let value = (transform)(value);
                let typed = func(unsafe { T::from_value(&value) });
                typed.into()
            }),
            _phantom: PhantomData,
        }
    }

    pub fn bind(self, other: &'a impl ObjectType, other_prop: &'a str) -> Binding
    where
        T: Into<Value> + 'static,
    {
        let builder = self.object.bind_property(self.prop, other, other_prop);
        let transform = self.transform;

        builder
            .transform_to(move |_b, value| Some(transform(value)))
            .build()
    }
}

pub trait BindExt<'a, 'f>
where
    Self: ObjectType,
{
    fn bind<T>(&'a self, prop: &'a str) -> BindData<'a, Self, T>;
}

impl<'a, 'f, O> BindExt<'a, 'f> for O
where
    O: ObjectType,
{
    fn bind<T>(&'a self, prop: &'a str) -> BindData<'a, Self, T> {
        BindData::new(self, prop)
    }
}
