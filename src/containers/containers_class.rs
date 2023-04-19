use crate::{containers::enum_containers::ContainerTypes, refillers::refiller_class::Refiller};

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Container {
    max_amount: u64,
    total_amount: Arc<Mutex<u64>>,
    cont_type: ContainerTypes,
    refiller: Refiller
}

impl Container {
    pub fn new(max: u64, cont: ContainerTypes) -> Self {
        Self {
            max_amount: max,
            total_amount: Arc::new(Mutex::new(max)),
            cont_type: cont,
            refiller: Refiller::new(cont.get_refiller_capacity(), cont.get_refiller()),
        }
    }


    pub fn serve(&mut self, amount: u64) -> Result<(),String> {
        let mut total_amount = self.total_amount.lock().expect("Problem with container!");
        if amount > *total_amount {
            let to_recharge = self.refiller.recharge(self.max_amount);
            if to_recharge == 0{
                return Err(format!("Nos quedamos sin {:#?}", self.cont_type))
            }else{
                println!(
                    "El contenedor de {:#?} ha sido recargado.",
                    self.cont_type
                );
                *total_amount += to_recharge;
            }
        }
        *total_amount -= amount;
        thread::sleep(Duration::from_millis(*total_amount));
        let percentage: f64 = ((*total_amount) as f64 / (self.max_amount) as f64) * 100.0;
        if percentage < 15.0 {
            println!(
                "El contenedor de {:#?} tiene {}% de contenido.",
                self.cont_type, percentage
            )
        }
        Ok(())
    }
}
