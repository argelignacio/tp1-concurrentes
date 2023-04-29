#[cfg(test)]
pub mod DispensersMock {
    use crate::mocks::contenedor_mock::Container;
    use std::sync::{Arc, Mutex};

    /// Este struct representa a los dispensadores de la cafetera, cuya responsabilidad es servir los ingredientes de las ordenes utilizando
    /// los contenedores de ingredientes.
    pub struct Dispensers {
        pub order: Vec<u64>,
        pub coffe_act: u64,
        pub report: bool,
    }

    /// El dispenser necesita conocer los contenedores de ingredientes y su número.
    impl Dispensers {
        pub fn new(containers_ref: Arc<Vec<Arc<Mutex<Container>>>>, numb: i32) -> Self {
            Self {
                order: vec![0, 0, 0, 0],
                coffe_act: 0,
                report: false,
            }
        }

        /// Esta función se encarga de servir TODOS los ingredientes de las ordenes, a traves de los contenedores de ingredientes.
        pub fn prepare(
            &mut self,
            order: Vec<u64>,
            coffe_act: u64,
            report: bool,
        ) -> Result<(), String> {
            self.order = order;
            self.coffe_act = coffe_act;
            self.report = report;
            Ok(())
        }
    }
}
