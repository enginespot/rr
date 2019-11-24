use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn combine_latest(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
