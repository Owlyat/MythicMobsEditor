use serde::{Deserialize, Serialize};

#[derive(strum::Display, Clone, Serialize, Deserialize, PartialEq, strum::EnumIter)]
pub enum Targeter {
    #[strum(to_string = "{0}")]
    SingleEntity(SingleEntityTarget),
    #[strum(to_string = "{0}")]
    MultyEntity(MultyEntity),
    #[strum(to_string = "{0}")]
    ThreatTable(ThreatTableTarget),
    #[strum(to_string = "{0}")]
    SingleLocation(SingleLocationTarget),
    #[strum(to_string = "{0}")]
    MultiLocation(MultiLocationTarget),
    #[strum(to_string = "{0}")]
    MetaEntity(MetaEntityTarget),
    #[strum(to_string = "{0}")]
    MetaLocation(MetaLocationTarget),
    #[strum(to_string = "@Region")]
    Region,
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
            Targeter::SingleEntity(_) => "Single Entity",
            Targeter::MultyEntity(_) => "Multi Entity",
            Targeter::ThreatTable(_) => "Threat Table",
            Targeter::SingleLocation(_) => "Single Location",
            Targeter::MultiLocation(_) => "Multi Location",
            Targeter::MetaEntity(_) => "Meta Entity",
            Targeter::MetaLocation(_) => "Meta Location",
            Targeter::Region => "Region",
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

impl From<ThreatTableTarget> for Targeter {
    fn from(value: ThreatTableTarget) -> Self {
        Self::ThreatTable(value)
    }
}

impl From<SingleLocationTarget> for Targeter {
    fn from(value: SingleLocationTarget) -> Self {
        Self::SingleLocation(value)
    }
}

impl From<MultiLocationTarget> for Targeter {
    fn from(value: MultiLocationTarget) -> Self {
        Self::MultiLocation(value)
    }
}

impl From<MetaEntityTarget> for Targeter {
    fn from(value: MetaEntityTarget) -> Self {
        Self::MetaEntity(value)
    }
}

impl From<MetaLocationTarget> for Targeter {
    fn from(value: MetaLocationTarget) -> Self {
        Self::MetaLocation(value)
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

    #[strum(to_string = "@MobsInRadius")]
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

    #[strum(to_string = "@TrackedPlayers")]
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

    #[strum(to_string = "@Bound")]
    ///Targets all living entities inside a cuboid, sphere, or cylinder
    Bound,
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
pub enum ThreatTableTarget {
    #[strum(to_string = "@ThreatTable")]
    #[default]
    ///Targets every entity on the casting mob's threat table
    ThreatTable,

    #[strum(to_string = "@ThreatTablePlayers")]
    ///Targets all the players on the casting mob's threat table
    ThreatTablePlayers,

    #[strum(to_string = "@RandomThreatTarget")]
    ///Targets a random entity on the casting mob's threat table
    RandomThreatTarget,

    #[strum(to_string = "@RandomThreatTargetLocation")]
    ///Targets the location of a random entity on the casting mob's threat table
    RandomThreatTargetLocation,
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
pub enum SingleLocationTarget {
    #[strum(to_string = "@SelfLocation")]
    #[default]
    ///Targets the caster's location
    SelfLocation,

    #[strum(to_string = "@SelfEyeLocation")]
    ///Targets the caster's eye location
    SelfEyeLocation,

    #[strum(to_string = "@Forward")]
    ///Targets a location in front of caster's facing direction
    Forward,

    #[strum(to_string = "@ProjectileForward")]
    ///Targets a location in front of the casting projectile, relative to its direction
    ProjectileForward,

    #[strum(to_string = "@TargetLocation")]
    ///Targets the caster's target's location
    TargetLocation,

    #[strum(to_string = "@TargetPredictedLocation")]
    ///Targets the predicted location of the caster's top target after an amount of ticks
    TargetPredictedLocation,

    #[strum(to_string = "@TriggerLocation")]
    ///Targets the location of the entity that triggered the skill
    TriggerLocation,

    #[strum(to_string = "@SpawnLocation")]
    ///Targets the world's spawn location
    SpawnLocation,

    #[strum(to_string = "@CasterSpawnLocation")]
    ///Targets the location the caster spawned at
    CasterSpawnLocation,

    #[strum(to_string = "@Location")]
    ///Targets the specified coordinates in the caster's world
    Location,

    #[strum(to_string = "@Origin")]
    ///Targets the location of the "origin" or "source" of a meta-skill
    Origin,

    #[strum(to_string = "@ObstructingBlock")]
    ///Tries to target the block in front of the caster that is obstructing it
    ObstructingBlock,

    #[strum(to_string = "@TargetBlock")]
    ///Targets the block the casting player is looking at
    TargetBlock,

    #[strum(to_string = "@BuildPlace")]
    ///Targets the block position where the casting player would place a block
    BuildPlace,

    #[strum(to_string = "@TrackedLocation")]
    ///Targets the mob's tracked location
    TrackedLocation,

    #[strum(to_string = "@NearestStructure")]
    ///Targets the nearest structure of the specified type within a radius
    NearestStructure,

    #[strum(to_string = "@VariableLocation")]
    ///Targets the location stored in the specified variable
    VariableLocation,

    #[strum(to_string = "@HighestBlock")]
    ///Targets the highest block at the skill origin
    HighestBlock,

    #[strum(to_string = "@PlayerLocationByName")]
    ///Targets a specific player's location by their name
    PlayerLocationByName,

    #[strum(to_string = "@EscapeLocation")]
    ///Targets a nearby safe escape location like how an Enderman would choose its teleport destination
    EscapeLocation,
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
pub enum MultiLocationTarget {
    #[strum(to_string = "@ForwardWall")]
    #[default]
    ///Targets a plane in front of the caster
    ForwardWall,

    #[strum(to_string = "@PlayerLocationsInRadius")]
    ///Targets all player locations in the given radius
    PlayerLocationsInRadius,

    #[strum(to_string = "@Pin")]
    ///Targets the location(s) of a pin
    Pin,

    #[strum(to_string = "@Ring")]
    ///Target points to form a ring of locations
    Ring,

    #[strum(to_string = "@RandomRingPoint")]
    ///Targets random points in a ring around the caster
    RandomRingPoint,

    #[strum(to_string = "@Cone")]
    ///Returns the number of target locations that comprise a cone
    Cone,

    #[strum(to_string = "@Sphere")]
    ///Targets points in a sphere around the caster
    Sphere,

    #[strum(to_string = "@Rectangle")]
    ///Returns the number of target locations that comprise a rectangle
    Rectangle,

    #[strum(to_string = "@RandomLocationsNearCaster")]
    ///Targets random locations near the caster
    RandomLocationsNearCaster,

    #[strum(to_string = "@RandomLocationsNearOrigin")]
    ///Targets random locations near the origin of a skill
    RandomLocationsNearOrigin,

    #[strum(to_string = "@BlocksNearOrigin")]
    ///Targets all blocks in a radius around the origin of the metaskill
    BlocksNearOrigin,

    #[strum(to_string = "@RingAroundOrigin")]
    ///Targets locations in a specified ring around the origin
    RingAroundOrigin,

    #[strum(to_string = "@Spawners")]
    ///Targets the location of the specified spawners
    Spawners,

    #[strum(to_string = "@BlocksInPinRegion")]
    ///Targets the blocks in a region delimited by two pins
    BlocksInPinRegion,

    #[strum(to_string = "@ChunksInWERegion")]
    ///Targets the 0,0 chunk corners of the chunks in a specified WorldGuard region
    ChunksInWERegion,
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
pub enum MetaEntityTarget {
    #[strum(to_string = "@LivingInLine")]
    #[default]
    ///Targets any entities in a line between the inherited target and the casting mob
    LivingInLine,

    #[strum(to_string = "@LivingNearTargetLocation")]
    ///Targets all living entities near the inherited target
    LivingNearTargetLocation,

    #[strum(to_string = "@PlayersNearTargetLocations")]
    ///Targets all players near the inherited targets
    PlayersNearTargetLocations,

    #[strum(to_string = "@TargetedTarget")]
    ///Targets the inherited targeted entities
    TargetedTarget,
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
pub enum MetaLocationTarget {
    #[strum(to_string = "@Line")]
    #[default]
    ///Targets locations between the mob and the inherited targets
    Line,

    #[strum(to_string = "@RandomLocationsNearTargets")]
    ///Targets random locations around the inherited targets
    RandomLocationsNearTargets,

    #[strum(to_string = "@FloorOfTargets")]
    ///Targets the blocks underneath the inherited targets
    FloorOfTargets,

    #[strum(to_string = "@LocationsOfTargets")]
    ///Targets the location of the inherited target entities
    LocationsOfTargets,

    #[strum(to_string = "@TargetedLocation")]
    ///Targets the location of the inherited target locations
    TargetedLocation,

    #[strum(to_string = "@BlocksInRadius")]
    ///Targets all blocks in a radius of the inherited targets
    BlocksInRadius,

    #[strum(to_string = "@BlocksInChunk")]
    ///Targets all blocks in a chunk relative to the inherited target
    BlocksInChunk,

    #[strum(to_string = "@BlockVein")]
    ///Target all adjacent blocks that match the blocktype, starting from the origin of the skill
    BlockVein,
}

impl Targeter {
    /// A short description of what this targeter category does.
    pub fn get_desc(&self) -> &'static str {
        match self {
            Targeter::SingleEntity(_) => "Targets a single entity",
            Targeter::MultyEntity(_) => "Targets multiple entities at once",
            Targeter::ThreatTable(_) => {
                "Targets entities on the caster's threat table (requires threat tables)"
            }
            Targeter::SingleLocation(_) => "Targets a single location",
            Targeter::MultiLocation(_) => "Targets multiple locations at once",
            Targeter::MetaEntity(_) => "Targets entities relative to an inherited target",
            Targeter::MetaLocation(_) => "Targets locations relative to an inherited target",
            Targeter::Region => "Targets a region (only works with specific mechanics)",
            Targeter::None => "No targeter (the mechanic uses no target)",
        }
    }
}

impl SingleEntityTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            SingleEntityTarget::SelfTarget => "Targets the caster of the mechanic",
            SingleEntityTarget::Target => "Targets the caster's target",
            SingleEntityTarget::Trigger => "Targets the entity that triggered the skill",
            SingleEntityTarget::NearestPlayer => "Targets the nearest player in radius",
            SingleEntityTarget::WolfOwner => "Targets the owner of the wolf",
            SingleEntityTarget::Owner => "Targets the owner of the mob or dropped item",
            SingleEntityTarget::Parent => "Targets the parent of the mob",
            SingleEntityTarget::Mount => "Targets the caster's original mount",
            SingleEntityTarget::Father => "Targets the father of the casting mob",
            SingleEntityTarget::Mother => "Targets the mother of the casting mob",
            SingleEntityTarget::Passenger => "Targets the rider of the casting mob",
            SingleEntityTarget::PlayerByName => {
                "Targets a specific player by name (supports placeholders)"
            }
            SingleEntityTarget::UniqueIdentifier => {
                "Targets a specific entity by UUID (supports placeholders)"
            }
            SingleEntityTarget::Vehicle => "Targets the caster's vehicle",
            SingleEntityTarget::InteractionLastAttacker => {
                "Targets the last entity that attacked the casting INTERACTION entity"
            }
            SingleEntityTarget::InteractionLastInteract => {
                "Targets the last entity that interacted with the casting INTERACTION entity"
            }
            SingleEntityTarget::OwnerLocation => "Targets the position of the owner of the mob",
            SingleEntityTarget::ParentLocation => "Targets the position of the parent of the mob",
        }
    }
}

impl MultyEntity {
    pub fn get_desc(&self) -> &'static str {
        match self {
            MultyEntity::LivingInCone => {
                "Targets all living entities in a cone relative to the caster's facing direction"
            }
            MultyEntity::LivingInWorld => "Targets all living entities in the caster's world",
            MultyEntity::NotLivingNearOrigin => {
                "Targets all non-living entities in a radius near the origin"
            }
            MultyEntity::PlayersInRadius => "Targets all players in the given radius",
            MultyEntity::MobsInRadius => {
                "Targets all MythicMobs or vanilla overrides of the given type in a radius"
            }
            MultyEntity::EntitiesInRadius => "Targets all entities in the given radius",
            MultyEntity::EntitiesInRing => "Targets all entities in the given ring",
            MultyEntity::EntitiesInRingNearOrigin => {
                "Targets all entities in the given ring around the origin"
            }
            MultyEntity::PlayerInWorld => "Targets all players in the current world",
            MultyEntity::PlayersOnServer => "Targets all players on the server",
            MultyEntity::PlayersInRing => {
                "Targets all players between the specified min and max radius"
            }
            MultyEntity::PlayersNearOrigin => {
                "Targets players near the origin of a meta-skill"
            }
            MultyEntity::TrackedPlayers => {
                "Targets players within the render distance of the caster"
            }
            MultyEntity::MobsNearOrigin => {
                "Targets all MythicMobs or vanilla overrides of the given type(s) near the origin"
            }
            MultyEntity::EntitiesNearOrigin => {
                "Targets all entities near the origin of a meta-skill"
            }
            MultyEntity::Children => "Targets any child entities summoned by the caster",
            MultyEntity::Siblings => {
                "Targets any mobs that share the same parent as the caster"
            }
            MultyEntity::ItemsNearOrigin => {
                "Targets item drops near the origin of a meta-skill"
            }
            MultyEntity::ItemsInRadius => "Targets all item drops in the given radius",
            MultyEntity::Bound => {
                "Targets all living entities inside a cuboid, sphere, or cylinder"
            }
        }
    }
}

impl ThreatTableTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            ThreatTableTarget::ThreatTable => {
                "Targets every entity on the casting mob's threat table"
            }
            ThreatTableTarget::ThreatTablePlayers => {
                "Targets all players on the casting mob's threat table"
            }
            ThreatTableTarget::RandomThreatTarget => {
                "Targets a random entity on the casting mob's threat table"
            }
            ThreatTableTarget::RandomThreatTargetLocation => {
                "Targets the location of a random entity on the casting mob's threat table"
            }
        }
    }
}

