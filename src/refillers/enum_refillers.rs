#[derive(Debug)]

pub enum RefillerTypes {
    Agua,
    Granos,
    Leche,
    Cacao,
}
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