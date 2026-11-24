use crate::periplanning::compania::Compania;
use crate::periplanning::importe::Importe;

#[allow(dead_code)]
pub struct Informe {
    compania: Compania,
    reserva: Option<Importe>,
}
