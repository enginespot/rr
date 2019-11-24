use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn take_while(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
