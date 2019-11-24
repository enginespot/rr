use crate::Publisher;
impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn sum(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
