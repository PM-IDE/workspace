use crate::{
  context_key,
  event_log::{core::event::event::Event, xes::xes_event::XesEventImpl},
  utils::{
    references::HeapedOrOwned,
    user_data::user_data::{UserData, UserDataOwner},
  },
};
use lazy_static::lazy_static;

const DISPLAY_NAME: &str = "DISPLAY_NAME";
context_key! { DISPLAY_NAME, String }

pub fn get_display_name(event: &XesEventImpl) -> HeapedOrOwned<String> {
  match event.user_data().concrete(DISPLAY_NAME_KEY.key()) {
    None => HeapedOrOwned::Heaped(event.name_pointer().clone()),
    Some(name) => HeapedOrOwned::Owned(name.clone()),
  }
}
