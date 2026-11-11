/// Días que quedan antes de que el expediente sufra penalización (issue #15).
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
