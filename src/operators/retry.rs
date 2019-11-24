use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn retry(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
