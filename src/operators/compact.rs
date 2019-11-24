use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn compact(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
