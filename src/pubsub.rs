use std::marker::PhantomData;

use thiserror::Error;

use crate::{UseInPerun, multiple, single};

pib

pub struct PubSubProcess<'topics, Event, TX, RX, Spawn>
where
    Spawn: Fn(usize) -> (TX, RX),
{
    _event: PhantomData<Event>,
    spawn_fn: Spawn,
    topics: radix_trie::Trie<&'topics str, Vec<RX>>,
}

#[derive(Debug, Error, PartialEq)]
pub enum PubSubError<'a> {
    #[error("TopicUndefined: {0}")]
    TopicUndefined(&'a str),
}

impl<'topics, Event, TX, RX, Spawn> PubSubProcess<'topics, Event, TX, RX, Spawn>
where
    TX: multiple::Sender<Event>,
    RX: single::Receiver<Event>,
    Spawn: Fn(usize) -> (TX, RX),
{
    pub fn new(spawn_fn: Spawn, topics: &[&'topics str]) -> Self {
        Self {
            _event: PhantomData::default(),
            spawn_fn,
            topics: topics.iter().map(|topic| (*topic, Vec::new())).collect(),
        }
    }
    
    pub fn 
}
