use crate::containers::containers_class::Container;
use std::sync::{Arc, Mutex};

pub struct Dispensers {
    containers: Arc<Vec<Arc<Mutex<Container>>>>,
    busy: Arc<Mutex<bool>>,
    numb: i32
}

impl Dispensers {
    pub fn new(containers_ref: Arc<Vec<Arc<Mutex<Container>>>>, numb: i32) -> Self {
        Self {
            containers: containers_ref,
            busy: Arc::new(Mutex::new(false)),
            numb
        }
    }

    pub fn prepare(&mut self, order: Vec<u64>, coffe_act: u64) -> Result<(), String> {
        *self.busy.lock().expect("Error marcando ocupacion") = true;
        println!("Cafe {} se esta preparando en dispenser {}", coffe_act, self.numb);
        for (index, amount) in order.iter().enumerate() {
            let mut cont_act = self.containers[index]
                .lock()
                .expect("Error trying to use container");
            cont_act.serve(*amount)?
        }
        println!("Cafe {} listo", coffe_act);
        *self.busy.lock().expect("Error marcando ocupacion") = false;
        Ok(())
    }

    pub fn is_busy(&self) -> bool {
        *self.busy.lock().expect("Error al consultar disponibilidad")
    }
}
