#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    StaysSilent,
    Testifies,
}

pub trait Strategy {
    fn act(&mut self, agent_id: usize) -> Action;
    fn feedback(&mut self, agent_id: usize, action: Action);
    fn identify(&self) -> String;
}


