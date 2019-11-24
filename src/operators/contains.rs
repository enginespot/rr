use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn contains(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
