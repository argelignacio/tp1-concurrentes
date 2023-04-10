#[derive(Debug)]
pub enum ContainerTypes {
    Agua,
    Cafe,
    Espuma,
    Cacao
}
impl ContainerTypes {
    pub fn iter() -> impl Iterator<Item = ContainerTypes> {
        vec![
            ContainerTypes::Cacao,
            ContainerTypes::Agua,
            ContainerTypes::Cafe,
            ContainerTypes::Espuma,
        ]
        .into_iter()
    }
}