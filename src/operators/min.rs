use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn min(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
