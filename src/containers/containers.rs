use std::sync::{Arc, Mutex};
use crate::containers::enum_containers::ContainerTypes;

pub struct Container {
    total_amount: Arc<Mutex<u64>>,
    cont_type: ContainerTypes
}

impl Container {
    pub fn new(max: u64, cont: ContainerTypes) -> Self {
        Self {
            total_amount: Arc::new(Mutex::new(max)),
            cont_type: cont
        }
    }

    pub fn serve(&self, amount:u64 ) {
        let mut total_amount = self.total_amount.lock().expect("Problem with container!");
        *total_amount -= amount;
        println!("{}",total_amount);
    }
}