/// Situación de un expediente en su seguimiento (issue #15).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Estado {
    Abierto,
    Cerrado,
    Reabierto,
}
