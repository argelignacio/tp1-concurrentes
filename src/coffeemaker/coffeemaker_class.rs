use std::{thread, collections::VecDeque, sync::{Arc, Mutex}};

use std_semaphore::Semaphore;

use crate::dispensers::dispensers_class::Dispensers;
const PERIOD_TO_REPORT: i32 = 3;


pub struct CoffeeMaker {
    dispensers_ref: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>>,
    sem: Arc<Semaphore>,
    orders: Vec<Vec<u64>>,
    threads: Vec<thread::JoinHandle<()>>,
    count_coffees_processed: Arc<Mutex<i32>>,
}

impl CoffeeMaker {
    pub fn new(orders: Vec<Vec<u64>>, n_dispensers: isize, dispensers: VecDeque<Arc<Mutex<Dispensers>>>) -> Self{
        let mut threads: Vec<thread::JoinHandle<()>> = vec![];
        let sem: Arc<Semaphore> = Arc::new(Semaphore::new(n_dispensers));
        Self{
            orders,
            dispensers_ref: Arc::new(Mutex::new(dispensers)),
            threads,
            sem,
            count_coffees_processed:Arc::new(Mutex::new(0)),
        }
    }

    pub fn start_prepare_coffees(mut self) {
        let mut coffee_act = 0;
        for order in &self.orders {
            let cantidad_cafes_clone: Arc<Mutex<i32>> = Arc::clone(&self.count_coffees_processed);
            coffee_act += 1;
            let order_act: Vec<u64> = order.clone();
            let sem_clone: Arc<Semaphore> = Arc::clone(&self.sem);
            let dispensers_clone: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>> = self.dispensers_ref.clone();
            let handle: thread::JoinHandle<()> = self.init_order(
                order_act,
                sem_clone,
                dispensers_clone,
                coffee_act,
                cantidad_cafes_clone,
            );
            self.threads.push(handle);
        }
        for handle in self.threads {
            handle.join().expect("Error joining a thread");
        }
        println!("Todos los cafes fueron preparados!");
    }

    pub fn init_order(
        &self,
        order: Vec<u64>,
        sem: Arc<Semaphore>,
        dispensers: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>>,
        coffee_act: u64,
        cantidad_cafes: Arc<Mutex<i32>>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            sem.acquire();
            let dispenser_guard: Arc<Mutex<Dispensers>> = match dispensers.lock() {
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
            if let Ok(mut dispenser) = dispenser_guard.lock() {
                println!("INFO: Dispenser para la orden {} encontrado.", coffee_act);
                if let Ok(mut cant) = cantidad_cafes.lock() {
                    if let Err(error) =
                        dispenser.prepare(order, coffee_act, *cant % PERIOD_TO_REPORT == 0)
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
                    if let Err(error) = dispenser.prepare(order, coffee_act, true) {
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
            println!("------------------------------------------------------------------\n\n");
    
            if let Ok(mut dispensers_clone_act) = dispensers.lock() {
                dispensers_clone_act.push_front(dispenser_guard);
                sem.release()
            } else {
                println!("Por error interno, se perdió un dispenser")
            }
        })
    }
            

}