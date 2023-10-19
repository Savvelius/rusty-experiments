use std::collections::VecDeque;
use std::thread::JoinHandle;
use crate::chans::{channel, Receiver, Sender};

mod chans;
mod potters;
mod atomics;
mod hatiko;

// obj - event_loop in main thread:
// obj.schedule(func: fn(...) -> T, )

struct Res<T> {
    result: T,
    id: isize
}

struct Worker<T: Send> {
    worker: JoinHandle<T>,
    chan: Sender<Res<T>>,
    available: bool
}

impl<T: Send> Worker<T> {
    fn new(f: fn() -> T, send: Sender<Res<T>>) -> Self {
        Self {
            worker: std::thread::spawn(f),
            chan: send,
            available: true
        }
    }
}

struct EventQueue<T: Send> {
    queue: VecDeque<Worker<T>>,
    max_workers: usize
}

impl<T: Send> EventQueue<T> {
    fn new() -> Self {
        let max_workers = std::thread::available_parallelism().unwrap().get() - 2;
        if max_workers < 2 {
            panic!("This is useless, not enough threads on your computer");
        }
        Self {
            queue: VecDeque::new(),
            max_workers
        }
    }

    fn run(&mut self) -> Receiver<Res<T>> {
        let (sen, rec) = channel::<Res<T>>();
        let f = || 1;
        for _ in 0..self.max_workers {
            self.queue.push_back(
                Worker::new(f.clone(), sen.clone())
            )
        }
        rec
    }
}

fn main() {
}











