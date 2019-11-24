use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn unpack(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
