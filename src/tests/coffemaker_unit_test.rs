#[cfg(test)]
pub mod unit_tests_coffeemaker {
    use crate::{mocks::contenedor_mock::Container, containers::enum_containers::ContainerTypes};
    use crate::mocks::dispenser_mock::DispensersMock::Dispensers;
    use std::{sync::{Arc, Mutex}, collections::VecDeque};

    use crate::coffeemaker::coffeemaker_class::CoffeeMaker;
    /// Esta función crea los contenedores de ingredientes de la cafetera.
    fn create_containers() -> Vec<Arc<Mutex<Container>>> {
        let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
        for container_type in ContainerTypes::iter() {
            let container: Arc<Mutex<Container>> = Arc::new(Mutex::new(Container::new()));
            containers_vec.push(container);
        }
        containers_vec
    }

    /// Esta función crea los dispensadores de la cafetera.
    fn create_dispensers(
        containers_vec: Vec<Arc<Mutex<Container>>>,
    ) -> VecDeque<Arc<Mutex<Dispensers>>> {
        let mut i: i32 = 0;
        let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = (0..2)
            .map(|_| {
                i += 1;
                let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
                    Arc::clone(&Arc::new(containers_vec.clone()));
                Arc::new(Mutex::new(Dispensers::new(containers_ref, i)))
            })
            .collect();
        dispensers
    }

    #[test]
    fn test_start_prepair_coffees_well() {
        let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
        let orders: Vec<Vec<u64>> =vec![vec![1u64;4];4];
        let containers_ref = Arc::clone(&Arc::new(containers_vec.clone()));
        let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = create_dispensers(containers_vec);
        let coffeemaker = CoffeeMaker::new(orders, 2, dispensers);
        // let report_maker = ReportMaker::new();
        // CoffeeMaker::new(, 1)
    }
}
