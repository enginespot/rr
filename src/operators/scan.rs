use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn scan(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
