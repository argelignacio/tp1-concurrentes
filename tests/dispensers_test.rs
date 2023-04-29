#[cfg(test)]

mod tests {
    use std::{
        sync::{Arc, Mutex},
        vec,
    };

    use tp1::{
        containers::{containers_class::Container, enum_containers::ContainerTypes},
        dispensers::dispensers_class::Dispensers,
    };
   
    
    // Test de integracion
    
    #[test]
    fn test_prepare_coffee() {
        let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
        for container_type in ContainerTypes::iter() {
            let container: Arc<Mutex<Container>> =
                Arc::new(Mutex::new(Container::new(100, container_type, 15.0)));
            containers_vec.push(container);
        }
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        let order = vec![10, 10, 10, 10];
        if let Ok(mut dispenser) = dispenser.lock(){
            let coffee_act = 1;
            let report = false;
            assert_eq!(dispenser.prepare(order, coffee_act, report), Ok(()));
        };
    }
    #[test]
    fn test_fail_prepare_coffee_container_capacity_exceeded() {
        let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
        for container_type in ContainerTypes::iter() {
            let container: Arc<Mutex<Container>> =
                Arc::new(Mutex::new(Container::new(100, container_type, 15.0)));
            containers_vec.push(container);
        }
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        let order = vec![101, 10, 10, 10];
        if let Ok(mut dispenser) = dispenser.lock(){
            let coffee_act = 1;
            let report = false;
            assert_eq!(dispenser.prepare(order, coffee_act, report), Err(String::from("La cantidad de Cafe es mayor a la capacidad del contenedor.")));
        };
    }
    #[test]
    fn test_fail_prepare_coffee_container_empty() {
        let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
        for container_type in ContainerTypes::iter() {
            let container: Arc<Mutex<Container>> =
                Arc::new(Mutex::new(Container::new(100, container_type, 15.0)));
            containers_vec.push(container);
        }
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispenser = Arc::new(Mutex::new(Dispensers::new(containers_ref, 1)));
        let order = vec![100, 10, 10, 100];
        if let Ok(mut dispenser) = dispenser.lock(){
            let coffee_act = 1;
            let report = false;
            assert_eq!(dispenser.prepare(order.clone(), coffee_act, report), Ok(()));
            assert_eq!(dispenser.prepare(order.clone(), coffee_act, report), Err(String::from("Nos quedamos sin Cacao.")));
        };
    }
}
