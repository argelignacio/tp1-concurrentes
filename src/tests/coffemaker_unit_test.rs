#[cfg(test)]
pub mod coffemaker_unit_test {
    use crate::mocks::dispenser_mock::DispensersMock::Dispensers;
    use crate::mocks::report_mock::ReportMock::ReportMaker;
    use crate::tests;
    use crate::{containers::enum_containers::ContainerTypes, mocks::contenedor_mock::Container};
    use std::os::unix::thread;
    use std::sync::RwLock;
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    use crate::coffeemaker::coffeemaker_class::CoffeeMaker;

    #[test]
    fn test_start_prepair_coffees_well() {
        let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
        let orders: Vec<Vec<u64>> = vec![vec![1u64; 4]; 2];
        let dispensers: VecDeque<Arc<Mutex<Dispensers>>> =
            create_dispensers(containers_vec.clone());
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispensers_ref = dispensers.clone();
        let coffeemaker = CoffeeMaker::new(orders.clone(), 2, dispensers_ref.clone());
        let mut report_maker = ReportMaker::new();
        let report = report_maker.start_reports(Arc::new(Mutex::new(containers_ref.to_vec())));
        let report_maker_clone = Arc::new(RwLock::new(report_maker.clone()));
        coffeemaker.start_prepare_coffees(&report_maker_clone);

        if let Ok(rep) = report_maker_clone.read() {
            assert_eq!(rep.report, false);
            assert_eq!(rep.call_start_report, true);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        if let Ok(disp) = dispensers_ref[0].lock() {
            assert_eq!(disp.coffe_act<=2, true);
            assert_eq!(orders.contains(&disp.order) , true);
        }
        if let Ok(disp) = dispensers_ref[1].lock() {
            assert_eq!(disp.coffe_act<=2, true);
            assert_eq!(orders.contains(&disp.order), true);
        }
        if let Err(e) = report.join() {
            println!("Error: {:?}", e);
        }
        
    }
    #[test]    
    fn test_start_no_prepare_empty_orders() {
        let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
        let orders: Vec<Vec<u64>> = vec![vec![1u64; 0]; 0];
        let dispensers: VecDeque<Arc<Mutex<Dispensers>>> =
            create_dispensers(containers_vec.clone());
        let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
            Arc::clone(&Arc::new(containers_vec.clone()));
        let dispensers_ref = dispensers.clone();
        let coffeemaker = CoffeeMaker::new(orders.clone(), 2, dispensers_ref.clone());
        let mut report_maker = ReportMaker::new();
        let report = report_maker.start_reports(Arc::new(Mutex::new(containers_ref.to_vec())));
        let report_maker_clone = Arc::new(RwLock::new(report_maker.clone()));
        coffeemaker.start_prepare_coffees(&report_maker_clone);

        if let Ok(rep) = report_maker_clone.read() {
            assert_eq!(rep.report, false);
            assert_eq!(rep.call_start_report, true);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        
        if let Ok(disp) = dispensers_ref[0].lock() {
            assert_eq!(disp.coffe_act==0, true);
            assert_eq!(disp.called , false);
            assert_eq!(disp.order, [0,0,0,0]);
        }
        if let Ok(disp) = dispensers_ref[1].lock() {
            assert_eq!(disp.coffe_act==0, true);
            assert_eq!(disp.called, false);
            assert_eq!(disp.order, [0,0,0,0]);
        }
        if let Err(e) = report.join() {
            println!("Error: {:?}", e);
        }
        
    }
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
}
