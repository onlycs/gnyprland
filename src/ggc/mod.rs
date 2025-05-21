use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone, Copy)]
pub struct Id<T> {
    id: u64,
    _ty: PhantomData<T>,
}

unsafe impl<T> Send for Id<T> {}
unsafe impl<T> Sync for Id<T> {}

impl<T: 'static> Deref for Id<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        get_static(self)
    }
}

impl<T: 'static> DerefMut for Id<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        get_static(self)
    }
}

type Ggc = HashMap<u64, *mut dyn Any>;
static mut GGC: Option<Ggc> = None;

#[allow(static_mut_refs)]
fn get_ggc() -> &'static mut Ggc {
    unsafe {
        if GGC.is_none() {
            GGC = Some(Ggc::new());
        }

        GGC.as_mut().unwrap()
    }
}

pub fn put<T: 'static>(data: T) -> Id<T> {
    let leak = Box::leak(Box::new(data) as Box<dyn Any>);

    let mut u = rand::random::<u64>();
    while get_ggc().contains_key(&u) {
        u = rand::random::<u64>();
    }

    get_ggc().insert(u, leak);

    Id {
        id: u,
        _ty: PhantomData,
    }
}

pub fn get_static<T: 'static>(id: &Id<T>) -> &'static mut T {
    let any = get_ggc().get(&id.id).unwrap();
    unsafe { &mut *(*any as *mut T) }
}
