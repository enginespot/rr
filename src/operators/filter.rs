use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn filter(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
