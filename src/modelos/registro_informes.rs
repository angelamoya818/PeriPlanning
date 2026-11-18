use std::collections::HashMap;

use crate::modelos::informe::Informe;
use crate::modelos::numero_siniestro::NumeroSiniestro;

#[allow(dead_code)]
pub struct RegistroInformes {
    informes: HashMap<NumeroSiniestro, Informe>,
}
