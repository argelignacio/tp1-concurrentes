use std::collections::VecDeque;
use std::fs;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use tp1::coffeemaker::coffeemaker_class::CoffeeMaker;
use tp1::containers::containers_class::Container;
use tp1::containers::enum_containers::ContainerTypes;


use tp1::dispensers::dispensers_class::Dispensers;

use std_semaphore::Semaphore;


const N_DISPENSERS: isize = 6;
const MAX_CAPACITY_CONTAINERS: u64 = 100;
const ALERT_CAPACITY: f64 = 15.0;
const PERIOD_TO_REPORT: i32 = 3;


pub fn main() {
    /* Esta función main representa el controlador de una cafetera con N dispensadores conectados a los contenedores
    de cada ingrediente posiblemente necesario en la realización de un cafe. */

    // Leo las orders del archivo orders.txt e inicio contador de cafes
    let orders: Vec<Vec<u64>> = read_orders();

    // Creo un array de containers con su debida capacidad
    let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
    // Creo un array de dispensers con su referencia a los containers que lo proveen
    let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = create_dispensers(containers_vec);
 

    let coffeemaker = CoffeeMaker::new(orders, N_DISPENSERS, dispensers);

    coffeemaker.start_prepare_coffees();
}


pub fn main_old() {
    /* Esta función main representa el controlador de una cafetera con N dispensadores conectados a los contenedores
    de cada ingrediente posiblemente necesario en la realización de un cafe. */

    // Leo las orders del archivo orders.txt e inicio contador de cafes
    let orders: Vec<Vec<u64>> = read_orders();
    let mut coffee_act: u64 = 0;

    // Añado un array de handles para los threads correspondientes a cada orden
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];

    // Creo un array de containers con su debida capacidad
    let containers_vec: Vec<Arc<Mutex<Container>>> = create_containers();
    // Creo un array de dispensers con su referencia a los containers que lo proveen
    let dispensers: VecDeque<Arc<Mutex<Dispensers>>> = create_dispensers(containers_vec);
    let dispensers_ref: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>> =
        Arc::new(Mutex::new(dispensers));
    // Creo un semáforo que va a permitir acceder a los dispensers
    let sem: Arc<Semaphore> = Arc::new(Semaphore::new(N_DISPENSERS));
    let cantidad_cafes: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    // Itero las orders y las inicio en threads
    for order in orders {
        let cantidad_cafes_clone: Arc<Mutex<i32>> = Arc::clone(&cantidad_cafes);
        coffee_act += 1;
        let order_act: Vec<u64> = order.clone();
        let sem_clone: Arc<Semaphore> = Arc::clone(&sem);
        let dispensers_clone: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>> = dispensers_ref.clone();
        let handle: thread::JoinHandle<()> = init_order(
            order_act,
            sem_clone,
            dispensers_clone,
            coffee_act,
            cantidad_cafes_clone,
        );
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Error joining a thread");
    }

    println!("Todos los cafes fueron preparados!");
}


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

fn init_order(
    order_act: Vec<u64>,
    sem_clone: Arc<Semaphore>,
    dispensers_clone: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>>,
    coffee_act: u64,
    cantidad_cafes_clone: Arc<Mutex<i32>>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        sem_clone.acquire();
        let dispenser_guard: Arc<Mutex<Dispensers>> = match dispensers_clone.lock() {
            Ok(mut dispensers_clone_act) => {
                if let Some(dispenser) = dispensers_clone_act.pop_front() {
                    dispenser
                } else {
                    panic!("No hay dispensers, acceso invalido")
                }
            }
            Err(_e) => {
                panic!("No se pudo acceder a los dispensers, no se pueden preparar los cafes");
            }
        };
        if let Ok(dispenser) = dispenser_guard.lock() {
            println!("INFO: Dispenser para la orden {} encontrado.", coffee_act);
            prepare_coffee(dispenser, order_act, coffee_act, cantidad_cafes_clone)
        }
        println!("------------------------------------------------------------------\n\n");

        if let Ok(mut dispensers_clone_act) = dispensers_clone.lock() {
            dispensers_clone_act.push_front(dispenser_guard);
            sem_clone.release()
        } else {
            println!("Por error interno, se perdió un dispenser")
        }
    })
}

fn prepare_coffee(mut dispenser:MutexGuard<Dispensers>, order_act:Vec<u64>, coffee_act: u64, cantidad_cafes_clone: Arc<Mutex<i32>>) {
    if let Ok(mut cant) = cantidad_cafes_clone.lock() {
        if let Err(error) =
            dispenser.prepare(order_act, coffee_act, *cant % PERIOD_TO_REPORT == 0)
        {
            println!(
                "ERR: No se pudo terminar la orden {} debido a un error:",
                coffee_act
            );
            println!("\t {}", error);
        } else {
            *cant += 1;
            println!("INFO: {} cafes hechos hasta ahora.", cant);
            drop(cant);
        }
    } else {
        if let Err(error) = dispenser.prepare(order_act, coffee_act, true) {
            println!(
                "ERR: No se pudo terminar la orden {} debido a un error:",
                coffee_act
            );
            println!("\t {}", error);
        } else {
            println!("ERR: Este cafe fue finalizado pero no pudo ser contado.");
        }
    }
}