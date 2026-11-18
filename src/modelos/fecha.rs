#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fecha {
    anio: u16,
    mes: u8,
    dia: u8,
}

impl Fecha {
    pub fn new(anio: u16, mes: u8, dia: u8) -> Self {
        Self { anio, mes, dia }
    }

    pub fn anio(&self) -> u16 {
        self.anio
    }

    pub fn mes(&self) -> u8 {
        self.mes
    }

    pub fn dia(&self) -> u8 {
        self.dia
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaApertura(pub Fecha);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaContacto(pub Fecha);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaCierre(pub Fecha);
