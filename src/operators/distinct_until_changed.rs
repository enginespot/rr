use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn distinct_until_changed(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
