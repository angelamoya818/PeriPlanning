use crate::periplanning::compania::Compania;
use crate::periplanning::dias_restantes::DiasRestantes;
use crate::periplanning::estado::Estado;
use crate::periplanning::importe::Importe;

#[allow(dead_code)]
pub struct Informe {
    compania: Compania,
    reserva: Option<Importe>,
    stado: Estado,
    dias_restantes: DiasRestantes,
}
