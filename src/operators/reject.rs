use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn reject(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
