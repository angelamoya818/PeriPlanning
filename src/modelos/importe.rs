/// Cantidad de dinero en euros, guardada en centimos (issue #17).
///
/// Se guarda como número entero de céntimos y no como decimal porque los
/// decimales de coma flotante no representan bien ciertos valores lo que
/// haría fallar la comparación entre importes.

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
