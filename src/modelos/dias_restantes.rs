#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiasRestantes(u32);

impl DiasRestantes {
    pub fn new(dias: u32) -> Self {
        Self(dias)
    }

    pub fn dias(&self) -> u32 {
        self.0
    }
}
