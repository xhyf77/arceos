use alloc::sync::Arc;
use core::ops::Deref;

use linked_list::{Adapter, Links, List};

use core::sync::atomic::Ordering;
use alloc::collections::VecDeque;

use crate::BaseScheduler;

use core::sync::atomic::AtomicIsize;
/// A task wrapper for the [`FifoScheduler`].
///
/// It add extra states to use in [`linked_list::List`].
pub struct FifoTask<T> {
    inner: T,
    links: Links<Self>,
    time_slice: AtomicIsize,
}

unsafe impl<T> Adapter for FifoTask<T> {
    type EntryType = Self;

    #[inline]
    fn to_links(t: &Self) -> &Links<Self> {
        &t.links
    }
}

impl<T> FifoTask<T> {
    /// Creates a new [`FifoTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            links: Links::new(),
            time_slice: AtomicIsize::new(5 as isize),
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }

    fn time_slice(&self) -> isize {
        self.time_slice.load(Ordering::Acquire)
    }

    fn reset_time_slice(&self) {
        self.time_slice.store(5 as isize, Ordering::Release);
    }
}

impl<T> Deref for FifoTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A simple FIFO (First-In-First-Out) cooperative scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
/// It internally uses a linked list as the ready queue.
pub struct FifoScheduler<T> {
    ready_queue: VecDeque<Arc<FifoTask<T>>>,
}

impl<T> FifoScheduler<T> {
    /// Creates a new empty [`FifoScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue:  VecDeque::new(),
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "FIFO"
    }
}

impl<T> BaseScheduler for FifoScheduler<T> {
    type SchedItem = Arc<FifoTask<T>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedItem) {
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.ready_queue.pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, preempt: bool) {
        if prev.time_slice() > 0 && preempt {
            self.ready_queue.push_front(prev)
        } else {
            prev.reset_time_slice();
            self.ready_queue.push_back(prev)
        }
    }

    fn task_tick(&mut self, current: &Self::SchedItem) -> bool {
        let old_slice = current.time_slice.fetch_sub(1, Ordering::Release);
        old_slice <= 1
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}
