use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn zip(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
