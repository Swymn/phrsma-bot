use poise::{
    CreateReply,
    serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter, Timestamp},
};

use crate::domain::{agent::Agent, role::RoleName};

impl RoleName {
    fn get_color(&self) -> Colour {
        match self {
            RoleName::Duelist => Colour::ROSEWATER,
            RoleName::Controller => Colour::DARK_GOLD,
            RoleName::Initiator => Colour::DARK_BLUE,
            RoleName::Sentinel => Colour::DARK_TEAL,
        }
    }
}

pub fn create_embed(agent: &Agent) -> CreateReply {
    let fields = agent
        .get_abilities()
        .iter()
        .map(|ability| (ability.get_name(), ability.get_slot(), true));

    let embed = CreateEmbed::new()
        .title(format!("Tu dois jouer {}", agent.get_name()))
        .color(agent.get_role_name().get_color())
        .image(agent.get_icon())
        .fields(fields)
        .footer(CreateEmbedFooter::new("Amuse toi bien !"))
        .timestamp(Timestamp::now());

    CreateReply::default().embed(embed)
}
