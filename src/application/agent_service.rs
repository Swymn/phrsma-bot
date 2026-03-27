use rand::seq::IteratorRandom;

use crate::domain::{agent::Agent, role::RoleName};

pub fn pick_random_agent(agents: &[Agent], role: Option<RoleName>) -> Option<Agent> {
    let mut rng = rand::rng();
    agents
        .iter()
        .filter(|&agent| match &role {
            Some(r) => r == agent.get_role_name(),
            None => true,
        })
        .choose(&mut rng)
        .cloned()
}

#[cfg(test)]
mod tests {
    use crate::{
        application::agent_service::pick_random_agent,
        domain::{
            agent::Agent,
            role::{Role, RoleName},
        },
    };

    #[test]
    fn should_pick_random_agent() {
        // GIVEN an array of agent
        let agents = vec![
            Agent::new(
                "1".to_string(),
                "Jett".to_string(),
                "icon".to_string(),
                Role::new("1".to_string(), RoleName::Duelist),
                vec![],
            ),
            Agent::new(
                "2".to_string(),
                "Reyna".to_string(),
                "icon".to_string(),
                Role::new("1".to_string(), RoleName::Duelist),
                vec![],
            ),
        ];

        // WHEN pick a random agent from that array
        let agent = pick_random_agent(&agents, None);

        // THEN it should return an agent
        assert!(agent.is_some());
    }
}
