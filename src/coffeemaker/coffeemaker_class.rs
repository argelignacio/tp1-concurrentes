use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, RwLock},
    thread,
};

use std_semaphore::Semaphore;

#[cfg(test)]
use crate::mocks::dispenser_mock::DispensersMock::Dispensers;

#[cfg(not(test))]
use crate::dispensers::dispensers_class::Dispensers;

use super::report_maker::{ReportMaker};


/// Este struct representa la cafetera, cuya responsabilidad es orquestar la preparación de las ordenes llevando un conteo de los cafes preparados.
pub struct CoffeeMaker {
    dispensers_ref: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>>,
    sem: Arc<Semaphore>,
    orders: Vec<Vec<u64>>,
    threads: Vec<thread::JoinHandle<()>>,
    count_coffees_processed: Arc<RwLock<i32>>,
    report: Arc<Mutex<bool>>,
}

/// La cafetera al inicializarse debe conocer los dispensers, su cantidad y las ordenes que va a preparar.
impl CoffeeMaker {
    pub fn new(
        orders: Vec<Vec<u64>>,
        n_dispensers: isize,
        dispensers: VecDeque<Arc<Mutex<Dispensers>>>,
    ) -> Self {
        let threads: Vec<thread::JoinHandle<()>> = vec![];
        let sem: Arc<Semaphore> = Arc::new(Semaphore::new(n_dispensers));
        let report = Arc::new(Mutex::new(false));
        Self {
            orders,
            dispensers_ref: Arc::new(Mutex::new(dispensers)),
            threads,
            sem,
            count_coffees_processed: Arc::new(RwLock::new(0)),
            report,
        }
    }

    /// Esta función inicia el proceso de preparación de las ordenes y manejar los threads resultantes de las mismas.
    pub fn start_prepare_coffees(mut self,mut report_maker: ReportMaker) {
        let mut coffee_act = 0;
        for order in &self.orders {
            let cantidad_cafes_clone: Arc<RwLock<i32>> = Arc::clone(&self.count_coffees_processed);
            coffee_act += 1;
            let order_act: Vec<u64> = order.clone();
            let sem_clone: Arc<Semaphore> = Arc::clone(&self.sem);
            let dispensers_clone: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>> =
                self.dispensers_ref.clone();
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
        report_maker.stop_reports();
        println!("Todos los cafes fueron preparados!");
    }

    /// Esta función se encarga de enviar, a algún dispenser disponible, la orden para poder ser preparada.
    /// Esto incluye el manejo de los dispensers y en base al conteo de cafes preparados, se decide si se debe reportar el estado de los
    /// dispensers.
    pub fn init_order(
        &self,
        order: Vec<u64>,
        sem: Arc<Semaphore>,
        dispensers: Arc<Mutex<VecDeque<Arc<Mutex<Dispensers>>>>>,
        coffee_act: u64,
        cantidad_cafes: Arc<RwLock<i32>>,
    ) -> thread::JoinHandle<()> {
        let report_clone = Arc::clone(&self.report);
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
                let mut well_done = false;
                let mut report_act = false;
                if let Ok(report) = report_clone.lock() {
                    report_act = *report;
                }
                if let Err(error) = dispenser.prepare(order, coffee_act, report_act) {
                    println!(
                        "ERR: No se pudo terminar la orden {} debido a un error:",
                        coffee_act
                    );
                    println!("\t {}", error);
                } else {
                    well_done = true;
                }
                if report_act {
                    if let Ok(mut report) = report_clone.lock() {
                        *report = false;
                    }
                }

                if well_done {
                    if let Ok(mut cant) = cantidad_cafes.write() {
                        *cant += 1;
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
