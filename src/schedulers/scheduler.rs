use crate::util::{RRResult, Task};

pub trait Scheduler<'a> {
    fn schedule(&self, func: Box<Task<'a>>) -> RRResult<()>;

    fn start(&self) -> RRResult<()>;

    fn stop(&self) -> RRResult<()>;
}
