use crate::schedulers::{NewThreadScheduler, Scheduler};

pub struct Schedulers;

impl Schedulers {
    pub fn io<'a>() -> impl Scheduler<'a> {
        NewThreadScheduler::new()
    }

    pub fn new_thread<'a>() -> impl Scheduler<'a> {
        NewThreadScheduler::new()
    }

    pub fn main_thread<'a>() -> impl Scheduler<'a> {
        NewThreadScheduler::new()
    }

    pub fn computation<'a>() -> impl Scheduler<'a> {
        NewThreadScheduler::new()
    }

    pub fn immediate<'a>() -> impl Scheduler<'a> {
        NewThreadScheduler::new()
    }
}
