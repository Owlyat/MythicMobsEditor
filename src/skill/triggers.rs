use std::default;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, strum::VariantArray, strum::Display, Default,
)]
pub enum Trigger {
    #[strum(to_string = "~onCombat")]
    ///Default
    Combat,

    #[strum(to_string = "~onAttack")]
    ///When the mob hits something
    Attack,

    #[strum(to_string = "~onDamaged")]
    ///When the mob is damaged
    Damaged,

    #[strum(to_string = "~onSpawn")]
    ///When the mob spawns
    Spawn,

    #[strum(to_string = "~onDespawn")]
    ///When the mob is despawned
    Despawn,

    #[strum(to_string = "~onReady")]
    ///Triggered the first time a mob is spawned from a spawner
    Ready,

    #[strum(to_string = "~onLoad")]
    ///When the mob is loaded after a server restart
    Load,

    #[strum(to_string = "~onSpawnOrLoad")]
    ///When the mob either [spawns](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/Skills/Triggers/onspawn) or [loads](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/Skills/Triggers/onload)
    SpawnOrLoad,

    #[strum(to_string = "~onDeath")]
    ///When the mob dies
    Death,

    #[strum(to_string = "~onTimer")]
    ///Every # ticks (where # is the interval in ticks)
    Timer,

    #[strum(to_string = "~onInteract")]
    /// When the mob is right-clicked
    Interact,

    #[strum(to_string = "~onPlayerKill")]
    ///When the mob kills a player
    PlayerKill,

    #[strum(to_string = "~onEnterCombat")]
    ///When the mob enters combat (requires threat tables be on)
    EnterCombat,

    #[strum(to_string = "~onDropCombat")]
    ///When the mob leaves combat (requires threat tables be on)
    DropCombat,

    #[strum(to_string = "~onChangeTarget")]
    ///When the mob changes targets (requires threat tables be on)
    ChangeTarget,

    #[strum(to_string = "~onExplode")]
    ///When the mob explodes (typically only used for creepers)
    Explode,

    #[strum(to_string = "~onPrime")]
    ///When the creeper charges up for an explosion
    Prime,

    #[strum(to_string = "~onCreeperCharge")]
    ///When the creeper is charged (when lightning hits a creeper)
    CreeperCharge,

    #[strum(to_string = "~onTeleport")]
    ///When the mob teleports (typically only used for endermen)
    Teleport,

    #[strum(to_string = "~onSignal")]
    ///When the mob receives a signal
    Signal,

    #[strum(to_string = "~onShoot")]
    ///When the mob fires a projectile
    Shoot,

    #[strum(to_string = "~onBowHit")]
    ///When the mob's fired projectile hits an entity
    BowHit,

    #[strum(to_string = "~onTame")]
    ///When the mob gets tamed
    Tame,

    #[strum(to_string = "~onBreed")]
    ///When the mob breeds with another mob.
    Breed,

    #[strum(to_string = "~onTrade")]
    ///When the Villager completes a trade. Requires Paper
    Trade,

    #[strum(to_string = "~onChangeWorld")]
    ///When the mob changes world
    ChangeWorld,

    #[strum(to_string = "~onBucket")]
    ///When the cow is milked or an entity is bucketed (axolotl etc.)
    Bucket,

    #[strum(to_string = "~onSkillDamage")]
    ///When the mob deals damage to other entities via a mechanic
    SkillDamage,

    #[strum(to_string = "~onHear")]
    ///When the mob hears a sound, [if enabled](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/Mobs/Mobs#hearing)
    Hear,

    #[strum(to_string = "~onProjectileHit")]
    ///When a mob's special projectile hits an entity
    ProjectileHit,

    #[strum(to_string = "~onProjectileLand")]
    ///When a mob's special projectile hits a block
    ProjectileLand,

    #[strum(to_string = "")]
    #[default]
    None,
}

impl Trigger {
    pub fn get_fields(&self) -> impl Into<String> {
        match self {
            Trigger::Combat => "Combat",
            Trigger::Attack => "Attack",
            Trigger::Damaged => "Damaged",
            Trigger::Spawn => "Spawn",
            Trigger::Despawn => "Despawn",
            Trigger::Ready => "Ready",
            Trigger::Load => "Load",
            Trigger::SpawnOrLoad => "Spawn Or Load",
            Trigger::Death => "Death",
            Trigger::Timer => "Timer",
            Trigger::Interact => "Interact",
            Trigger::PlayerKill => "Player Kill",
            Trigger::EnterCombat => "Enter Combat",
            Trigger::DropCombat => "Drop Combat",
            Trigger::ChangeTarget => "Change Target",
            Trigger::Explode => "Explode",
            Trigger::Prime => "Prime",
            Trigger::CreeperCharge => "Creeper Charge",
            Trigger::Teleport => "Teleport",
            Trigger::Signal => "Signal",
            Trigger::Shoot => "Shoot",
            Trigger::BowHit => "Bow Hit",
            Trigger::Tame => "Tame",
            Trigger::Breed => "Breed",
            Trigger::Trade => "Trade",
            Trigger::ChangeWorld => "Change World",
            Trigger::Bucket => "Bucket",
            Trigger::SkillDamage => "Skill Damage",
            Trigger::Hear => "Hear",
            Trigger::ProjectileHit => "Projectile Hit",
            Trigger::ProjectileLand => "Projectile Land",
            Trigger::None => "None",
        }
    }
}
