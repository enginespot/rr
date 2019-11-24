use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn with(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
