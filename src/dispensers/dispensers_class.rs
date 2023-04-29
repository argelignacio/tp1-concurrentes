#[cfg(test)]
use crate::mocks::contenedor_mock::Container;



#[cfg(not(test))]
use crate::containers::containers_class::Container;
use std::sync::{Arc, Mutex};

/// Este struct representa a los dispensadores de la cafetera, cuya responsabilidad es servir los ingredientes de las ordenes utilizando
/// los contenedores de ingredientes.
pub struct Dispensers {
    containers: Arc<Vec<Arc<Mutex<Container>>>>,
    numb: i32,
}

/// El dispenser necesita conocer los contenedores de ingredientes y su número.
impl Dispensers {
    pub fn new(containers_ref: Arc<Vec<Arc<Mutex<Container>>>>, numb: i32) -> Self {
        Self {
            containers: containers_ref,
            numb,
        }
    }

    /// Esta función se encarga de servir TODOS los ingredientes de las ordenes, a traves de los contenedores de ingredientes.
    pub fn prepare(&mut self, order: Vec<u64>, coffe_act: u64, report: bool) -> Result<(), String> {
        println!(
            "Cafe {} se esta preparando en dispenser {}",
            coffe_act, self.numb
        );
        for (index, amount) in order.iter().enumerate() {       
            if let Ok(mut cont_act) = self.containers[index].lock() {
                cont_act.serve(*amount, report)?;
            } else {
                println!("Err: Por un error interno no se le puede servir un ingrediente.")
            }
        }
        println!("Cafe {} listo", coffe_act);
        Ok(())
    }
}
