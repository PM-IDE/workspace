use crate::event_log::core::event::event::Event;
use crate::event_log::core::event_log::EventLog;
use crate::event_log::core::trace::trace::Trace;
use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::cases::cases_discovery_state::CasesDiscoveryState;
use fancy_regex::Regex;

pub fn discover_cases(log: &XesEventLogImpl, start_regex_str: &str, end_regex_str: &str, inline_nested: bool) -> XesEventLogImpl {
    let start_regex = Regex::new(start_regex_str).expect("Must create regex");
    let end_regex = Regex::new(end_regex_str).expect("Must create regex");

    let mut state = CasesDiscoveryState::new(inline_nested);

    for trace in log.traces() {
        let trace = trace.borrow();

        for event in trace.events() {
            let event = event.borrow();
            let event_name = event.name().as_str();
            if start_regex.is_match(event_name).expect("Regex") {
                state.handle_start_event(&event);
                continue;
            }

            if end_regex.is_match(event_name).expect("Regex") {
                state.handle_end_event(&event);
                continue;
            }

            state.handle_default_event(&event);
        }

        state.handle_trace_end();
    }

    state.log()
}
