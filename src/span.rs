#[derive(Clone)]
pub struct Span {
    pub min: i32,
    pub max: i32
}

impl Span {
    pub fn range(&self) -> usize {
        (self.max - self.min) as usize
    }

    pub fn contains(&self, num: i32) -> bool {
        self.min <= num && num <= self.max
    }
}
