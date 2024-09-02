use crate::event_log::xes::xes_event_log::XesEventLogImpl;

unsafe impl Sync for XesEventLogImpl {}
unsafe impl Send for XesEventLogImpl {}
