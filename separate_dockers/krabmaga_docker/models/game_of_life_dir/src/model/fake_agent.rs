use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct FakeAgent {
    pub id: u32,
}

impl Hash for FakeAgent {
    fn hash<H>(&self, state: &mut H)
        where
            H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for FakeAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for FakeAgent {}

impl PartialEq for FakeAgent {
    fn eq(&self, other: &FakeAgent) -> bool {
        self.id == other.id
    }
}

impl Agent for FakeAgent {
    fn step(&mut self, state: &mut dyn State) {

    }
}