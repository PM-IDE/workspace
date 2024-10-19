use crate::features::discovery::petri_net::arc::Arc;
use crate::features::discovery::petri_net::petri_net::PetriNet;
use crate::features::discovery::petri_net::place::Place;
use crate::features::discovery::petri_net::transition::Transition;
use crate::utils::xml_utils::{StartEndElementCookie, XmlWriteError};
use quick_xml::events::{BytesText, Event};
use quick_xml::Writer;
use std::cell::RefCell;
use std::fs;
use std::io::Cursor;

const PNML_TAG_NAME: &'static str = "pnml";
const TRANSITION_TAG_NAME: &'static str = "transition";
const ARC_TAG_NAME: &'static str = "arc";
const PLACE_TAG_NAME: &'static str = "place";
const NET_TAG_NAME: &'static str = "net";
const TEXT_TAG_NAME: &'static str = "text";
const NAME_TAG_NAME: &'static str = "name";

const ID_ATTR_NAME: &'static str = "id";
const SOURCE_ATTR_NAME: &'static str = "source";
const TARGET_ATTR_NAME: &'static str = "target";

pub fn serialize_to_pnml_file<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    save_path: &str,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    match serialize_to_pnml(net, use_names_as_ids) {
        Ok(content) => match fs::write(save_path, content) {
            Ok(_) => Ok(()),
            Err(error) => Err(XmlWriteError::IOError(error)),
        },
        Err(error) => Err(error),
    }
}

pub fn serialize_to_pnml<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    use_names_as_ids: bool,
) -> Result<String, XmlWriteError>
where
    TTransitionData: ToString,
{
    let writer = RefCell::new(Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2));

    let pnml_cookie = StartEndElementCookie::new(&writer, PNML_TAG_NAME)?;
    let net_cookie = StartEndElementCookie::new(&writer, NET_TAG_NAME)?;

    write_places(net, &writer, use_names_as_ids)?;
    write_transitions(net, &writer, use_names_as_ids)?;
    write_arcs(net, &writer, use_names_as_ids)?;

    drop(net_cookie);
    drop(pnml_cookie);

    let content = writer.borrow().get_ref().get_ref().clone();
    match String::from_utf8(content) {
        Ok(string) => Ok(string),
        Err(error) => Err(XmlWriteError::FromUt8Error(error)),
    }
}

fn write_places<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    let mut places = net.all_places();
    places.sort_by(|left, right| left.name().cmp(right.name()));

    for place in places {
        let _ = StartEndElementCookie::new_with_attrs(
            writer,
            PLACE_TAG_NAME,
            &vec![(ID_ATTR_NAME, create_place_id(place, use_names_as_ids).as_str())],
        )?;
    }

    Ok(())
}

fn write_transitions<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    for transition in created_ordered_transitions_list(net) {
        let cookie = StartEndElementCookie::new_with_attrs(
            &writer,
            TRANSITION_TAG_NAME,
            &vec![(ID_ATTR_NAME, create_transition_id(transition, use_names_as_ids).as_str())],
        );

        if let Some(data) = transition.data() {
            let name = StartEndElementCookie::new(&writer, NAME_TAG_NAME);
            let text = StartEndElementCookie::new(&writer, TEXT_TAG_NAME);

            match writer
                .borrow_mut()
                .write_event(Event::Text(BytesText::new(data.to_string().as_str())))
            {
                Ok(()) => {}
                Err(error) => return Err(XmlWriteError::WriterError(error)),
            };

            drop(text);
            drop(name);
        }

        drop(cookie)
    }

    Ok(())
}

fn created_ordered_transitions_list<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
) -> Vec<&Transition<TTransitionData, TArcData>>
where
    TTransitionData: ToString,
{
    let mut transitions = net.all_transitions();
    transitions.sort_by(|left, right| left.name().cmp(right.name()));

    transitions
}

fn write_arcs<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    for transition in created_ordered_transitions_list(net) {
        write_incoming_arcs(net, transition, writer, use_names_as_ids)?;
        write_outgoing_arcs(net, transition, writer, use_names_as_ids)?;
    }

    Ok(())
}

fn write_incoming_arcs<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    transition: &Transition<TTransitionData, TArcData>,
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    let incoming_arcs = patch_arcs_list(transition.incoming_arcs(), use_names_as_ids, |arc| {
        create_arc_name::<TArcData>(
            create_place_id(net.place(&arc.place_id()), use_names_as_ids),
            create_transition_id(transition, use_names_as_ids),
        )
    });

    for arc in &incoming_arcs {
        StartEndElementCookie::new_with_attrs(
            &writer,
            ARC_TAG_NAME,
            &vec![
                (ID_ATTR_NAME, arc.1.as_str()),
                (
                    SOURCE_ATTR_NAME,
                    create_place_id(net.place(&arc.0.place_id()), use_names_as_ids).as_str(),
                ),
                (TARGET_ATTR_NAME, create_transition_id(transition, use_names_as_ids).as_str()),
            ],
        )?;
    }

    Ok(())
}

fn patch_arcs_list<TArcData>(
    arcs: &Vec<Arc<TArcData>>,
    use_names_as_ids: bool,
    names_creator: impl Fn(&Arc<TArcData>) -> String,
) -> Vec<(&Arc<TArcData>, String)> {
    let mut arcs: Vec<(&Arc<TArcData>, String)> = arcs
        .iter()
        .map(|arc| {
            (
                arc,
                match use_names_as_ids {
                    true => names_creator(arc),
                    false => arc.id().to_string(),
                },
            )
        })
        .collect();

    arcs.sort_by(|first, second| first.1.cmp(&second.1));
    arcs
}

fn write_outgoing_arcs<TTransitionData, TArcData>(
    net: &PetriNet<TTransitionData, TArcData>,
    transition: &Transition<TTransitionData, TArcData>,
    writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
    use_names_as_ids: bool,
) -> Result<(), XmlWriteError>
where
    TTransitionData: ToString,
{
    let outgoing_arcs = patch_arcs_list(transition.outgoing_arcs(), use_names_as_ids, |arc| {
        create_arc_name::<TArcData>(
            create_transition_id(transition, use_names_as_ids),
            create_place_id(net.place(&arc.place_id()), use_names_as_ids),
        )
    });

    for arc in outgoing_arcs {
        StartEndElementCookie::new_with_attrs(
            &writer,
            ARC_TAG_NAME,
            &vec![
                (ID_ATTR_NAME, arc.1.as_str()),
                (
                    TARGET_ATTR_NAME,
                    create_place_id(net.place(&arc.0.place_id()), use_names_as_ids).as_str(),
                ),
                (SOURCE_ATTR_NAME, create_transition_id(transition, use_names_as_ids).as_str()),
            ],
        )?;
    }

    Ok(())
}

fn create_place_id(place: &Place, use_names_as_ids: bool) -> String {
    match use_names_as_ids {
        true => place.name().to_owned(),
        false => place.id().to_string(),
    }
}

fn create_transition_id<TTransitionData, TArcData>(transition: &Transition<TTransitionData, TArcData>, use_names_as_ids: bool) -> String
where
    TTransitionData: ToString,
{
    match use_names_as_ids {
        true => transition.name().to_string(),
        false => transition.id().to_string(),
    }
}

fn create_arc_name<TArcData>(from_name: String, to_name: String) -> String {
    format!("[{{{}}}--{{{}}}]", from_name, to_name)
}
