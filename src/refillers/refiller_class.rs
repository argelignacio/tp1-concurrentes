use crate::refillers::enum_refillers::RefillerTypes;

/// Este struct representa un recargador de un ingrediente, el cual tiene una capacidad máxima y un tipo de ingrediente.
pub struct Refiller {
    max: u64,
    amount: u64,
    refiller_type: RefillerTypes,
    consumed: u64,
    alert_capacity: f64,
}

/// El recargador necesita conocer su capacidad maxima, el tipo de ingrediente que contiene y la capacidad porcentual a la que debe imprimir alertas.
impl Refiller {
    pub fn new(amount: u64, refiller: RefillerTypes, alert_capacity: f64) -> Self {
        let consumed: u64 = 0;
        Self {
            max: amount,
            amount,
            refiller_type: refiller,
            consumed,
            alert_capacity,
        }
    }

    /// Esta función consume el ingrediente y en caso de no tener suficiente, maneja si recarga o lanza error por insuficiencia.
    pub fn recharge(&mut self, amount: u64) -> u64 {
        if amount == 0 {
            println!(
                "ERR: No se puede recargar, recargador de {:#?} sin contenido.",
                self.refiller_type
            );
            return 0;
        }
        if self.amount > amount {
            self.amount -= amount;
            let percentage: f64 = ((self.amount) as f64 / (self.max) as f64) * 100.0;
            if percentage < self.alert_capacity {
                println!(
                    "INFO: El recargador de {:#?} tiene menos de la mitad de su contenido.",
                    self.refiller_type
                );
            }
            amount
        } else {
            let old_amount: u64 = self.amount;
            self.amount = 0;
            old_amount
        }
    }

    /// Esta función imprime el reporte de consumos.
    pub fn report(&self) {
        match &self.refiller_type {
            RefillerTypes::Agua => {}
            RefillerTypes::Cacao => {}
            default => println!(
                "INFO: Hasta el momento se han utilizado {} de {:#?}",
                self.consumed, default
            ),
        };
    }
}
