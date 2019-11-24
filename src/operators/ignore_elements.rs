use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn ignore_elements(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
