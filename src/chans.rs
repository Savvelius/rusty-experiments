use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};

// sync channel - has capacity, if queue is full on send, sender waits

pub struct Sender<T> {
    shared: Arc<Shared<T>>
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
         Self {
             shared: Arc::clone(&self.shared)
         }
    }
}

impl<T> Sender<T> {
    pub fn send(&self, val: T) {
        let mut shared = self.shared.inner.lock().unwrap();
        shared.queue.push_back(val);
        drop(shared);
        self.shared.available.notify_one();
    }
}
pub struct Receiver<T> {
    shared: Arc<Shared<T>>
}

impl<T> Receiver<T> {
    pub fn recv(&self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => break Some(t),
                None if inner.senders == 0 => return None,
                None => inner = self.shared.available.wait(inner).unwrap()
            }
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        self.shared.inner.lock().unwrap().queue.pop_front()
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self { shared: self.shared.clone() }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Mutex::new(Inner{ queue: VecDeque::new(), senders: 1 });
    let shared = Arc::new(Shared {
        inner,
        available: Condvar::new()
    });
    (Sender { shared: shared.clone() }, Receiver { shared })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ping() {
        let (tx, rx) = channel();
        tx.send(2);
        let ans = rx.recv();
        assert_eq!(ans, Some(2));

    }

    #[test]
    fn closed() {
        let (tx, rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None);
    }
}


