use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

const LOCKED: bool = true;
const UNLOCKED: bool = false;
struct Mutex<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            locked:AtomicBool::new(UNLOCKED),
            v: UnsafeCell::new(val)
        }
    }

    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // cmp_exch to make (spin then change lock to locked) a single atomic op
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Relaxed, Ordering::Relaxed).is_err()
        {
            // this because bouncing memory between cores is expensive
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                std::thread::yield_now();
            }
            std::thread::yield_now();
        }
        f(unsafe { &mut *self.v.get() })
    }
}
