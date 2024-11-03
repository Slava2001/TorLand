use botc::code::{Command, Dir};
use super::WordAccessor;

pub(super) enum Action {
    Ok,
    Die,
    Mov(Dir)
}


#[derive(Debug)]
struct State {}

#[derive(Debug)]
pub struct Bot {
    state: State,
    genom: Vec<Command>,
}

impl Bot {
    pub(super) fn new() -> Self {
        Self {
            state: State {},
            genom: Vec::new(),
        }
    }

    pub(super) fn update(&mut self, _wa: &mut WordAccessor) -> Action {
        Action::Mov(Dir::Front)
    }
}
