use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn distinct(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
