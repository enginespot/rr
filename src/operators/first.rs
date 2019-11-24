use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn first(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
