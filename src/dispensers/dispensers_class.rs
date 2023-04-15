use crate::containers::containers_class::Container;
use std::sync::{Arc, Mutex};

pub struct Dispensers {
    containers: Arc<Vec<Arc<Mutex<Container>>>>,
    busy: Arc<Mutex<bool>>,
}

impl Dispensers {
    pub fn new(containers_ref: Arc<Vec<Arc<Mutex<Container>>>>) -> Self {
        Self {
            containers: containers_ref,
            busy: Arc::new(Mutex::new(false)),
        }
    }

    pub fn prepare(&mut self, order: Vec<u64>, coffe_act: u64) {
        *self.busy.lock().expect("Error marcando ocupacion") = true;
        for (index, amount) in order.iter().enumerate() {
            let cont_act = self.containers[index]
                .lock()
                .expect("Error trying to use container");
            cont_act.serve(*amount);
        }
        println!("Cafe {} listo", coffe_act);
        *self.busy.lock().expect("Error marcando ocupacion") = false;
    }
    pub fn is_busy(&self) -> bool {
        *self.busy.lock().expect("Error al consultar disponibilidad")
    }
}
