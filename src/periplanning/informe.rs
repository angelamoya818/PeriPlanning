use crate::periplanning::compania::Compania;
use crate::periplanning::dias_restantes::DiasRestantes;
use crate::periplanning::estado::Estado;
use crate::periplanning::fecha::{FechaApertura, FechaCierre, FechaContacto};
use crate::periplanning::importe::Importe;
use crate::periplanning::numero_siniestro::NumeroSiniestro;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Informe {
    numero_siniestro: NumeroSiniestro,
    compania: Compania,
    estado: Estado,
    dias_restantes: DiasRestantes,
    reserva: Option<Importe>,
    fecha_apertura: FechaApertura,
    fecha_contacto: Option<FechaContacto>,
    fecha_cierre: Option<FechaCierre>,
}
