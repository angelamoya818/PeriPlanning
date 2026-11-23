use std::collections::HashMap;

use crate::periplanning::informe::Informe;
use crate::periplanning::numero_siniestro::NumeroSiniestro;

#[allow(dead_code)]
pub struct RegistroInformes {
    informes: HashMap<NumeroSiniestro, Informe>,
}
