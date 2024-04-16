use fancy_regex::Regex;

use crate::pipelines::pipeline_parts::PipelineParts;
use crate::{
    event_log::{
        core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
        xes::xes_event::XesEventImpl,
    },
    features::analysis::patterns::activity_instances::{SubTraceKind, UNDEF_ACTIVITY_NAME},
    utils::{
        colors::{Color, ColoredRectangle},
        user_data::user_data::UserData,
    },
};

use super::{
    context::PipelineContext, errors::pipeline_errors::PipelinePartExecutionError, keys::context_keys::ContextKeys,
    pipelines::PipelinePartFactory,
};

impl PipelineParts {
    pub(super) fn traces_diversity_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::TRACES_DIVERSITY_DIAGRAM, &|context, _, keys, _| {
            let log = Self::get_user_data(context, keys.event_log())?;
            let colors_holder = context.concrete_mut(keys.colors_holder().key()).expect("Should be initialized");

            let mut result = vec![];
            for trace in log.traces() {
                let mut vec = vec![];
                let mut index = 0usize;
                for event in trace.borrow().events() {
                    let event = event.borrow();
                    let name = event.name();
                    let color = colors_holder.get_or_create(name.as_str());

                    vec.push(ColoredRectangle::square(color, index, name.to_owned()));
                    index += 1;
                }

                result.push(vec);
            }

            context.put_concrete(keys.colors_event_log().key(), result);

            Ok(())
        })
    }

    pub(super) fn draw_placements_of_event_by_name() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_PLACEMENT_OF_EVENT_BY_NAME, &|context, _, keys, config| {
            let event_name = Self::get_user_data(config, keys.event_name())?;
            Self::draw_events_placement(context, keys, &|event| event.name() == event_name)
        })
    }

    pub(super) fn draw_events_placement(
        context: &mut PipelineContext,
        keys: &ContextKeys,
        selector: &impl Fn(&XesEventImpl) -> bool,
    ) -> Result<(), PipelinePartExecutionError> {
        let log = Self::get_user_data(context, keys.event_log())?;
        let colors_holder = Self::get_user_data_mut(context, keys.colors_holder()).expect("Default value should be initialized");

        let mut colors_log = vec![];
        for trace in log.traces() {
            let mut colors_trace = vec![];
            let mut index = 0usize;
            for event in trace.borrow().events() {
                let event = event.borrow();
                let name = event.name();
                if selector(&event) {
                    let color = colors_holder.get_or_create(name.as_str());
                    colors_trace.push(ColoredRectangle::square(color, index, name.to_owned()));
                } else {
                    colors_trace.push(ColoredRectangle::square(Color::black(), index, UNDEF_ACTIVITY_NAME.to_owned()));
                }

                index += 1;
            }

            colors_log.push(colors_trace);
        }

        context.put_concrete(keys.colors_event_log().key(), colors_log);
        Ok(())
    }

    pub(super) fn draw_events_placements_by_regex() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_PLACEMENT_OF_EVENT_BY_REGEX, &|context, _, keys, config| {
            let regex = Self::get_user_data(config, keys.regex())?;
            let regex = Regex::new(regex).ok().unwrap();
            Self::draw_events_placement(context, keys, &|event| regex.is_match(event.name()).ok().unwrap())
        })
    }

    pub(super) fn draw_full_activities_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_FULL_ACTIVITIES_DIAGRAM, &|context, _, keys, _| {
            let traces_activities = Self::get_user_data(context, keys.trace_activities())?;
            let log = Self::get_user_data(context, keys.event_log())?;
            let colors_holder = Self::get_user_data_mut(context, keys.colors_holder())?;

            let mut colors_log = vec![];
            for (activities, trace) in traces_activities.into_iter().zip(log.traces().into_iter()) {
                let mut colors_trace = vec![];

                Self::execute_with_activities_instances(activities, trace.borrow().events().len(), &mut |sub_trace| match sub_trace {
                    SubTraceKind::Attached(activity) => {
                        let color = colors_holder.get_or_create(&activity.node.borrow().name);
                        let name = activity.node.borrow().name.to_owned();
                        colors_trace.push(ColoredRectangle::new(color, activity.start_pos, activity.length, name));
                    }
                    SubTraceKind::Unattached(start_pos, length) => {
                        colors_trace.push(ColoredRectangle::new(
                            Color::black(),
                            start_pos,
                            length,
                            UNDEF_ACTIVITY_NAME.to_string(),
                        ));
                    }
                })?;

                colors_log.push(colors_trace);
            }

            context.put_concrete(keys.colors_event_log().key(), colors_log);

            Ok(())
        })
    }

    pub(super) fn draw_short_activities_diagram() -> (String, PipelinePartFactory) {
        Self::create_pipeline_part(Self::DRAW_SHORT_ACTIVITIES_DIAGRAM, &|context, _, keys, _| {
            let traces_activities = Self::get_user_data(context, keys.trace_activities())?;
            let log = Self::get_user_data(context, keys.event_log())?;
            let colors_holder = Self::get_user_data_mut(context, keys.colors_holder())?;

            let mut colors_log = vec![];
            for (activities, trace) in traces_activities.into_iter().zip(log.traces().into_iter()) {
                let mut colors_trace = vec![];
                let mut index = 0;
                Self::execute_with_activities_instances(activities, trace.borrow().events().len(), &mut |sub_trace| {
                    match sub_trace {
                        SubTraceKind::Attached(activity) => {
                            let color = colors_holder.get_or_create(&activity.node.borrow().name);
                            let name = activity.node.borrow().name.to_owned();
                            colors_trace.push(ColoredRectangle::new(color, index, 1, name));
                        }
                        SubTraceKind::Unattached(_, _) => {
                            colors_trace.push(ColoredRectangle::new(Color::black(), index, 1, UNDEF_ACTIVITY_NAME.to_owned()));
                        }
                    }

                    index += 1;
                })?;

                colors_log.push(colors_trace);
            }

            context.put_concrete(keys.colors_event_log().key(), colors_log);

            Ok(())
        })
    }
}
