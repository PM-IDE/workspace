use crate::utils::graph::graph::NEXT_ID;
use std::sync::atomic::Ordering;
use crate::utils::user_data::user_data::UserDataImpl;

#[derive(Debug)]
pub struct GraphEdge<TEdgeData>
where
  TEdgeData: ToString,
{
  pub(crate) id: u64,
  pub(crate) first_node_id: u64,
  pub(crate) second_node_id: u64,
  pub(crate) data: Option<TEdgeData>,
  pub(crate) weight: f64,
  pub(crate) user_data: UserDataImpl
}

impl<TEdgeData> GraphEdge<TEdgeData>
where
  TEdgeData: ToString,
{
  pub fn new(first_node_id: u64, second_node_id: u64, weight: f64, data: Option<TEdgeData>) -> Self {
    Self {
      first_node_id,
      second_node_id,
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      data,
      weight,
      user_data: UserDataImpl::new()
    }
  }

  pub fn data(&self) -> Option<&TEdgeData> {
    self.data.as_ref()
  }

  pub fn id(&self) -> &u64 {
    &self.id
  }

  pub fn from_node(&self) -> &u64 {
    &self.first_node_id
  }

  pub fn to_node(&self) -> &u64 {
    &self.second_node_id
  }

  pub fn weight(&self) -> f64 {
    self.weight
  }
  
  pub fn user_data(&self) -> &UserDataImpl {
    &self.user_data
  }

  pub fn user_data_mut(&mut self) -> &mut UserDataImpl {
    &mut self.user_data
  }
}
