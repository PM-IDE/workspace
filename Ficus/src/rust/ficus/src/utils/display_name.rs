use crate::event_log::core::event::event::Event;
use crate::event_log::xes::xes_event::XesEventImpl;
use crate::utils::context_key::DefaultContextKey;
use crate::utils::references::HeapedOrOwned;
use crate::utils::user_data::user_data::{UserData, UserDataOwner};
use lazy_static::lazy_static;

lazy_static!(
   pub static ref DISPLAY_NAME_KEY: DefaultContextKey<String> = DefaultContextKey::new("DISPLAY_NAME");
);

pub fn get_display_name(event: &XesEventImpl) -> HeapedOrOwned<String> {
  match event.user_data().concrete(DISPLAY_NAME_KEY.key()) {
    None => HeapedOrOwned::Heaped(event.name_pointer().clone()),
    Some(name) => HeapedOrOwned::Owned(name.clone())
  }
}