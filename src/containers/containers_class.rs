use crate::{containers::enum_containers::ContainerTypes, refillers::refiller_class::Refiller};

use std::{thread, time::Duration};

/// Este struct representa un contenedor de un ingrediente, el cual tiene una capacidad máxima y un tipo de ingrediente.
pub struct Container {
    max_amount: u64,
    total_amount: u64,
    cont_type: ContainerTypes,
    refiller: Refiller,
    alert_capacity: f64,
    consumed: u64,
}

/// El contenedor necesita conocer su capacidad maxima, el tipo de ingrediente que contiene y la 
/// capacidad porcentual a la que debe imprimir alertas.
impl Container {
    pub fn new(max: u64, cont: ContainerTypes, alert_capacity: f64) -> Self {
        let consumed: u64 = 0;
        Self {
            max_amount: max,
            total_amount: max,
            cont_type: cont,
            refiller: Refiller::new(
                cont.get_refiller_capacity(),
                cont.get_refiller(),
                alert_capacity,
            ),
            alert_capacity,
            consumed,
        }
    }

    /// Esta función consume el ingrediente y en caso de no tener suficiente, maneja si recarga o lanza error por insuficiencia.
    /// En caso de que el amount sea mayor a la capacidad del contenedor, se lanza un error.
    /// En caso de que el reporte sea true, se imprime el reporte de consumos.
    pub fn serve(&mut self, amount: u64, report: bool) -> Result<(), String> {
        if amount > self.max_amount {
            return Err(format!(
                "La cantidad de {:#?} es mayor a la capacidad del contenedor.",
                self.cont_type
            ));
        }
        if amount > self.total_amount {
            let to_recharge = self.refiller.recharge(self.max_amount);
            if to_recharge == 0 {
                return Err(format!("Nos quedamos sin {:#?}.", self.cont_type));
            } else {
                println!(
                    "INFO: El contenedor de {:#?} ha sido recargado.",
                    self.cont_type
                );
                self.total_amount += to_recharge;
            }
        }
        self.total_amount -= amount;
        self.consumed += amount;
        thread::sleep(Duration::from_millis(amount*100));
        let percentage: f64 = ((self.total_amount) as f64 / (self.max_amount) as f64) * 100.0;
        if percentage < self.alert_capacity && !report {
            println!(
                "INFO: El contenedor de {:#?} tiene {}% de contenido.",
                self.cont_type, percentage
            );
        }

        Ok(())
    }
    /// Esta función imprime el reporte de consumos del contendedor.
    pub fn periodic_reports(&self) {
        let percentage: f64 = ((self.total_amount) as f64 / (self.max_amount) as f64) * 100.0;
        println!(
            "INFO: El contenedor de {:#?} tiene {}% de contenido.",
            self.cont_type, percentage
        );
        println!(
            "INFO: Hasta el momento se han utilizado {} de {:#?}",
            self.consumed, self.cont_type
        );
        self.refiller.report();
    }
}
