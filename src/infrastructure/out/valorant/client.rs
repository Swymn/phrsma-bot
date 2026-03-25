use serde::Deserialize;

use crate::domain::agent::Agent;

const VALORANT_API_URL: &str = "https://valorant-api.com/v1/agents";

#[derive(Deserialize)]
struct ValorantApiResponse {
    data: Vec<Agent>,
}

pub async fn fetch_agents() -> Result<Vec<Agent>, reqwest::Error> {
    let response = reqwest::get(VALORANT_API_URL)
        .await?
        .json::<ValorantApiResponse>()
        .await?;

    Ok(response.data)
}

#[cfg(test)]
mod tests {
    use super::fetch_agents;

    #[tokio::test]
    async fn should_retrieve_some_agents() {
        // GIVEN a valorant client
        // WHEN fetching agents
        let agents = fetch_agents().await;

        // THEN the vector should not be empty
        assert!(agents.is_ok());
        assert!(!agents.unwrap().is_empty())
    }

    #[tokio::test]
    async fn should_correctly_map_into_model() {
        // GIVEN a valorant client
        // WHEN fetching agents
        let agents = fetch_agents().await;

        // THEN the vector should contains at least one agent
        assert!(!agents.as_ref().unwrap().is_empty());

        // AND the agent should have all his input defined
        let agent = agents.as_ref().unwrap().get(0).unwrap();
        assert!(!agent.get_id().is_empty());
        assert!(!agent.get_name().is_empty());
    }
}
