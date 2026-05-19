use crate::{
  context_key,
  event_log::{core::event::event::Event, xes::xes_event::XesEventImpl},
  utils::user_data::user_data::{UserData, UserDataOwner},
};
use lazy_static::lazy_static;
use std::sync::Arc;

const DISPLAY_NAME: &str = "DISPLAY_NAME";
context_key! { DISPLAY_NAME, Arc<str> }

pub fn get_display_name(event: &XesEventImpl) -> Arc<str> {
  match event.user_data().concrete(DISPLAY_NAME_KEY.key()) {
    None => event.name_pointer().clone(),
    Some(name) => name.clone(),
  }
}
