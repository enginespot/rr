use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn reduce(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
