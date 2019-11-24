use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn skip(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
