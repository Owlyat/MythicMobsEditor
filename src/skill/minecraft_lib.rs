use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr, VariantArray};
#[derive(
    Debug,
    Serialize,
    Deserialize,
    EnumString,
    Display,
    VariantArray,
    IntoStaticStr,
    PartialEq,
    Clone,
)]
pub enum MinecraftMob {
    #[strum(to_string = "MetaSkill")]
    MetaSkill,
    #[strum(serialize = "AcaciaBoat", to_string = "AcaciaBoat")]
    AcaciaBoat,
    #[strum(serialize = "AcaciaChestBoat", to_string = "AcaciaChestBoat")]
    AcaciaChestBoat,
    #[strum(serialize = "Allay", to_string = "Allay")]
    Allay,
    #[strum(serialize = "AreaEffectCloud", to_string = "AreaEffectCloud")]
    AreaEffectCloud,
    #[strum(serialize = "Armadillo", to_string = "Armadillo")]
    Armadillo,
    #[strum(serialize = "ArmorStand", to_string = "ArmorStand")]
    ArmorStand,
    #[strum(serialize = "Arrow", to_string = "Arrow")]
    Arrow,
    #[strum(serialize = "Axolotl", to_string = "Axolotl")]
    Axolotl,
    #[strum(serialize = "BambooChestRaft", to_string = "BambooChestRaft")]
    BambooChestRaft,
    #[strum(serialize = "BambooRaft", to_string = "BambooRaft")]
    BambooRaft,
    #[strum(serialize = "Bat", to_string = "Bat")]
    Bat,
    #[strum(serialize = "Bee", to_string = "Bee")]
    Bee,
    #[strum(serialize = "BirchBoat", to_string = "BirchBoat")]
    BirchBoat,
    #[strum(serialize = "BirchChestBoat", to_string = "BirchChestBoat")]
    BirchChestBoat,
    #[strum(serialize = "Blaze", to_string = "Blaze")]
    Blaze,
    #[strum(serialize = "BlockDisplay", to_string = "BlockDisplay")]
    BlockDisplay,
    #[strum(serialize = "Bogged", to_string = "Bogged")]
    Bogged,
    #[strum(serialize = "Breeze", to_string = "Breeze")]
    Breeze,
    #[strum(serialize = "BreezeWindCharge", to_string = "BreezeWindCharge")]
    BreezeWindCharge,
    #[strum(serialize = "Camel", to_string = "Camel")]
    Camel,
    #[strum(serialize = "Cat", to_string = "Cat")]
    Cat,
    #[strum(serialize = "CaveSpider", to_string = "CaveSpider")]
    CaveSpider,
    #[strum(serialize = "CherryBoat", to_string = "CherryBoat")]
    CherryBoat,
    #[strum(serialize = "CherryChestBoat", to_string = "CherryChestBoat")]
    CherryChestBoat,
    #[strum(serialize = "ChestMinecart", to_string = "ChestMinecart")]
    ChestMinecart,
    #[strum(serialize = "Chicken", to_string = "Chicken")]
    Chicken,
    #[strum(serialize = "Cod", to_string = "Cod")]
    Cod,
    #[strum(serialize = "CommandBlockMinecart", to_string = "CommandBlockMinecart")]
    CommandBlockMinecart,
    #[strum(serialize = "Cow", to_string = "Cow")]
    Cow,
    #[strum(serialize = "Creaking", to_string = "Creaking")]
    Creaking,
    #[strum(serialize = "Creeper", to_string = "Creeper")]
    Creeper,
    #[strum(serialize = "DarkOakBoat", to_string = "DarkOakBoat")]
    DarkOakBoat,
    #[strum(serialize = "DarkOakChestBoat", to_string = "DarkOakChestBoat")]
    DarkOakChestBoat,
    #[strum(serialize = "Dolphin", to_string = "Dolphin")]
    Dolphin,
    #[strum(serialize = "Donkey", to_string = "Donkey")]
    Donkey,
    #[strum(serialize = "DragonFireball", to_string = "DragonFireball")]
    DragonFireball,
    #[strum(serialize = "Drowned", to_string = "Drowned")]
    Drowned,
    #[strum(serialize = "Egg", to_string = "Egg")]
    Egg,
    #[strum(serialize = "ElderGuardian", to_string = "ElderGuardian")]
    ElderGuardian,
    #[strum(serialize = "EndCrystal", to_string = "EndCrystal")]
    EndCrystal,
    #[strum(serialize = "EnderDragon", to_string = "EnderDragon")]
    EnderDragon,
    #[strum(serialize = "EnderPearl", to_string = "EnderPearl")]
    EnderPearl,
    #[strum(serialize = "Enderman", to_string = "Enderman")]
    Enderman,
    #[strum(serialize = "Endermite", to_string = "Endermite")]
    Endermite,
    #[strum(serialize = "Evoker", to_string = "Evoker")]
    Evoker,
    #[strum(serialize = "EvokerFangs", to_string = "EvokerFangs")]
    EvokerFangs,
    #[strum(serialize = "ExperienceBottle", to_string = "ExperienceBottle")]
    ExperienceBottle,
    #[strum(serialize = "ExperienceOrb", to_string = "ExperienceOrb")]
    ExperienceOrb,
    #[strum(serialize = "EyeOfEnder", to_string = "EyeOfEnder")]
    EyeOfEnder,
    #[strum(serialize = "FallingBlock", to_string = "FallingBlock")]
    FallingBlock,
    #[strum(serialize = "Fireball", to_string = "Fireball")]
    Fireball,
    #[strum(serialize = "FireworkRocket", to_string = "FireworkRocket")]
    FireworkRocket,
    #[strum(serialize = "FishingBobber", to_string = "FishingBobber")]
    FishingBobber,
    #[strum(serialize = "Fox", to_string = "Fox")]
    Fox,
    #[strum(serialize = "Frog", to_string = "Frog")]
    Frog,
    #[strum(serialize = "FurnaceMinecart", to_string = "FurnaceMinecart")]
    FurnaceMinecart,
    #[strum(serialize = "Ghast", to_string = "Ghast")]
    Ghast,
    #[strum(serialize = "Giant", to_string = "Giant")]
    Giant,
    #[strum(serialize = "GlowItemFrame", to_string = "GlowItemFrame")]
    GlowItemFrame,
    #[strum(serialize = "GlowSquid", to_string = "GlowSquid")]
    GlowSquid,
    #[strum(serialize = "Goat", to_string = "Goat")]
    Goat,
    #[strum(serialize = "Guardian", to_string = "Guardian")]
    Guardian,
    #[strum(serialize = "HappyGhast", to_string = "HappyGhast")]
    HappyGhast,
    #[strum(serialize = "Hoglin", to_string = "Hoglin")]
    Hoglin,
    #[strum(serialize = "HopperMinecart", to_string = "HopperMinecart")]
    HopperMinecart,
    #[strum(serialize = "Horse", to_string = "Horse")]
    Horse,
    #[strum(serialize = "Husk", to_string = "Husk")]
    Husk,
    #[strum(serialize = "Illusioner", to_string = "Illusioner")]
    Illusioner,
    #[strum(serialize = "Interaction", to_string = "Interaction")]
    Interaction,
    #[strum(serialize = "IronGolem", to_string = "IronGolem")]
    IronGolem,
    #[strum(serialize = "Item", to_string = "Item")]
    Item,
    #[strum(serialize = "ItemDisplay", to_string = "ItemDisplay")]
    ItemDisplay,
    #[strum(serialize = "ItemFrame", to_string = "ItemFrame")]
    ItemFrame,
    #[strum(serialize = "JungleBoat", to_string = "JungleBoat")]
    JungleBoat,
    #[strum(serialize = "JungleChestBoat", to_string = "JungleChestBoat")]
    JungleChestBoat,
    #[strum(serialize = "LeashKnot", to_string = "LeashKnot")]
    LeashKnot,
    #[strum(serialize = "LightningBolt", to_string = "LightningBolt")]
    LightningBolt,
    #[strum(serialize = "LingeringPotion", to_string = "LingeringPotion")]
    LingeringPotion,
    #[strum(serialize = "Llama", to_string = "Llama")]
    Llama,
    #[strum(serialize = "LlamaSpit", to_string = "LlamaSpit")]
    LlamaSpit,
    #[strum(serialize = "MagmaCube", to_string = "MagmaCube")]
    MagmaCube,
    #[strum(serialize = "MangroveBoat", to_string = "MangroveBoat")]
    MangroveBoat,
    #[strum(serialize = "MangroveChestBoat", to_string = "MangroveChestBoat")]
    MangroveChestBoat,
    #[strum(serialize = "Marker", to_string = "Marker")]
    Marker,
    #[strum(serialize = "Minecart", to_string = "Minecart")]
    Minecart,
    #[strum(serialize = "Mooshroom", to_string = "Mooshroom")]
    Mooshroom,
    #[strum(serialize = "Mule", to_string = "Mule")]
    Mule,
    #[strum(serialize = "OakBoat", to_string = "OakBoat")]
    OakBoat,
    #[strum(serialize = "OakChestBoat", to_string = "OakChestBoat")]
    OakChestBoat,
    #[strum(serialize = "Ocelot", to_string = "Ocelot")]
    Ocelot,
    #[strum(serialize = "OminousItemSpawner", to_string = "OminousItemSpawner")]
    OminousItemSpawner,
    #[strum(serialize = "Painting", to_string = "Painting")]
    Painting,
    #[strum(serialize = "PaleOakBoat", to_string = "PaleOakBoat")]
    PaleOakBoat,
    #[strum(serialize = "PaleOakChestBoat", to_string = "PaleOakChestBoat")]
    PaleOakChestBoat,
    #[strum(serialize = "Panda", to_string = "Panda")]
    Panda,
    #[strum(serialize = "Parrot", to_string = "Parrot")]
    Parrot,
    #[strum(serialize = "Phantom", to_string = "Phantom")]
    Phantom,
    #[strum(serialize = "Pig", to_string = "Pig")]
    Pig,
    #[strum(serialize = "Piglin", to_string = "Piglin")]
    Piglin,
    #[strum(serialize = "PiglinBrute", to_string = "PiglinBrute")]
    PiglinBrute,
    #[strum(serialize = "Pillager", to_string = "Pillager")]
    Pillager,
    #[strum(serialize = "Player", to_string = "Player")]
    Player,
    #[strum(serialize = "PolarBear", to_string = "PolarBear")]
    PolarBear,
    #[strum(serialize = "Pufferfish", to_string = "Pufferfish")]
    Pufferfish,
    #[strum(serialize = "Rabbit", to_string = "Rabbit")]
    Rabbit,
    #[strum(serialize = "Ravager", to_string = "Ravager")]
    Ravager,
    #[strum(serialize = "Salmon", to_string = "Salmon")]
    Salmon,
    #[strum(serialize = "Sheep", to_string = "Sheep")]
    Sheep,
    #[strum(serialize = "Shulker", to_string = "Shulker")]
    Shulker,
    #[strum(serialize = "ShulkerBullet", to_string = "ShulkerBullet")]
    ShulkerBullet,
    #[strum(serialize = "Silverfish", to_string = "Silverfish")]
    Silverfish,
    #[strum(serialize = "Skeleton", to_string = "Skeleton")]
    Skeleton,
    #[strum(serialize = "SkeletonHorse", to_string = "SkeletonHorse")]
    SkeletonHorse,
    #[strum(serialize = "Slime", to_string = "Slime")]
    Slime,
    #[strum(serialize = "SmallFireball", to_string = "SmallFireball")]
    SmallFireball,
    #[strum(serialize = "Sniffer", to_string = "Sniffer")]
    Sniffer,
    #[strum(serialize = "SnowGolem", to_string = "SnowGolem")]
    SnowGolem,
    #[strum(serialize = "Snowball", to_string = "Snowball")]
    Snowball,
    #[strum(serialize = "SpawnerMinecart", to_string = "SpawnerMinecart")]
    SpawnerMinecart,
    #[strum(serialize = "SpectralArrow", to_string = "SpectralArrow")]
    SpectralArrow,
    #[strum(serialize = "Spider", to_string = "Spider")]
    Spider,
    #[strum(serialize = "SplashPotion", to_string = "SplashPotion")]
    SplashPotion,
    #[strum(serialize = "SpruceBoat", to_string = "SpruceBoat")]
    SpruceBoat,
    #[strum(serialize = "SpruceChestBoat", to_string = "SpruceChestBoat")]
    SpruceChestBoat,
    #[strum(serialize = "Squid", to_string = "Squid")]
    Squid,
    #[strum(serialize = "Stray", to_string = "Stray")]
    Stray,
    #[strum(serialize = "Strider", to_string = "Strider")]
    Strider,
    #[strum(serialize = "Tadpole", to_string = "Tadpole")]
    Tadpole,
    #[strum(serialize = "TextDisplay", to_string = "TextDisplay")]
    TextDisplay,
    #[strum(serialize = "Tnt", to_string = "Tnt")]
    Tnt,
    #[strum(serialize = "TntMinecart", to_string = "TntMinecart")]
    TntMinecart,
    #[strum(serialize = "TraderLlama", to_string = "TraderLlama")]
    TraderLlama,
    #[strum(serialize = "Trident", to_string = "Trident")]
    Trident,
    #[strum(serialize = "TropicalFish", to_string = "TropicalFish")]
    TropicalFish,
    #[strum(serialize = "Turtle", to_string = "Turtle")]
    Turtle,
    #[strum(serialize = "Unknown", to_string = "Unknown")]
    Unknown,
    #[strum(serialize = "Vex", to_string = "Vex")]
    Vex,
    #[strum(serialize = "Villager", to_string = "Villager")]
    Villager,
    #[strum(serialize = "Vindicator", to_string = "Vindicator")]
    Vindicator,
    #[strum(serialize = "WanderingTrader", to_string = "WanderingTrader")]
    WanderingTrader,
    #[strum(serialize = "Warden", to_string = "Warden")]
    Warden,
    #[strum(serialize = "WindCharge", to_string = "WindCharge")]
    WindCharge,
    #[strum(serialize = "Witch", to_string = "Witch")]
    Witch,
    #[strum(serialize = "Wither", to_string = "Wither")]
    Wither,
    #[strum(serialize = "WitherSkeleton", to_string = "WitherSkeleton")]
    WitherSkeleton,
    #[strum(serialize = "WitherSkull", to_string = "WitherSkull")]
    WitherSkull,
    #[strum(serialize = "Wolf", to_string = "Wolf")]
    Wolf,
    #[strum(serialize = "Zoglin", to_string = "Zoglin")]
    Zoglin,
    #[strum(serialize = "Zombie", to_string = "Zombie")]
    Zombie,
    #[strum(serialize = "ZombieHorse", to_string = "ZombieHorse")]
    ZombieHorse,
    #[strum(serialize = "ZombieVillager", to_string = "ZombieVillager")]
    ZombieVillager,
    #[strum(serialize = "ZombifiedPiglin", to_string = "ZombifiedPiglin")]
    ZombifiedPiglin,
}

impl Default for MinecraftMob {
    fn default() -> Self {
        MinecraftMob::Zombie
    }
}
