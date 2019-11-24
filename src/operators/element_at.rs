use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn element_at(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
