use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn delay(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
