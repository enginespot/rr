use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn last(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