impl SingleLocationTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            SingleLocationTarget::SelfLocation => "Targets the caster's location",
            SingleLocationTarget::SelfEyeLocation => "Targets the caster's eye location",
            SingleLocationTarget::Forward => {
                "Targets a location in front of the caster's facing direction"
            }
            SingleLocationTarget::ProjectileForward => {
                "Targets a location in front of the casting projectile, relative to its direction"
            }
            SingleLocationTarget::TargetLocation => "Targets the caster's target's location",
            SingleLocationTarget::TargetPredictedLocation => {
                "Targets the predicted location of the caster's top target after an amount of ticks"
            }
            SingleLocationTarget::TriggerLocation => {
                "Targets the location of the entity that triggered the skill"
            }
            SingleLocationTarget::SpawnLocation => "Targets the world's spawn location",
            SingleLocationTarget::CasterSpawnLocation => {
                "Targets the location the caster spawned at"
            }
            SingleLocationTarget::Location => {
                "Targets the specified coordinates in the caster's world"
            }
            SingleLocationTarget::Origin => {
                "Targets the location of the origin/source of a meta-skill"
            }
            SingleLocationTarget::ObstructingBlock => {
                "Tries to target the block in front of the caster that is obstructing it"
            }
            SingleLocationTarget::TargetBlock => {
                "Targets the block the casting player is looking at"
            }
            SingleLocationTarget::BuildPlace => {
                "Targets the block position where the casting player would place a block"
            }
            SingleLocationTarget::TrackedLocation => "Targets the mob's tracked location",
            SingleLocationTarget::NearestStructure => {
                "Targets the nearest structure of the specified type within a radius"
            }
            SingleLocationTarget::VariableLocation => {
                "Targets the location stored in the specified variable"
            }
            SingleLocationTarget::HighestBlock => {
                "Targets the highest block at the skill origin"
            }
            SingleLocationTarget::PlayerLocationByName => {
                "Targets a specific player's location by their name"
            }
            SingleLocationTarget::EscapeLocation => {
                "Targets a nearby safe escape location, like an Enderman's teleport destination"
            }
        }
    }
}

