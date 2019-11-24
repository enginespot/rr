use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn tap(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
