use crate::models::domain::bxes_value::BxesValue;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BxesDrivers {
    pub drivers: Vec<BxesDriver>,
}

impl Hash for BxesDrivers {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for driver in &self.drivers {
            driver.hash(state);
        }
    }
}

impl PartialEq for BxesDrivers {
    fn eq(&self, other: &Self) -> bool {
        if self.drivers.len() != other.drivers.len() {
            return false;
        }

        for (self_driver, other_driver) in self.drivers.iter().zip(&other.drivers) {
            if !self_driver.eq(other_driver) {
                return false;
            }
        }

        return true;
    }
}

#[derive(Clone, Debug)]
pub struct BxesDriver {
    pub amount: BxesValue,
    pub name: Rc<Box<BxesValue>>,
    pub driver_type: Rc<Box<BxesValue>>,
}

impl BxesDriver {
    pub fn amount(&self) -> f64 {
        if let BxesValue::Float64(amount) = self.amount {
            return amount;
        }

        panic!("Expected f64 BxesValue, got {:?}", self.amount)
    }
}

impl Hash for BxesDriver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.amount.hash(state);
        self.name.hash(state);
        self.driver_type.hash(state);
    }
}

impl PartialEq for BxesDriver {
    fn eq(&self, other: &Self) -> bool {
        self.amount == other.amount
            && self.name == other.name
            && self.driver_type == other.driver_type
    }
}
