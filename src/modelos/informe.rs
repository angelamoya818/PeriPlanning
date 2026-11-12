use crate::modelos::compania::Compania;
use crate::modelos::dias_restantes::DiasRestantes;
use crate::modelos::estado::Estado;
use crate::modelos::importe::Importe;
use crate::modelos::numero_siniestro::NumeroSiniestro;

/// Expediente de peritación que aparece en la tabla diaria (issue #19).
///
/// Es una entidad: se identifica por su número de siniestro.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Informe {
    numero_siniestro: NumeroSiniestro,
    compania: Compania,
    estado: Estado,
    dias_restantes: DiasRestantes,
    reserva: Option<Importe>,
}

impl Informe {
    pub fn new(
        numero_siniestro: NumeroSiniestro,
        compania: Compania,
        estado: Estado,
        dias_restantes: DiasRestantes,
        reserva: Option<Importe>,
    ) -> Self {
        Self {
            numero_siniestro,
            compania,
            estado,
            dias_restantes,
            reserva,
        }
    }

    pub fn numero_siniestro(&self) -> &NumeroSiniestro {
        &self.numero_siniestro
    }
}

impl PartialEq for Informe {
    fn eq(&self, otro: &Self) -> bool {
        self.numero_siniestro == otro.numero_siniestro
    }
}

impl Eq for Informe {}
