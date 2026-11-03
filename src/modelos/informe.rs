/// Informe pericial activo en la tabla de HU001.
///
/// Su identidad es el número de siniestro de la tabla. La fila puede cambiar
/// en las actualizaciones diarias, pero sigue representando el mismo expediente.
pub struct Informe {
    siniestro: String,
    estado: String,
    compania: String,
    dias_abierto: i32,
    dias_restantes: i32,
    dias_inactivo: i32,
    tasacion_euros: Option<String>,
}

impl Informe {
    /// Crea un informe a partir de los datos necesarios para el análisis de HU001.
    /// El resto de columnas del CSV se incorporarán cuando se necesiten en el dominio.
    pub fn nuevo(
        siniestro: String,
        estado: String,
        compania: String,
        dias_abierto: i32,
        dias_restantes: i32,
        dias_inactivo: i32,
        tasacion_euros: Option<String>,
    ) -> Result<Self, ErrorInforme> {
        if siniestro.trim().is_empty() {
            return Err(ErrorInforme::SiniestroVacio);
        }

        Ok(Self {
            siniestro,
            estado,
            compania,
            dias_abierto,
            dias_restantes,
            dias_inactivo,
            tasacion_euros,
        })
    }

    pub fn siniestro(&self) -> &str {
        &self.siniestro
    }

    pub fn estado(&self) -> &str {
        &self.estado
    }

    pub fn compania(&self) -> &str {
        &self.compania
    }

    pub fn dias_abierto(&self) -> i32 {
        self.dias_abierto
    }

    pub fn dias_restantes(&self) -> i32 {
        self.dias_restantes
    }

    pub fn dias_inactivo(&self) -> i32 {
        self.dias_inactivo
    }

    pub fn tasacion_euros(&self) -> Option<&str> {
        self.tasacion_euros.as_deref()
    }
}

pub enum ErrorInforme {
    SiniestroVacio,
}
