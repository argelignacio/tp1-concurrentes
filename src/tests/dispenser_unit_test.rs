// Test unitarios
#[cfg(test)]
pub mod dispenser_unit_test {
    use crate::dispensers::dispensers_class::Dispensers;
    use crate::mocks::contenedor_mock::Container;
    use std::sync::{Arc, Mutex};
    #[test]
    fn test_prepare_coffee_calls_well() {
        let containers_vec: Vec<Arc<Mutex<Container>>> =
            vec![Arc::new(Mutex::new(Container::new()))];
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        dispenser.lock().expect("error").prepare(vec![1], 1, false);
        assert_eq!(containers_vec[0].lock().expect("Error en test").amount, 1);
        assert_eq!(
            containers_vec[0].lock().expect("Error en test").report,
            false
        );
    }

    #[test]
    fn test_prepare_coffee_calls_well_2() {
        let containers_vec: Vec<Arc<Mutex<Container>>> =
            vec![Arc::new(Mutex::new(Container::new()))];
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        dispenser.lock().expect("error").prepare(vec![20], 1, true);
        assert_eq!(containers_vec[0].lock().expect("Error en test").amount, 20);
        assert_eq!(
            containers_vec[0].lock().expect("Error en test").report,
            true
        );
    }
    #[test]
    fn test_prepare_coffee_calls_well_with_few_containers() {
        let containers_vec: Vec<Arc<Mutex<Container>>> =
            vec![Arc::new(Mutex::new(Container::new())), Arc::new(Mutex::new(Container::new())), Arc::new(Mutex::new(Container::new()))];
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        dispenser.lock().expect("error").prepare(vec![20, 22, 24], 1, true);
        assert_eq!(containers_vec[0].lock().expect("Error en test").amount, 20);
        assert_eq!(
            containers_vec[0].lock().expect("Error en test").report,
            true
        );
        assert_eq!(containers_vec[1].lock().expect("Error en test").amount, 22);
        assert_eq!(
            containers_vec[1].lock().expect("Error en test").report,
            true
        );
        assert_eq!(containers_vec[2].lock().expect("Error en test").amount, 24);
        assert_eq!(
            containers_vec[2].lock().expect("Error en test").report,
            true
        );
    }
}
