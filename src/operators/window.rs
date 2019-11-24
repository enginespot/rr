use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn window(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
