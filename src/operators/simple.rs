use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn simple(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
