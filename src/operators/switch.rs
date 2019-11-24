use crate::Publisher;

impl<'a, T: 'a + Send> Publisher<'a, T> {
    pub fn switch(self) -> Publisher<'a, T> {
        unimplemented!()
    }
}
