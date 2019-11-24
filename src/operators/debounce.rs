use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn debounce(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
