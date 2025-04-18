use crate::features::discovery::root_sequence::models::RootSequenceKind;
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::UserDataImpl;

type NameExtractor<'a, T> = &'a dyn Fn(&T) -> HeapedOrOwned<String>;
type ArtificialStartEnd<'a, T> = &'a dyn Fn() -> (T, T);
type NodeDataTransfer<'a, T> = &'a dyn Fn(&T, &mut UserDataImpl, bool) -> ();

pub struct DiscoveryContext<'a, T> {
  name_extractor: NameExtractor<'a, T>,
  artificial_start_end_events_factory: ArtificialStartEnd<'a, T>,
  root_sequence_kind: RootSequenceKind,
  event_to_graph_node_info_transfer: NodeDataTransfer<'a, T>,
}

impl<'a, T> DiscoveryContext<'a, T> {
  pub fn new(
    name_extractor: NameExtractor<'a, T>,
    artificial_start_end_events_factory: ArtificialStartEnd<'a, T>,
    root_sequence_kind: RootSequenceKind,
    event_to_graph_node_info_transfer: NodeDataTransfer<'a, T>,
  ) -> Self {
    Self {
      name_extractor,
      artificial_start_end_events_factory,
      root_sequence_kind,
      event_to_graph_node_info_transfer,
    }
  }

  pub fn name_extractor(&self) -> NameExtractor<'a, T> { self.name_extractor }
  pub fn artificial_start_end_events_factory(&self) -> ArtificialStartEnd<'a, T> { self.artificial_start_end_events_factory }
  pub fn root_sequence_kind(&self) -> RootSequenceKind { self.root_sequence_kind }
  pub fn event_to_graph_node_info_transfer(&self) -> NodeDataTransfer<'a, T> { self.event_to_graph_node_info_transfer }
}