
use crate::refillers::enum_refillers::RefillerTypes;

pub struct Refiller {
    amount: u64,
    refiller_type: RefillerTypes,
}

impl Refiller {
    pub fn new(amount: u64, refiller: RefillerTypes) -> Self {
        Self {
            amount: amount,
            refiller_type: refiller,
        }
    }

    pub fn recharge(&mut self, amount: u64) -> u64{
        if amount==0{
            println!("No se puede recargar, recargador de {:#?} sin contenido.", self.refiller_type);
            return 0
        }
        if self.amount > amount{
            self.amount -= amount;
            return amount
        }else{
            let old_amount = self.amount;
            self.amount = 0;
            return old_amount
        }
    }
}