/// Compañía aseguradora a la que pertenece un expediente (issue #18).
/// La HU001 distingue estas cuatro, el resto las pongo como Otra.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compania {
    Meridiano,
    Santander,
    AxaEmpresas,
    MapfreEmpresas,
    Otra,
}
