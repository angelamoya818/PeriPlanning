#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Importe(u64);

impl Importe {
    pub fn desde_centimos(centimos: u64) -> Self {
        Self(centimos)
    }

    pub fn centimos(&self) -> u64 {
        self.0
    }
}
