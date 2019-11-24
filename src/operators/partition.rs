use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn partition(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
