use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn flat_map_latest(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
