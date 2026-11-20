use std::collections::HashMap;

use crate::modelo::informe::Informe;
use crate::modelo::numero_siniestro::NumeroSiniestro;

#[allow(dead_code)]
pub struct RegistroInformes {
    informes: HashMap<NumeroSiniestro, Informe>,
}
