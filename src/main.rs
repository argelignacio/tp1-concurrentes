use std::collections::VecDeque;
use std::fs;
use std::sync::{Arc, Mutex, RwLock};

use tp1::coffeemaker::coffeemaker_class::CoffeeMaker;
use tp1::coffeemaker::report_maker::ReportMaker;
use tp1::containers::containers_class::Container;
use tp1::containers::enum_containers::ContainerTypes;

use tp1::dispensers::dispensers_class::Dispensers;

const N_DISPENSERS: isize = 6;
const MAX_CAPACITY_CONTAINERS: u64 = 100;
const ALERT_CAPACITY: f64 = 15.0;

///Esta función inicializa toda la cafetera e inicia el proceso de preparación de las ordenes.
pub fn main() {
    let orders: Vec<Vec<u64>> = read_orders();
    let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
    let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
        Arc::clone(&Arc::new(containers_vec.clone()));
    let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = create_dispensers(containers_vec);
    let coffeemaker = CoffeeMaker::new(orders, N_DISPENSERS, dispensers);
    let report_maker = ReportMaker::new();
    let report = report_maker.start_reports(Arc::new(Mutex::new(containers_ref.to_vec())));
    let report_maker_clone = Arc::new(RwLock::new(report_maker));
    coffeemaker.start_prepare_coffees(&report_maker_clone);
    if let Err(e) = report.join() {
        println!("Error: {:?}", e);
    }
}

/// Esta función lee el archivo de ordenes y devuelve un vector de vectores de u64 que indica la cantidad de cada ingrediente que se necesita para preparar cada orden.
fn read_orders() -> Vec<Vec<u64>> {
    let file: String = fs::read_to_string(std::path::PathBuf::from("src/orders.txt"))
        .expect("Failed to read the file");
    let lines: Vec<&str> = file.split('\n').collect();
    let mut orders: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let line_orders: Vec<u64> = line
            .split(',')
            .map(|num: &str| num.parse().expect("Ingredient must be integer"))
            .collect();
        orders.push(line_orders);
    }
    orders
}

/// Esta función crea los contenedores de ingredientes de la cafetera.
fn create_containers() -> Vec<Arc<Mutex<Container>>> {
    let mut containers_vec: Vec<Arc<Mutex<Container>>> = vec![];
    for container_type in ContainerTypes::iter() {
        let container: Arc<Mutex<Container>> = Arc::new(Mutex::new(Container::new(
            MAX_CAPACITY_CONTAINERS,
            container_type,
            ALERT_CAPACITY,
        )));
        containers_vec.push(container);
    }
    containers_vec
}

/// Esta función crea los dispensadores de la cafetera.
fn create_dispensers(
    containers_vec: Vec<Arc<Mutex<Container>>>,
) -> VecDeque<Arc<Mutex<Dispensers>>> {
    let mut i: i32 = 0;
    let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = (0..N_DISPENSERS)
        .map(|_| {
            i += 1;
            let containers_ref: Arc<Vec<Arc<Mutex<Container>>>> =
                Arc::clone(&Arc::new(containers_vec.clone()));
            Arc::new(Mutex::new(Dispensers::new(containers_ref, i)))
        })
        .collect();
    dispensers
}
