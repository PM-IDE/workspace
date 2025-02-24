use crate::features::discovery::petri_net::ids::next_id;
use crate::utils::user_data::user_data::UserDataImpl;

const EMPTY_PLACE_NAME: &'static str = "EmptyPlace";

#[derive(Debug)]
pub struct Place {
  id: u64,
  name: String,
  user_data: UserDataImpl,
}

impl Place {
  pub fn empty() -> Self {
    Self {
      id: next_id(),
      name: EMPTY_PLACE_NAME.to_owned(),
      user_data: UserDataImpl::new(),
    }
  }

  pub fn with_name(name: String) -> Self {
    Self {
      id: next_id(),
      name,
      user_data: UserDataImpl::new(),
    }
  }

  pub fn id(&self) -> u64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn user_data(&self) -> &UserDataImpl {
    &self.user_data
  }

  pub fn user_data_mut(&mut self) -> &mut UserDataImpl {
    &mut self.user_data
  }
}
