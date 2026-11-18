use crate::modelos::compania::Compania;
use crate::modelos::dias_restantes::DiasRestantes;
use crate::modelos::estado::Estado;
use crate::modelos::fecha::{FechaApertura, FechaCierre, FechaContacto};
use crate::modelos::importe::Importe;
use crate::modelos::numero_siniestro::NumeroSiniestro;

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
