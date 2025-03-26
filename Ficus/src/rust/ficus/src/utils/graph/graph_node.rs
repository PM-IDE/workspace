use crate::utils::graph::graph::{Graph, NEXT_ID};
use crate::utils::user_data::user_data::UserDataImpl;
use std::sync::atomic::Ordering;

pub struct GraphNode<TNodeData>
where
  TNodeData: ToString,
{
  pub(crate) id: u64,
  pub(crate) data: Option<TNodeData>,
  pub(crate) user_data: UserDataImpl,
}

impl<TNodeData> GraphNode<TNodeData>
where
  TNodeData: ToString,
{
  pub fn new(data: Option<TNodeData>) -> Self {
    Self {
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      data,
      user_data: UserDataImpl::new(),
    }
  }

  pub fn new_with_user_data(data: Option<TNodeData>, user_data: UserDataImpl) -> Self {
    Self {
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      data,
      user_data,
    }
  }

  pub fn data(&self) -> Option<&TNodeData> {
    self.data.as_ref()
  }

  pub fn id(&self) -> &u64 {
    &self.id
  }

  pub fn user_data_mut(&mut self) -> &mut UserDataImpl {
    &mut self.user_data
  }

  pub fn user_data(&self) -> &UserDataImpl {
    &self.user_data
  }
}
