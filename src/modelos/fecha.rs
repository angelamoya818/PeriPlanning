/// Fecha del calendario: año, mes y día (issue #11).
///
/// Lo modelo a mano porque la biblioteca estándar de Rust no tiene un tipo
/// de fecha de calendario, y no se añaden dependencias externas.
/// Derivar Ord compara por año, luego mes y luego día.

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

/// Día en que se abrió el expediente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaApertura(pub Fecha);

/// Día en que se contactó con el asegurado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaContacto(pub Fecha);

/// Día en que se cerró el expediente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FechaCierre(pub Fecha);
