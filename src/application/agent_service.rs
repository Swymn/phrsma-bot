use rand::seq::IndexedRandom;

use crate::domain::agent::Agent;

pub fn pick_random_agent(agents: &[Agent]) -> Option<Agent> {
    let mut rng = rand::rng();
    agents.choose(&mut rng).cloned()
}

#[cfg(test)]
mod tests {
    use crate::{application::agent_service::pick_random_agent, domain::agent::Agent};

    #[test]
    fn should_pick_random_agent() {
        // GIVEN an array of agent
        let agents = vec![
            Agent::new("1".to_string(), "Jett".to_string()),
            Agent::new("2".to_string(), "Reyna".to_string()),
        ];

        // WHEN pick a random agent from that array
        let agent = pick_random_agent(&agents);

        // THEN it should return an agent
        assert!(agent.is_some());
    }
}
