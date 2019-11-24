use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn count(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
