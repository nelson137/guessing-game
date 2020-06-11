#[derive(Clone)]
pub struct Span {
    pub min: u32,
    pub max: u32
}

impl Span {
    pub fn range(&self) -> usize {
        (self.max - self.min) as usize
    }

    pub fn contains(&self, num: u32) -> bool {
        self.min <= num && num <= self.max
    }
}
