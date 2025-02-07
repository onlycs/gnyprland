use astal_io::traits::VariableBaseExt;
use glib::value::FromValue;
use gtk::glib::ObjectExt;
use std::{cell::UnsafeCell, marker::PhantomData, ops::Deref};

use crate::prelude::*;
use astal_obj::*;

pub struct Variable<T> {
    value: UnsafeCell<Value>,
    inner: astal_io::Variable,
    phantom: PhantomData<T>,
}

impl<T> Variable<T> {
    pub fn new(data: T) -> Self
    where
        T: Into<Value>,
    {
        let mut value = data.into();
        let inner = astal_io::Variable::new(&mut value);

        Self {
            value: UnsafeCell::new(value),
            inner,
            phantom: PhantomData,
        }
    }

    pub unsafe fn from_astal(variable: astal_io::Variable) -> Self {
        Self {
            value: UnsafeCell::new(variable.value()),
            inner: variable,
            phantom: PhantomData,
        }
    }

    pub fn bind<'a>(&'a self) -> Binding<'a, Self, T>
    where
        T: for<'v> FromValue<'v> + Send + Sync + 'static,
    {
        unsafe { Binding::new(self, "") }
    }

    pub fn set(&self, data: T)
    where
        T: Into<Value>,
    {
        let value = unsafe { &mut *(self.value.get()) };

        *value = data.into();
        self.inner.set_value(value);
        self.inner.emit_changed();
    }
}

unsafe impl<T> BindSource for Variable<T> {
    fn connect(&self, prop: &str, cb: impl ValueConsumer) {
        assert_eq!(
            prop, "",
            "Binding with `Variable` must be initialized with `Variable::bind`"
        );

        self.inner.connect_changed(move |var| {
            let value = var.value();
            cb(&value).unwrap();
        });

        self.inner.emit_changed();
    }

    fn raw_bind(
        &self,
        prop: &str,
        dst: &impl BindDest,
        dstprop: &str,
        transform: impl ValueTransformer,
    ) {
        assert_eq!(
            prop, "",
            "Binding with `Variable` must be initialized with `Variable::bind`"
        );

        self.inner
            .bind_property("value", dst.as_object(), dstprop)
            .transform_to(move |_, val| transform(val).ok())
            .build();

        self.inner.emit_changed();
    }
}

impl<T> Deref for Variable<T> {
    type Target = astal_io::Variable;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
