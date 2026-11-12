/// Número que identifica un expediente entre las cargas diarias (issue #14).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NumeroSiniestro(String);

impl NumeroSiniestro {
    pub fn new(valor: String) -> Self {
        Self(valor)
    }

    pub fn valor(&self) -> &str {
        &self.0
    }
}
