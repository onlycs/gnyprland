use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::{
        atomic::{self, AtomicUsize},
        Arc,
    },
};

#[derive(Debug)]
pub struct Id<T> {
    id: u64,
    refs: Arc<AtomicUsize>,
    _ty: PhantomData<T>,
}

unsafe impl<T> Send for Id<T> {}
unsafe impl<T> Sync for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        self.refs.fetch_add(1, atomic::Ordering::Relaxed);

        Self {
            id: self.id,
            refs: Arc::clone(&self.refs),
            _ty: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct Container<T> {
    ptr: *mut dyn Any,
    _ph: PhantomData<T>,
}

impl<T> Deref for Container<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let ptr = self.ptr as *const T;
            &*ptr
        }
    }
}

impl<T> DerefMut for Container<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let ptr = self.ptr as *mut T;
            &mut *ptr
        }
    }
}

impl<T> Drop for Id<T> {
    fn drop(&mut self) {
        self.refs.fetch_sub(1, atomic::Ordering::Relaxed);
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
        refs: Arc::new(AtomicUsize::new(1)),
        _ty: PhantomData,
    }
}

pub fn get<T: 'static>(id: &Id<T>) -> &'static mut T {
    let any = get_ggc().get(&id.id).unwrap();
    unsafe { &mut *(*any as *mut T) }
}
