#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    data: Vec<u64>,
}

impl From<&[u64]> for Array {
    fn from(v: &[u64]) -> Self {
        Self { data: v.to_vec() }
    }
}
