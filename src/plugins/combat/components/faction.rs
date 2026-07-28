use bevy::prelude::*;

#[derive(Component, Clone, FromTemplate)]
pub enum Faction {
    #[default]
    Friendly,
    Zombie,
}

impl From<Faction> for FactionTemplate {
    fn from(value: Faction) -> Self {
        match value {
            Faction::Friendly => FactionTemplate::Friendly,
            Faction::Zombie => FactionTemplate::Zombie,
        }
    }
}
