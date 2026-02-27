use crate::models::domain::bxes_value::BxesValue;
use std::{collections::HashSet, rc::Rc};

pub fn compare_list_of_attributes(
  first_attributes: &Option<Vec<(Rc<BxesValue>, Rc<BxesValue>)>>,
  second_attributes: &Option<Vec<(Rc<BxesValue>, Rc<BxesValue>)>>,
) -> bool {
  if first_attributes.is_none() && second_attributes.is_none() {
    return true;
  }

  if let Some(self_attributes) = first_attributes.as_ref()
    && let Some(other_attributes) = second_attributes.as_ref()
  {
    if self_attributes.len() != other_attributes.len() {
      return false;
    }

    let first_set = self_attributes.iter().collect::<HashSet<&(Rc<BxesValue>, Rc<BxesValue>)>>();

    let second_set = other_attributes.iter().collect::<HashSet<&(Rc<BxesValue>, Rc<BxesValue>)>>();

    return first_set.eq(&second_set);
  }

  false
}

pub fn attributes_equals(first_attribute: &(Rc<BxesValue>, Rc<BxesValue>), second_attribute: &(Rc<BxesValue>, Rc<BxesValue>)) -> bool {
  first_attribute.0.eq(&second_attribute.0) && first_attribute.1.eq(&second_attribute.1)
}
