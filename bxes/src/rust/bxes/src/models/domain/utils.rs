use crate::models::domain::bxes_value::BxesValue;
use std::rc::Rc;

pub fn compare_list_of_attributes(
    first_attributes: &Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
    second_attributes: &Option<Vec<(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>)>>,
) -> bool {
    if first_attributes.is_none() && second_attributes.is_none() {
        return true;
    }

    if let Some(self_attributes) = first_attributes.as_ref() {
        if let Some(other_attributes) = second_attributes.as_ref() {
            if self_attributes.len() != other_attributes.len() {
                return false;
            }

            for (self_attribute, other_attribute) in self_attributes.iter().zip(other_attributes) {
                if !(attributes_equals(self_attribute, other_attribute)) {
                    return false;
                }
            }

            return true;
        }
    }

    return false;
}

pub fn attributes_equals(
    first_attribute: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
    second_attribute: &(Rc<Box<BxesValue>>, Rc<Box<BxesValue>>),
) -> bool {
    first_attribute.0.eq(&second_attribute.0) && first_attribute.1.eq(&second_attribute.1)
}