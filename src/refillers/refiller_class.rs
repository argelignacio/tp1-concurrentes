use std::default;

use crate::refillers::enum_refillers::RefillerTypes;

pub struct Refiller {
    amount: u64,
    refiller_type: RefillerTypes,
    consumed: u64,
}

impl Refiller {
    pub fn new(amount: u64, refiller: RefillerTypes) -> Self {
        let consumed: u64 = 0;
        Self {
            amount,
            refiller_type: refiller,
            consumed
        }
    }

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
            amount
        } else {
            let old_amount: u64 = self.amount;
            self.amount = 0;
            old_amount
        }
    }

    pub fn report(&self){
        match &self.refiller_type {
            RefillerTypes::Agua => {},
            RefillerTypes::Cacao => {},
            default =>println!("Hasta el momento se han utilizado {} de {:#?}", self.consumed, default),
        };
        
    }
}
