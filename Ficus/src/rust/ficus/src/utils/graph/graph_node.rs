use crate::utils::graph::graph::NEXT_ID;
use std::sync::atomic::Ordering;

pub struct GraphNode<TNodeData>
where
  TNodeData: ToString,
{
  pub(crate) id: u64,
  pub(crate) data: Option<TNodeData>,
}

impl<TNodeData> GraphNode<TNodeData>
where
  TNodeData: ToString,
{
  pub fn new(data: Option<TNodeData>) -> Self {
    Self {
      id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
      data,
    }
  }

  pub fn data(&self) -> Option<&TNodeData> {
    self.data.as_ref()
  }

  pub fn id(&self) -> &u64 {
    &self.id
  }
}
