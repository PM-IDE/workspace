use crate::{
  features::discovery::petri_net::{arc::PetriNetArc, petri_net::PetriNet, place::Place, transition::Transition},
  utils::xml_utils::{StartEndElementCookie, XmlWriteError},
};
use quick_xml::{
  Writer,
  events::{BytesText, Event},
};
use std::{cell::RefCell, fs, io::Cursor};

const PNML_TAG_NAME: &str = "pnml";
const TRANSITION_TAG_NAME: &str = "transition";
const ARC_TAG_NAME: &str = "arc";
const PLACE_TAG_NAME: &str = "place";
const NET_TAG_NAME: &str = "net";
const TEXT_TAG_NAME: &str = "text";
const NAME_TAG_NAME: &str = "name";

const TOOL_SPECIFIC_TAG_NAME: &str = "toolspecific";
const TOOL_ATTR_NAME: &str = "tool";
const PROM_VALUE: &str = "ProM";
const VERSION_ATTR_NAME: &str = "version";
const VERSION_VALUE: &str = "6.4";
const ACTIVITY_ATTR_NAME: &str = "activity";
const SILENT_ACTIVITY: &str = "$invisible$";

const INITIAL_MARKING_TAG: &str = "initialMarking";
const FINAL_MARKINGS_TAG: &str = "finalmarkings";
const MARKING_TAG: &str = "marking";
const ID_REF_ATTR: &str = "idref";

const ID_ATTR_NAME: &str = "id";
const SOURCE_ATTR_NAME: &str = "source";
const TARGET_ATTR_NAME: &str = "target";

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
  write_final_markings(net, &writer)?;

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
    let cookie = StartEndElementCookie::new_with_attrs(
      writer,
      PLACE_TAG_NAME,
      &vec![(ID_ATTR_NAME, create_place_id(place, use_names_as_ids).as_str())],
    )?;

    let marking = net.initial_marking().and_then(|m| m.active_places().iter().find(|m| m.place_id() == place.id()));
    if let Some(m) = marking {
      let i_m_cookie = StartEndElementCookie::new(writer, INITIAL_MARKING_TAG)?;
      let count_cookie = StartEndElementCookie::new(writer, TEXT_TAG_NAME)?;

      write_text(writer, &m.tokens_count().to_string())?;

      drop(count_cookie);
      drop(i_m_cookie);
    }

    drop(cookie);
  }

  Ok(())
}

fn write_final_markings<TTransitionData: ToString, TArcData>(
  net: &PetriNet<TTransitionData, TArcData>,
  writer: &RefCell<Writer<Cursor<Vec<u8>>>>,
) -> Result<(), XmlWriteError> {
  let Some(marking) = net.final_marking() else { return Ok(()); };

  let f_m_cookie = StartEndElementCookie::new(writer, FINAL_MARKINGS_TAG)?;
  for m in marking.active_places() {
    let m_cookie = StartEndElementCookie::new(writer, MARKING_TAG)?;
    let p_cookie = StartEndElementCookie::new_with_attrs(
      writer,
      PLACE_TAG_NAME,
      &vec![(ID_REF_ATTR, &m.place_id().to_string())],
    )?;

    let t_cookie = StartEndElementCookie::new(writer, TEXT_TAG_NAME);

    write_text(writer, &m.tokens_count().to_string())?;

    drop(t_cookie);
    drop(p_cookie);
    drop(m_cookie);
  }

  drop(f_m_cookie);

  Ok(())
}

fn write_text(writer: &RefCell<Writer<Cursor<Vec<u8>>>>, text: &str) -> Result<(), XmlWriteError> {
  writer
    .borrow_mut()
    .write_event(Event::Text(BytesText::new(text)))
    .map_err(|e| XmlWriteError::WriterError(quick_xml::Error::Io(std::sync::Arc::new(e))))
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
      writer,
      TRANSITION_TAG_NAME,
      &vec![(ID_ATTR_NAME, create_transition_id(transition, use_names_as_ids).as_str())],
    );

    if let Some(data) = transition.data() {
      let name = StartEndElementCookie::new(writer, NAME_TAG_NAME);
      let text = StartEndElementCookie::new(writer, TEXT_TAG_NAME);

      write_text(writer, &data.to_string())?;

      drop(text);
      drop(name);
    }

    if transition.is_silent() {
      let _ = StartEndElementCookie::new_with_attrs(
        writer,
        TOOL_SPECIFIC_TAG_NAME,
        &vec![
          (TOOL_ATTR_NAME, PROM_VALUE),
          (VERSION_ATTR_NAME, VERSION_VALUE),
          (ACTIVITY_ATTR_NAME, SILENT_ACTIVITY),
        ],
      );
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
    create_arc_name(
      create_place_id(net.place(&arc.place_id()), use_names_as_ids),
      create_transition_id(transition, use_names_as_ids),
    )
  });

  for arc in &incoming_arcs {
    StartEndElementCookie::new_with_attrs(
      writer,
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
  arcs: &[PetriNetArc<TArcData>],
  use_names_as_ids: bool,
  names_creator: impl Fn(&PetriNetArc<TArcData>) -> String,
) -> Vec<(&PetriNetArc<TArcData>, String)> {
  let mut arcs: Vec<(&PetriNetArc<TArcData>, String)> = arcs
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
    create_arc_name(
      create_transition_id(transition, use_names_as_ids),
      create_place_id(net.place(&arc.place_id()), use_names_as_ids),
    )
  });

  for arc in outgoing_arcs {
    StartEndElementCookie::new_with_attrs(
      writer,
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

fn create_arc_name(from_name: String, to_name: String) -> String {
  format!("[{{{}}}--{{{}}}]", from_name, to_name)
}
