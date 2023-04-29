#[cfg(test)]

mod tests {
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex, RwLock},
        vec,
    };

    use tp1::{
        coffeemaker::{coffeemaker_class::CoffeeMaker, report_maker::ReportMaker},
        containers::{containers_class::Container, enum_containers::ContainerTypes},
        dispensers::{dispensers_class::Dispensers},
        mocks,
    };

    #[test]
    fn test_start_prepare_coffees() {
        let orders = vec![vec![1, 1, 1, 1], vec![1, 1, 1, 1], vec![1, 1, 1, 1]];
        let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
        let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = create_dispensers(containers_vec.clone());
        let coffeemaker = CoffeeMaker::new(orders, 2, dispensers);
        let report_maker = ReportMaker::new();
        let report = report_maker.start_reports(Arc::new(Mutex::new(containers_vec.to_vec())));
        let report_maker_clone = Arc::new(RwLock::new(report_maker));
        coffeemaker.start_prepare_coffees(&report_maker_clone);
        if let Err(e) = report.join() {
            println!("Error: {:?}", e);
        }
        
    }
    /// Esta función crea los contenedores de ingredientes de la cafetera.
    fn create_containers() -> Vec<Arc<Mutex<Container>>> {
        let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
        for container_type in ContainerTypes::iter() {
            let container: Arc<Mutex<Container>> = Arc::new(Mutex::new(Container::new(3, container_type, 15.0)));
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
}
