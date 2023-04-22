use crate::containers::containers_class::Container;
use std::sync::{Arc, Mutex};

pub struct Dispensers {
    containers: Arc<Vec<Arc<Mutex<Container>>>>,
    numb: i32,
}

impl Dispensers {
    pub fn new(containers_ref: Arc<Vec<Arc<Mutex<Container>>>>, numb: i32) -> Self {
        Self {
            containers: containers_ref,
            numb,
        }
    }

    pub fn prepare(&mut self, order: Vec<u64>, coffe_act: u64) -> Result<(), String> {
        println!(
            "Cafe {} se esta preparando en dispenser {}",
            coffe_act, self.numb
        );
        for (index, amount) in order.iter().enumerate() {
            if let Ok(mut cont_act) = self.containers[index].lock() {
                cont_act.serve(*amount)?
            } else {
                println!("Err: Por un error interno no se le puede servir un ingrediente.")
            }
        }
        println!("Cafe {} listo", coffe_act);
        Ok(())
    }
}
