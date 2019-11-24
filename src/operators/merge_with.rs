use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn merge_with(self, source: Publisher<'a, T>) -> Publisher<'a, T> {
        Self::merge(vec![self, source])
    }
}
