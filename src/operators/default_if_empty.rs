use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn default_if_empty(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
