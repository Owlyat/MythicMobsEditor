use serde::{Deserialize, Serialize};

#[derive(strum::Display, Clone, Serialize, Deserialize, PartialEq, strum::EnumIter)]
pub enum Targeter {
    #[strum(to_string = "{0}")]
    SingleEntity(SingleEntityTarget),
    #[strum(to_string = "{0}")]
    MultyEntity(MultyEntity),
    ThreatTable,
    #[strum(to_string = "")]
    None,
}

impl Default for Targeter {
    fn default() -> Self {
        Self::None
    }
}

impl Targeter {
    pub fn get_fields(&self) -> impl Into<String> {
        match self {
            Targeter::SingleEntity(single_entity_target) => "Single Entity",
            Targeter::MultyEntity(multy_entity) => "Multi Entity",
            Targeter::ThreatTable => "Threat Table",
            Targeter::None => "None",
        }
    }
}

impl From<SingleEntityTarget> for Targeter {
    fn from(value: SingleEntityTarget) -> Self {
        Self::SingleEntity(value)
    }
}

impl From<MultyEntity> for Targeter {
    fn from(value: MultyEntity) -> Self {
        Self::MultyEntity(value)
    }
}

#[derive(
    strum::Display,
    strum::EnumString,
    Clone,
    Serialize,
    Deserialize,
    strum::VariantArray,
    PartialEq,
    Default,
)]
pub enum SingleEntityTarget {
    #[strum(to_string = "@Self")]
    #[default]
    ///Targets the caster of the mechanic
    SelfTarget,

    #[strum(to_string = "@Target")]
    ///Targets the caster's target
    Target,

    #[strum(to_string = "@Trigger")]
    ///Targets the entity that triggered the skill
    Trigger,

    #[strum(to_string = "@NearestPlayer")]
    ///Targets the nearest player in radius
    NearestPlayer,

    #[strum(to_string = "@WolfOwner")]
    ///Targets the owner of the wolf
    WolfOwner,

    #[strum(to_string = "@Owner")]
    ///Targets the [owner](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/skills/mechanics/setowner) of the mob
    Owner,

    #[strum(to_string = "@Parent")]
    ///Targets the [parent](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/skills/mechanics/setparent) of the mob
    Parent,

    #[strum(to_string = "@Mount")]
    ///Targets the caster's original [mount](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/Mobs/Mobs#mount)
    Mount,

    #[strum(to_string = "@Father")]
    ///Targets the father of the casting mob.
    Father,

    #[strum(to_string = "@Mother")]
    ///Targets the mother of the casting mob.
    Mother,

    #[strum(to_string = "@Passenger")]
    ///Targets the rider of the casting
    Passenger,

    #[strum(to_string = "@PlayerByName")]
    ///Targets a specific player by name. Supports placeholders
    PlayerByName,

    #[strum(to_string = "@UniqueIdentifier")]
    ///Targets a specific entity by their UUID, supports placeholders
    UniqueIdentifier,

    #[strum(to_string = "@Vehicle")]
    ///Targets the caster's vehicle
    Vehicle,

    #[strum(to_string = "@InteractionLastAttacker")]
    ///Targets the last entity that attacked the casting INTERACTION entity
    InteractionLastAttacker,

    #[strum(to_string = "@InteractionLastInteract")]
    ///Targets the last entity that interacted with the casting INTERACTION entity
    InteractionLastInteract,

    #[strum(to_string = "@OwnerLocation")]
    ///Targets the position of the owner of the mob
    OwnerLocation,

    #[strum(to_string = "@ParentLocation")]
    ///Targets the position of the parent of the mob
    ParentLocation,
}

#[derive(
    strum::Display,
    strum::EnumString,
    Clone,
    Serialize,
    Deserialize,
    strum::VariantArray,
    PartialEq,
    Default,
)]
pub enum MultyEntity {
    #[strum(to_string = "@LivingInCone")]
    ///Targets all living entities in cone with a specified angle, length and rotation relative to facing direction
    LivingInCone,

    #[strum(to_string = "@LivingInWorld")]
    ///Targets all living entities in the caster's world
    LivingInWorld,

    #[strum(to_string = "@NotLivingNearOrigin")]
    ///Targets all non living entities in a radius near the origin
    NotLivingNearOrigin,

    #[strum(to_string = "@PlayersInRadius")]
    #[default]
    ///Targets all players in the given radius
    PlayersInRadius,

    #[strum(to_string = "@MobsInradius")]
    ///Targets all mythicmobs or vanilla overrides of the given type in a radius
    MobsInRadius,

    #[strum(to_string = "@EntitiesInRadius")]
    ///Targets all entities in the given radius
    EntitiesInRadius,

    #[strum(to_string = "@EntitiesInRing")]
    ///Targets all entities in the given ring.
    EntitiesInRing,

    #[strum(to_string = "@EntitiesInRingNearOrigin")]
    ///Targets all entities in the given ring around the origin.
    EntitiesInRingNearOrigin,

    #[strum(to_string = "@PlayersInWorld")]
    ///Targets all players in the current world.
    PlayerInWorld,

    #[strum(to_string = "@PlayersOnServer")]
    ///Targets all players in the server
    PlayersOnServer,

    #[strum(to_string = "@PlayersInRing")]
    ///Target all players between the specified min and max radius.
    PlayersInRing,

    #[strum(to_string = "@PlayersNearOrigin")]
    ///Targets players near the [origin](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/skills/targeters/origin) of a meta-skill.
    PlayersNearOrigin,

    #[strum(to_string = "@TrackerPlayers")]
    ///Targets players that are within the render distance of the caster
    TrackedPlayers,

    #[strum(to_string = "@MobsNearOrigin")]
    ///Targets all MythicMobs or vanilla overrides of the given type(s) in a radius around the origin
    MobsNearOrigin,

    #[strum(to_string = "@EntitiesNearOrigin")]
    ///Targets all entities near the [origin](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/skills/targeters/origin) of a meta-skill
    EntitiesNearOrigin,

    #[strum(to_string = "@Children")]
    ///Targets any child entities summoned by the caster.
    Children,

    #[strum(to_string = "@Siblings")]
    ///Targets any mobs that share the same parent as the caster.
    Siblings,

    #[strum(to_string = "@ItemsNearOrigin")]
    ///Targets item drops near the [origin](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/skills/targeters/origin) of a meta-skill.
    ItemsNearOrigin,

    #[strum(to_string = "@ItemsInRadius")]
    ///Targets all item drops in the given radius
    ItemsInRadius,
}
