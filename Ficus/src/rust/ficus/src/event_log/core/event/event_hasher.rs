use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use fancy_regex::{Error, Regex};

use super::event::Event;

pub trait EventHasher<TEvent>
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64;
}

pub struct NameEventHasher;

impl NameEventHasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl<TEvent> EventHasher<TEvent> for NameEventHasher
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64 {
        default_class_extractor(event)
    }
}

pub fn default_class_extractor<TEvent>(event: &TEvent) -> u64
where
    TEvent: Event,
{
    default_class_extractor_name(event.name())
}

pub fn default_class_extractor_name(name: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);

    hasher.finish()
}

pub struct RegexEventHasher {
    regex: Regex,
}

impl<TEvent> EventHasher<TEvent> for RegexEventHasher
where
    TEvent: Event,
{
    fn hash(&self, event: &TEvent) -> u64 {
        self.hash_name(event.name())
    }
}

impl RegexEventHasher {
    pub fn new(regex: &String) -> Result<RegexEventHasher, Error> {
        match Regex::new(regex) {
            Ok(regex) => Ok(RegexEventHasher { regex }),
            Err(error) => Err(error),
        }
    }

    pub fn hash_name(&self, name: &str) -> u64 {
        match self.regex.find(name) {
            Ok(Some(m)) => {
                if m.start() == 0 {
                    let mut hasher = DefaultHasher::new();
                    name[0..m.end()].hash(&mut hasher);

                    hasher.finish()
                } else {
                    default_class_extractor_name(name)
                }
            }
            _ => default_class_extractor_name(name),
        }
    }

    pub fn transform<'a>(&self, name: &'a str) -> &'a str {
        match self.regex.find(name) {
            Ok(Some(m)) => {
                if m.start() == 0 {
                    &name[0..m.end()]
                } else {
                    name
                }
            }
            _ => name,
        }
    }
}
