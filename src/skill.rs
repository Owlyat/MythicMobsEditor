use mechanics::Mechanic;
use serde::{Deserialize, Serialize};
use targeters::Targeter;
use triggers::Trigger;

pub mod conditions;
pub mod mechanics;
pub mod params;
pub mod targeters;
pub mod triggers;
pub mod ui {}
pub mod minecraft_lib;

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Skill {
    pub name: String,
    pub mechanic: Option<Mechanic>,
    pub raw_args: String,
    pub targeter: Targeter,
    pub trigger: Trigger,
}

impl Skill {
    /// Set the name of the Skill, Takes ownership
    pub fn set_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    /// Rename the skill name
    pub fn rename(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = name.into();
        self
    }
}
