use crate::refillers::enum_refillers::RefillerTypes;
const CAPACITY_REFILLER_AGUA: u64 = 0xFFFFFFFFFFFFFFFF;
const CAPACITY_REFILLER_CACAO: u64 = 0;
const CAPACITY_REFILLER_CAFE: u64 = 100;
const CAPACITY_REFILLER_LECHE: u64 = 100;

#[derive(Debug, Clone, Copy)]

/// Este enum representa los tipos de contenedores que existen en la máquina.
pub enum ContainerTypes {
    Agua,
    Cafe,
    Espuma,
    Cacao,
}

impl ContainerTypes {
    /// Esta función retorna un iterador de los tipos de contenedores. Esto da un orden de servicio a los ingredientes.
    pub fn iter() -> impl Iterator<Item = ContainerTypes> {
        vec![
            ContainerTypes::Cafe,
            ContainerTypes::Agua,
            ContainerTypes::Espuma,
            ContainerTypes::Cacao,
        ]
        .into_iter()
    }
    /// Esta función retorna el tipo de recargador que necesita el contenedor.
    pub fn get_refiller(&self) -> RefillerTypes {
        match self {
            ContainerTypes::Agua => RefillerTypes::Agua,
            ContainerTypes::Cacao => RefillerTypes::Cacao,
            ContainerTypes::Cafe => RefillerTypes::Granos,
            ContainerTypes::Espuma => RefillerTypes::Leche,
        }
    }
    /// Esta función retorna la capacidad máxima del contenedor.
    pub fn get_refiller_capacity(&self) -> u64 {
        match self {
            ContainerTypes::Agua => CAPACITY_REFILLER_AGUA,
            ContainerTypes::Cacao => CAPACITY_REFILLER_CACAO,
            ContainerTypes::Cafe => CAPACITY_REFILLER_CAFE,
            ContainerTypes::Espuma => CAPACITY_REFILLER_LECHE,
        }
    }
}
