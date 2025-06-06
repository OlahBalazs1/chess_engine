use std::{
    cell::{OnceCell, UnsafeCell},
    mem::MaybeUninit,
    sync::OnceLock,
};

pub struct StaticData<T>(UnsafeCell<MaybeUninit<T>>);

impl<T> StaticData<T> {
    pub const fn uninit() -> Self {
        StaticData(UnsafeCell::new(MaybeUninit::uninit()))
    }
    pub fn write(&self, data: T) {
        unsafe {
            (*self.0.get()).write(data);
        }
    }

    pub fn get<'a>(&self) -> &'a T {
        unsafe { (*self.0.get()).assume_init_ref() }
    }
}

unsafe impl<T> Sync for StaticData<T> {}
// pub struct StaticData<T>(OnceCell<T>);
// impl<T> StaticData<T> {
//     pub const fn uninit() -> Self {
//         StaticData(OnceCell::new())
//     }
//     pub fn write(&self, data: T) {
//         let _ = self.0.set(data);
//     }
//
//     pub fn get(&self) -> &T {
//         self.0.get().unwrap()
//     }
// }
//
// unsafe impl<T> Sync for StaticData<T> {}
