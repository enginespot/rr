use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn runner(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
