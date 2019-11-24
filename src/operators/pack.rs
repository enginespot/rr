use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn pack(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
