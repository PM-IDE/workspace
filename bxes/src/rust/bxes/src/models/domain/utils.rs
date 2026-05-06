use crate::models::domain::bxes_value::BxesValue;
use std::collections::HashSet;
use std::sync::Arc;

pub fn compare_list_of_attributes(
  first_attributes: &Option<Vec<(Arc<BxesValue>, Arc<BxesValue>)>>,
  second_attributes: &Option<Vec<(Arc<BxesValue>, Arc<BxesValue>)>>,
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

    let first_set = self_attributes.iter().collect::<HashSet<&(Arc<BxesValue>, Arc<BxesValue>)>>();

    let second_set = other_attributes.iter().collect::<HashSet<&(Arc<BxesValue>, Arc<BxesValue>)>>();

    return first_set.eq(&second_set);
  }

  false
}

pub fn attributes_equals(first_attribute: &(Arc<BxesValue>, Arc<BxesValue>), second_attribute: &(Arc<BxesValue>, Arc<BxesValue>)) -> bool {
  first_attribute.0.eq(&second_attribute.0) && first_attribute.1.eq(&second_attribute.1)
}
