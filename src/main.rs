use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

use tp1::containers::containers_class::Container;
use tp1::containers::enum_containers::ContainerTypes;

use tp1::dispensers::dispensers_class::Dispensers;

use std_semaphore::Semaphore;

const N_DISPENSERS: isize = 6;
const MAX_CAPACITY_CONTAINERS: u64 = 100;

fn main() {
    /* Esta funcion main representa el controlador de una cafetera con N dispensadores conectados a los contenedores
    de cada ingrediente posiblemente necesario en la realizacion de un cafe. */

    // Leo las orders del archivo orders.txt e inicio contador de cafes
    let orders: Vec<Vec<u64>> = read_orders();
    let mut coffe_act: u64 = 0;

    // Aniado un array de handles para los threads correspondientes a cada orden
    let mut threads = vec![];

    // Creo un array de containers con su debida capacidad
    let mut containers_vec = vec![];
    for container_type in ContainerTypes::iter() {
        let container = Arc::new(Mutex::new(Container::new(
            MAX_CAPACITY_CONTAINERS,
            container_type,
        )));
        containers_vec.push(container);
    }
    let mut i = 0;
    // Creo un array de dispensers con su referencia a los containers que lo proveen
    let dispensers: Vec<Arc<Mutex<Dispensers>>> = (0..N_DISPENSERS)
        .map(|_| {
            i+= 1;
            let containers_ref = Arc::clone(&Arc::new(containers_vec.clone()));
            Arc::new(Mutex::new(Dispensers::new(containers_ref, i.clone())))
        })
        .collect();

    // Creo un semaforo que va a permitir acceder a los dispensers
    let sem = Arc::new(Semaphore::new(N_DISPENSERS));
    let cantidad_cafes: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    // Itero las orders, pido un dispenser y se prepara la orden en ese dispenser
    for order in orders {
        let cantidad_cafes_clone = Arc::clone(&cantidad_cafes);
        coffe_act += 1;
        println!("\nINFO: Buscando dispenser para cafe {}.\n", coffe_act);
        let order_act: Vec<u64> = order.clone();
        let sem_clone = Arc::clone(&sem);
        let dispensers_clone = Arc::clone(&Arc::new(dispensers.clone()));
        let handle = thread::spawn(move || {
            sem_clone.acquire();
            for dispenser in dispensers_clone.iter() {
                let mut dispenser_act = dispenser.lock().expect("Error al consultar el dispenser.");
                if !dispenser_act.is_busy() {
                    println!("INFO: Dispenser para cafe {} encontrado.", coffe_act);
                    if let Err(error) = dispenser_act.prepare(order_act, coffe_act) {
                        println!(
                            "ERR: No se pudo terminar el cafe {} debido a un error:",
                            coffe_act
                        );
                        println!("\t {}", error);
                    } else {
                        let mut cant = cantidad_cafes_clone
                            .lock()
                            .expect("Error al ver cantidad de cafes.");
                        *cant += 1;
                        println!("INFO: {} cafes hechos hasta ahora.", cant);
                        drop(cant);
                    }

                    break;
                }
            }
            println!("INFO: Finalizacion de preparacion del cafe. Dispenser liberado.");
            println!("------------------------------------------------------------------\n\n");
            sem_clone.release()
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().expect("Error joining a thread");
    }

    println!("Todos los cafes fueron preparados!");
}

fn read_orders() -> Vec<Vec<u64>> {
    let file = fs::read_to_string(std::path::PathBuf::from("src/orders.txt"))
        .expect("Failed to read the file");
    let lines: Vec<&str> = file.split('\n').collect();
    let mut orders: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        let line_orders: Vec<u64> = line
            .split(',')
            .map(|num| num.parse().expect("Ingredient must be integer"))
            .collect();
        orders.push(line_orders);
    }
    orders
}
