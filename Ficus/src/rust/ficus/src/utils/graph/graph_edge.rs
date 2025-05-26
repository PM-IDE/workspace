use crate::utils::graph::graph::NEXT_ID;
use crate::utils::user_data::user_data::UserDataImpl;
use std::sync::atomic::Ordering;
use getset::{Getters, MutGetters};

#[derive(Debug, Getters, MutGetters, Clone)]
pub struct GraphEdge<TEdgeData>
where
  TEdgeData: ToString,
{
  #[getset(get = "pub")] pub(crate) id: u64,
  #[getset(get = "pub")] pub(crate) from_node: u64,
  #[getset(get = "pub")] pub(crate) to_node: u64,
  #[getset(get = "pub")] pub(crate) data: Option<TEdgeData>,
  #[getset(get = "pub")] pub(crate) weight: f64,
  #[getset(get = "pub", get_mut = "pub")] pub(crate) user_data: UserDataImpl,
}

impl<TEdgeData> GraphEdge<TEdgeData>
where
  TEdgeData: ToString,
{
  pub fn new(first_node_id: u64, second_node_id: u64, weight: f64, data: Option<TEdgeData>, user_data: Option<UserDataImpl>) -> Self {
    Self {
      from_node: first_node_id,
      to_node: second_node_id,
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      data,
      weight,
      user_data: user_data.unwrap_or(UserDataImpl::new())
    }
  }
}
