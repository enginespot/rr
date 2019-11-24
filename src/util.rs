use crate::error::RRError;
use std::error::Error;

pub type Task<'a> = dyn FnOnce() -> RRResult<()> + 'a + Send;
pub type FnNext<'a, T> = dyn Fn(T) -> RRResult<()> + 'a + Send + Sync;
pub type FnError<'a> = dyn Fn(Box<dyn Error + Send>) -> RRResult<()> + 'a + Send + Sync;
pub type FnComplete<'a> = dyn Fn() -> RRResult<()> + 'a + Send + Sync;
pub type RRResult<T> = Result<T, RRError>;
