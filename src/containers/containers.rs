use core::panic;
use std::sync::{Arc, Mutex};
use crate::containers::enum_containers::ContainerTypes;

pub struct Container {
    max_amount : u64,
    total_amount: Arc<Mutex<u64>>,
    cont_type: ContainerTypes
}

impl Container {
    pub fn new(max: u64, cont: ContainerTypes) -> Self {
        Self {
            max_amount: max,
            total_amount: Arc::new(Mutex::new(max)),
            cont_type: cont
        }
    }

    pub fn serve(&self, amount:u64 ) {
        let mut total_amount = self.total_amount.lock().expect("Problem with container!");
        if amount > *total_amount{
            panic!("NECESITA RECARGAR")
        }
        *total_amount -= amount;
        let percentage:f64 = ((*total_amount) as f64 /(self.max_amount)as f64)*100.0;
        if percentage < 15.0{
            println!("El contenedor de {:#?} tiene {}% de contenido.", self.cont_type, percentage)
        }
    }
}