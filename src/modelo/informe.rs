use crate::modelo::compania::Compania;
use crate::modelo::dias_restantes::DiasRestantes;
use crate::modelo::estado::Estado;
use crate::modelo::fecha::{FechaApertura, FechaCierre, FechaContacto};
use crate::modelo::importe::Importe;
use crate::modelo::numero_siniestro::NumeroSiniestro;

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
