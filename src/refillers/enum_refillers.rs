#[derive(Debug)]

/// Este enum representa los tipos de recargadores que existen en la máquina.
pub enum RefillerTypes {
    Agua,
    Granos,
    Leche,
    Cacao,
}
/// Esta función retorna un iterador de los tipos de recargadores. Esto debe ser ordenado de acuerdo al enum de los contenedores.
impl RefillerTypes {
    pub fn iter() -> impl Iterator<Item = RefillerTypes> {
        vec![
            RefillerTypes::Cacao,
            RefillerTypes::Agua,
            RefillerTypes::Granos,
            RefillerTypes::Leche,
        ]
        .into_iter()
    }
}
