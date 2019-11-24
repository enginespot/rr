use crate::Publisher;
use std::error::Error;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn flat_map<R, CB>(self, cb: Box<CB>) -> Publisher<'a, R>
    where
        Publisher<'a, R>: 'a + Send,
        R: 'a + Send,
        CB: Fn(T) -> Result<Publisher<'a, R>, Box<dyn Error + Send>> + 'a + Send + Sync,
    {
        self.map(cb).flatten()
    }
}
