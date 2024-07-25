use contestants::AlwaysCooperate;
use contestants::AlwaysDefect;
use contestants::TitForTat;

pub mod strategy;
pub mod contestants;

use strategy::Strategy;
use strategy::Action;

struct Agent {
    id: usize,
    score: i32,
    strategy: Box<dyn Strategy>,
}

impl Agent {
    fn new(id: usize, strategy: Box<dyn Strategy>) -> Self {
        Self {
            id,
            score: 0,
            strategy,
        }
    }

    fn act(&mut self, agent_id: usize) -> Action {
        self.strategy.act(agent_id)
    }

    fn feedback(&mut self, agent_id: usize, action: Action) {
        self.strategy.feedback(agent_id, action)
    }

    fn strategy_name(&self) -> String {
        self.strategy.identify()
    }
}

struct World {
    agents: Vec<Agent>,
}

impl World {
    fn new(agents: Vec<Agent>) -> Self {
        Self { agents }
    }

    fn update_scores(&mut self, actions: Vec<(usize, usize, Action, Action)>) {
        for (id1, id2, action1, action2) in actions {
            println!("{}({}) {:?} and {}({}) {:?}", id1, self.agents[id1].strategy_name(), action1 , id2, self.agents[id2].strategy_name(), action2 );
            match (action1, action2) {
                (Action::StaysSilent, Action::StaysSilent) => {
                    self.agents[id1].score += 1;
                    self.agents[id2].score += 1;
                }
                (Action::StaysSilent, Action::Testifies) => {
                    self.agents[id1].score += 3;
                }
                (Action::Testifies, Action::StaysSilent) => {
                    self.agents[id2].score += 3;
                }
                (Action::Testifies, Action::Testifies) => {
                    self.agents[id1].score += 2;
                    self.agents[id2].score += 2;
                }
            }
            self.agents[id1].feedback(id2, action2);
            self.agents[id2].feedback(id1, action1);
        }
    }

    fn world_loop(&mut self) {
        let mut actions = Vec::new();
        for i in 0..self.agents.len() {
            for j in (i+1) ..self.agents.len() {
                if i != j {

                    let action1 = self.agents[i].act(j);
                    let action2 = self.agents[j].act(i);
                    actions.push((i, j, action1, action2));
                }
            }
        }
        self.update_scores(actions);
        self.dump_state();
    }

    fn dump_state(&self) {
        for agent in &self.agents {
            println!("Agent {}: using Strategy: {} Scored: {}", agent.id, agent.strategy_name(), agent.score);
        }
    }
}

fn main() {
    let mut world = World::new(vec![
        Agent::new(0, Box::new(contestants::TitForTat::new())),
        Agent::new(1, Box::new(contestants::AlwaysDefect::new())),
        Agent::new(2, Box::new(contestants::AlwaysCooperate::new())),
        Agent::new(3, Box::new(contestants::OnlyBurntOnce::new())),
        Agent::new(4, Box::new(contestants::GrimTrigger::new())),
        Agent::new(5, Box::new(contestants::Random::new())),
        Agent::new(6, Box::new(contestants::TitForTwoTats::new())),
        Agent::new(7, Box::new(contestants::TwoTitsForTat::new())),
        Agent::new(8, Box::new(contestants::Pavlov::new())),
    ]);
    for epoch in 0..200 { 
        println!("\n Epoch {}", epoch);
        world.world_loop();
    }
}

