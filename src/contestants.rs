use rand::Rng;
use std::collections::HashMap;

use crate::strategy::{self, Strategy};

pub struct TitForTat {
    black_list: HashMap<usize, strategy::Action>,
}

impl TitForTat {
    pub fn new() -> Self {
        Self {
            black_list: HashMap::new(),
        }
    }
}

impl Strategy for TitForTat {
    fn act(&mut self, agent_id: usize) -> strategy::Action {
        *self.black_list.get(&agent_id).unwrap_or(&strategy::Action::StaysSilent)
    }

    fn feedback(&mut self, agent_id: usize, action: strategy::Action) {
        self.black_list.insert(agent_id, action);
    }
    
    fn identify(&self) -> String {
        "TitForTat".to_string()
    }
}

pub struct AlwaysDefect;

impl AlwaysDefect {
    pub fn new() -> Self {
        Self
    }
}

impl Strategy for AlwaysDefect {
    fn act(&mut self, _agent_id: usize) -> strategy::Action {
        strategy::Action::Testifies
    }

    fn feedback(&mut self, _agent_id: usize, _action: strategy::Action) {}
    
    fn identify(&self) -> String {
        "Always Defect".to_string()
    }
}

pub struct AlwaysCooperate;

impl AlwaysCooperate {
    pub fn new() -> Self {
        Self
    }
}

impl strategy::Strategy for AlwaysCooperate {
    fn act(&mut self, _agent_id: usize) -> strategy::Action {
        strategy::Action::StaysSilent
    }

    fn feedback(&mut self, _agent_id: usize, _action: strategy::Action) {}

    fn identify(&self) -> String {
        "Always Cooperate".to_string()
    }
}

pub struct OnlyBurntOnce {
    black_list: HashMap<usize, bool>,
}

impl OnlyBurntOnce {
    pub fn new() -> Self {
        Self {
            black_list: HashMap::new(),
        }
    }
}

impl strategy::Strategy for OnlyBurntOnce {
    fn act(&mut self, agent_id: usize) -> strategy::Action {
        if *self.black_list.get(&agent_id).unwrap_or(&false) {
            strategy::Action::Testifies
        } else {
            strategy::Action::StaysSilent
        }
    }

    fn feedback(&mut self, agent_id: usize, action: strategy::Action) {
        if action == strategy::Action::Testifies {
            self.black_list.insert(agent_id, true);
        }
    }
    
    fn identify(&self) -> String {
        "Only Burnt Once".to_string()
    }
}

pub struct GrimTrigger {
    defected: bool,
}

impl GrimTrigger {
    pub fn new() -> Self {
        Self { defected: false }
    }
}

impl strategy::Strategy for GrimTrigger {
    fn act(&mut self, _agent_id: usize) -> strategy::Action {
        if self.defected {
            strategy::Action::Testifies
        } else {
            strategy::Action::StaysSilent
        }
    }

    fn feedback(&mut self, _agent_id: usize, action: strategy::Action) {
        if action == strategy::Action::Testifies {
            self.defected = true;
        }
    }
    
    fn identify(&self) -> String {
        "Grim Trigger".to_string()
    }
}

pub struct Random;

impl Random {
    pub fn new() -> Self {
        Self
    }
}

impl strategy::Strategy for Random {
    fn act(&mut self, _agent_id: usize) -> strategy::Action {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {
            strategy::Action::StaysSilent
        } else {
            strategy::Action::Testifies
        }
    }

    fn feedback(&mut self, _agent_id: usize, _action: strategy::Action) {}
    
    fn identify(&self) -> String {
        "Random".to_string()
    }
}

pub struct TitForTwoTats {
    defect_count: HashMap<usize, usize>,
}

impl TitForTwoTats {
    pub fn new() -> Self {
        Self {
            defect_count: HashMap::new(),
        }
    }
}

impl Strategy for TitForTwoTats {
    fn act(&mut self, agent_id: usize) -> strategy::Action {
        if *self.defect_count.get(&agent_id).unwrap_or(&0) >= 2 {
            strategy::Action::Testifies
        } else {
            strategy::Action::StaysSilent
        }
    }

    fn feedback(&mut self, agent_id: usize, action: strategy::Action) {
        if action == strategy::Action::Testifies {
            *self.defect_count.entry(agent_id).or_insert(0) += 1;
        }
    }
    
    fn identify(&self) -> String {
        "Tit For Two Tats".to_string()
    }
}

pub struct TwoTitsForTat {
    last_action: HashMap<usize, strategy::Action>,
}

impl TwoTitsForTat {
    pub fn new() -> Self {
        Self {
            last_action: HashMap::new(),
        }
    }
}

impl Strategy for TwoTitsForTat {
    fn act(&mut self, agent_id: usize) -> strategy::Action {
        if let Some(&strategy::Action::Testifies) = self.last_action.get(&agent_id) {
            strategy::Action::Testifies
        } else {
            strategy::Action::StaysSilent
        }
    }

    fn feedback(&mut self, agent_id: usize, action: strategy::Action) {
        self.last_action.insert(agent_id, action);
    }
    
    fn identify(&self) -> String {
        "Two Tits For Tat".to_string()
    }
}

pub struct Pavlov {
    last_successful: bool,
}

impl Pavlov {
    pub fn new() -> Self {
        Self { last_successful: true }
    }
}

impl Strategy for Pavlov {
    fn act(&mut self, _agent_id: usize) -> strategy::Action {
        if self.last_successful {
            strategy::Action::StaysSilent
        } else {
            strategy::Action::Testifies
        }
    }

    fn feedback(&mut self, _agent_id: usize, action: strategy::Action) {
        self.last_successful = action == strategy::Action::StaysSilent;
    }
    
    fn identify(&self) -> String {
        "Pavlov".to_string()
    }
}


