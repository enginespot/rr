use std::{
    error::Error,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::{
    util::{FnComplete, FnError, FnNext, RRResult},
    Subscription,
};

pub struct Subscriber<'a, T: 'a + Send> {
    next: Box<FnNext<'a, T>>,
    error: Box<FnError<'a>>,
    completed: Box<FnComplete<'a>>,
    stopped: Arc<AtomicBool>,
}

impl<'a, T: 'a + Send> Subscriber<'a, T> {
    pub fn create(
        next: Box<FnNext<'a, T>>,
        error: Box<FnError<'a>>,
        completed: Box<FnComplete<'a>>,
    ) -> Subscriber<'a, T> {
        Subscriber {
            stopped: Arc::new(AtomicBool::new(false)),
            next,
            error,
            completed,
        }
    }

    pub fn on_subscribe(&self, s: Box<Subscription>) {}

    pub fn on_next(&self, f: T) -> RRResult<()> {
        let stopped = self.stopped.load(Ordering::Acquire);
        if !stopped {
            (self.next)(f)
        } else {
            Ok(())
        }
    }

    pub fn on_error(&self, f: Box<dyn Error + Send>) -> RRResult<()> {
        let stopped = self
            .stopped
            .compare_and_swap(false, true, Ordering::Acquire);
        if !stopped {
            (self.error)(f)
        } else {
            Ok(())
        }
    }

    pub fn on_completed(&self) -> RRResult<()> {
        let stopped = self
            .stopped
            .compare_and_swap(false, true, Ordering::Acquire);
        if !stopped {
            (self.completed)()
        } else {
            Ok(())
        }
    }
}
