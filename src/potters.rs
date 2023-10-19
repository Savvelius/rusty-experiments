// use std::alloc::Layout;
// use std::cell::UnsafeCell;
// use std::sync::Arc;
// use std::sync::atomic::AtomicUsize;
//
// pub struct Cell<T> {
//     value: UnsafeCell<T>
// }
//
// // implied by UnsafeCell
// // impl<T> !Sync for Cell<T> {}
//
// impl<T> Cell<T> {
//     pub fn new(value: T) -> Self {
//         Cell { value: UnsafeCell::new(value) }
//     }
//     pub fn set(&self, value: T) {
//         unsafe { *self.value.get() = value; }
//     }
//     pub fn get(&self) -> T
//     where
//         T: Copy { unsafe { *self.value.get() } }
// }
//
// enum State {
//     Exclusive,
//     Shared(usize),
//     Unshared
// }
//
// pub struct RefCell<T> {
//     value: UnsafeCell<T>,
//     state: Cell<State>
// }
//
// struct Ref<'refcell, T> {
//     refcell: &'refcell RefCell<T>
// }
//
// impl<T> Drop for Ref<'_, T> {
//     fn drop(&mut self) {
//         match self.refcell.state.get() {
//             State::Exclusive | State::Unshared => unreachable!(),
//             State::Shared(1) => self.refcell.state.set(State::Unshared),
//             State::Shared(n) => self.refcell.state.set(State::Shared(n - 1))
//         }
//     }
// }
//
// impl<T> RefCell<T> {
//
//     pub fn new(value: T) -> Self {
//         Self {
//             value: UnsafeCell::new(value),
//             state: Cell::new(State::Unshared)
//         }
//     }
//
//     pub fn borrow(&self) -> Option<Ref<T>> {
//         match self.state.get() {
//             State::Unshared => {
//                 self.state.set(State::Shared(1));
//                 Some(Ref { refcell: self })
//             },
//             State::Shared(ref mut ref_count) => {
//                 *ref_count += 1;
//                 Some(Ref { refcell: self })
//             },
//             State::Exclusive => None
//         }
//     }
//
//     pub fn borrow_mut(&self) ->Option<&mut T> {
//         // wont work but I have 0 fucks to implement it
//         match self.state.get() {
//             State::Unshared => {
//                 self.state.set(State::Shared(0));
//                 Some(unsafe { &mut *self.value.get() })
//             },
//             _ => None
//         }
//     }
// }
//
// struct Inner<T> {
//     val: T,
//     count: Cell<usize>
// }
//
// pub struct Rc<T> {
//     inner: *const Inner<T>
// }
//
// impl<T> Rc<T> {
//     pub fn new(val: T) -> Self {
//         let boxed = Box::new(Inner { val, count: Cell::new(1) });
//         Rc{ inner: Box::into_raw(boxed) }
//     }
// }
//
// impl<T> std::ops::Deref for Rc<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &unsafe { &*self.inner }.val
//     }
// }
//
// impl<T> Clone for Rc<T> {
//     fn clone(&self) -> Self {
//         unsafe {
//             (*self.inner).count.set((*self.inner).count.get() + 1);
//         }
//         Rc { inner: self.inner }
//     }
// }
//
// // RwLock - read-write lock
//
// impl<T> Drop for Rc<T> {
//     fn drop(&mut self) {
//         let inner = unsafe { &*self.inner };
//         let ref_count = inner.count.get();
//         if ref_count == 1 {
//             // drop it
//             std::mem::drop(self.inner);
//
//         } else if ref_count == 0 {
//             unreachable!()
//         }
//         else {
//             inner.count.set(ref_count - 1);
//         }
//     }
// }
//
//
// #[test]
// fn test() {
//     let c = Cell::new(2);
//     let r1 = &c;
//     let r2 = &c;
//     r1.set(10);
//     println!("{}", c.get());
// }
//