impl MultiLocationTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            MultiLocationTarget::ForwardWall => "Targets a plane in front of the caster",
            MultiLocationTarget::PlayerLocationsInRadius => {
                "Targets all player locations in the given radius"
            }
            MultiLocationTarget::Pin => "Targets the location(s) of a pin",
            MultiLocationTarget::Ring => "Targets points that form a ring of locations",
            MultiLocationTarget::RandomRingPoint => {
                "Targets random points in a ring around the caster"
            }
            MultiLocationTarget::Cone => {
                "Returns the number of target locations that comprise a cone"
            }
            MultiLocationTarget::Sphere => "Targets points in a sphere around the caster",
            MultiLocationTarget::Rectangle => {
                "Returns the number of target locations that comprise a rectangle"
            }
            MultiLocationTarget::RandomLocationsNearCaster => {
                "Targets random locations near the caster"
            }
            MultiLocationTarget::RandomLocationsNearOrigin => {
                "Targets random locations near the origin of a skill"
            }
            MultiLocationTarget::BlocksNearOrigin => {
                "Targets all blocks in a radius around the origin of the metaskill"
            }
            MultiLocationTarget::RingAroundOrigin => {
                "Targets locations in a specified ring around the origin"
            }
            MultiLocationTarget::Spawners => {
                "Targets the location of the specified spawners"
            }
            MultiLocationTarget::BlocksInPinRegion => {
                "Targets the blocks in a region delimited by two pins"
            }
            MultiLocationTarget::ChunksInWERegion => {
                "Targets the 0,0 chunk corners of the chunks in a specified WorldGuard region"
            }
        }
    }
}

