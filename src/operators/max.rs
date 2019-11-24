use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn max(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
