#[derive(Copy, Clone)]
pub enum OverflowStrategy {
    IGNORE,
    ERROR,
    DROP,
    LATEST,
    BUFFER,
}