impl MetaEntityTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            MetaEntityTarget::LivingInLine => {
                "Targets any entities in a line between the inherited target and the casting mob"
            }
            MetaEntityTarget::LivingNearTargetLocation => {
                "Targets all living entities near the inherited target"
            }
            MetaEntityTarget::PlayersNearTargetLocations => {
                "Targets all players near the inherited targets"
            }
            MetaEntityTarget::TargetedTarget => "Targets the inherited targeted entities",
        }
    }
}

impl MetaLocationTarget {
    pub fn get_desc(&self) -> &'static str {
        match self {
            MetaLocationTarget::Line => {
                "Targets locations between the mob and the inherited targets"
            }
            MetaLocationTarget::RandomLocationsNearTargets => {
                "Targets random locations around the inherited targets"
            }
            MetaLocationTarget::FloorOfTargets => {
                "Targets the blocks underneath the inherited targets"
            }
            MetaLocationTarget::LocationsOfTargets => {
                "Targets the location of the inherited target entities"
            }
            MetaLocationTarget::TargetedLocation => {
                "Targets the location of the inherited target locations"
            }
            MetaLocationTarget::BlocksInRadius => {
                "Targets all blocks in a radius of the inherited targets"
            }
            MetaLocationTarget::BlocksInChunk => {
                "Targets all blocks in a chunk relative to the inherited target"
            }
            MetaLocationTarget::BlockVein => {
                "Targets all adjacent blocks that match the blocktype, starting from the origin of the skill"
            }
        }
    }
}
