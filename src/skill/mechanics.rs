use std::fmt::Display;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Deserialize, Serialize, Clone, Default, PartialEq)]
pub enum MythicOption<T> {
    #[default]
    None,
    Some(String, T, String),
}
impl<T> MythicOption<T> {
    pub fn some(&mut self) -> Option<(&mut String, &mut T, &mut String)> {
        match self {
            Self::None => None,
            Self::Some(pre, t, suf) => Some((pre, t, suf)),
        }
    }
}

impl<T> std::fmt::Display for MythicOption<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MythicOption::None => write!(f, ""),
            MythicOption::Some(pre, t, suf) => write!(f, "{pre}{t}{suf}"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Xyz {
    x: i32,
    y: i32,
    z: i32,
}
impl std::fmt::Display for Xyz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

#[derive(
    Serialize,
    Deserialize,
    strum::Display,
    Clone,
    strum::EnumDiscriminants,
    strum::EnumIter,
    PartialEq,
)]
pub enum Mechanic {
    /// Activates a MythicMobs spawner at the targeted location
    #[strum(to_string = "- activatespawner{{spawner={spawner}}}")]
    ActivateSpawner { spawner: SpawnerSelect },

    /// Changes the trades of a villager
    #[strum(
        to_string = "- addTrade{{item1={ingredient}{ingredient_2};result={result}{experience_reward}{villager_exp}{price_multiplier}}}"
    )]
    AddTrade {
        /// The action to perform. Can be `ADD`, `REMOVE`, `REPLACE`.
        action: ActionMode,
        /// The slot to be selected for the action. Slot starts at 0, so if a villager has 3 trades, the middle trade would be slot 1
        slot: u8,
        /// The first ingredient
        ingredient: TradeIngredient,
        /// The second ingredient
        ingredient_2: MythicOption<TradeIngredient>,
        /// The result item of the trade
        result: String,
        /// The uses of trade
        max_uses: MythicOption<u8>,
        /// If the trade should drop experience
        experience_reward: MythicOption<bool>,
        /// The amount of experience to give to the villager upon successful trade
        villager_exp: MythicOption<u16>,
        /// The multiplier for the price when the player has made the villager angry
        price_multiplier: MythicOption<u8>,
        /// The demand of the trade
        demand: MythicOption<u8>,
        /// The special price for when the villager is friendly to the player (player reputation or hero of the village effect)
        special_price: MythicOption<u16>,
        /// If the discounts should be ignored
        ignore_discounts: MythicOption<bool>,
    },

    /// Animates an armor stand
    #[strum(to_string = "- animateArmorStand{{pose={pose};speed={speed};duration={duration}}}")]
    AnimateArmorStand {
        /// The pose to animate the armor stand to.
        pose: ArmorStandPose,
        /// The speed of the animation.
        speed: f32,
        /// The duration of the animation in ticks.
        duration: u32,
        /// Whether to ignore empty pose values.
        ignore_empty: bool,
        /// Whether to use smart animation.
        smart: bool,
    },

    /// Makes the caster swing their arm
    #[strum(to_string = "- armAnimation @self")]
    ArmAnimation,

    /// Fires a volley of arrows
    #[strum(
        to_string = "- arrowvolley{{a={amount};s={spread};v={velocity};f={fire_ticks};rd={remove_delay};pickup={can_pickup}}}"
    )]
    ArrowVolley {
        /// The number of arrows in the volley.
        amount: u32,
        /// How spread out the arrows are.
        spread: f32,
        /// The velocity of the arrows.
        velocity: f32,
        /// The duration hit entities will burn for in ticks.
        fire_ticks: u32,
        /// The time the arrows will stay before disappearing in ticks.
        remove_delay: u32,
        /// Whether the arrows can be picked up by players.
        can_pickup: bool,
    },

    /// Adds an attribute modifier to the attributable target
    #[strum(to_string = "- auraremove{{aura={aura_name};stacks={stacks}}}")]
    AuraRemove {
        /// The name of the aura to remove.
        aura_name: String,
        /// The amount of stacks to remove.
        stacks: u32,
    },

    /// Creates a custom boss bar on the casting mob
    #[strum(to_string = "- blackscreen{{d={duration}}}")]
    BlackScreen {
        /// The duration of the black screen effect.
        duration: u32,
    },

    /// Causes the targeted blocks to fall, as if affected by gravity
    #[strum(to_string = "- blockdestabilize")]
    BlockDestabilize,

    /// Temporarily masks a block as a different block
    #[strum(
        to_string = "- effect:blockmask{{m={material};r={radius};ry={radius_y};n={noise};d={duration};s={shape};na={no_air};oa={only_air};occ={occlude}}}"
    )]
    BlockMask {
        /// The type of block used for the blockmask.
        material: String,
        /// The radius of the blockmask effect.
        radius: u32,
        /// The y component of the radius.
        radius_y: u32,
        /// Defines the randomness of the effect.
        noise: u32,
        /// Duration of the effect in ticks.
        duration: u32,
        /// The shape of the effect.
        shape: Shape,
        /// Mask no air blocks only.
        no_air: bool,
        /// Mask air blocks only.
        only_air: bool,
        /// If only_air is used, target transparent blocks as well.
        occlude: bool,
    },

    /// Unmasks blocks that have been masked
    #[strum(to_string = "- effect:blockunmask{{r={radius};s={shape}}}")]
    BlockUnmask {
        /// The radius of the blockunmask effect.
        radius: u32,
        /// The shape of the effect.
        shape: Shape,
    },

    /// Triggers a block physics update at the target location
    #[strum(to_string = "- blockphysics")]
    BlockPhysics,

    /// Creates a wave of blocks at the target location
    #[strum(
        to_string = "- blockwave{{m={material};r={radius};ry={radius_y};d={duration};s={shape};v={velocity};vh={horizontal_velocity};sv={specific_velocities};vx={velocity_x};vy={velocity_y};vz={velocity_z};n={noise};hsb={hide_source_block};ia={ignore_air}}}"
    )]
    BlockWave {
        /// The material used for the blockwave.
        material: String,
        /// The radius of the blockwave effect.
        radius: u32,
        /// The y radius of the blockwave effect.
        radius_y: u32,
        /// Duration of the effect in ticks.
        duration: u32,
        /// The shape of the effect.
        shape: Shape,
        /// The speed of the effect.
        velocity: f32,
        /// The speed of the effect in the horizontal direction.
        horizontal_velocity: f32,
        /// Whether to make use of the specific velocities.
        specific_velocities: bool,
        /// The speed of the effect on the x axis.
        velocity_x: f32,
        /// The speed of the effect on the y axis.
        velocity_y: f32,
        /// The speed of the effect on the z axis.
        velocity_z: f32,
        /// The noise of the effect.
        noise: u32,
        /// Whether to hide the source block.
        hide_source_block: bool,
        /// Whether air blocks should be ignored.
        ignore_air: bool,
    },

    /// Makes the target's screen glow red
    #[strum(to_string = "- effect:bloodyScreen{{d={duration};c={cancel}}}")]
    BloodyScreen {
        /// The time (in ticks) that the effect is active.
        duration: u32,
        /// If true, it stops any existing redscreen.
        cancel: bool,
    },

    /// Applies a bone meal effect to the target blocks
    #[strum(to_string = "- bonemeal{{bf={block_face}}}")]
    BoneMeal {
        /// The block face to apply bonemeal to.
        block_face: String,
    },

    /// Creates an inescapable border around the mob
    #[strum(to_string = "- bossBorder{{r={radius}}}")]
    BossBorder {
        /// The radius of the border.
        radius: u32,
    },

    /// Applies an aura to the target that makes it bouncy
    #[strum(
        to_string = "- bouncy{{auraName={aura_name};onBounceSkill={on_bounce_skill};ce={cancel_event}}}"
    )]
    Bouncy {
        /// The name of the aura.
        aura_name: String,
        /// The metaskill to execute on bounce.
        on_bounce_skill: String,
        /// Whether to cancel fall damage for the duration of the aura.
        cancel_event: bool,
    },

    /// Breaks the block at the target location
    #[strum(to_string = "- breakblock{{d={do_drops};e={do_effect};t={use_tool}}}")]
    BreakBlock {
        /// Whether or not to drop the block.
        do_drops: bool,
        /// Whether or not to play the break block particles.
        do_effect: bool,
        /// Whether or not to use the tool in the players hands.
        use_tool: bool,
    },

    /// Breaks the block at the target location and gives an item/droptable
    #[strum(
        to_string = "- breakBlockAndGiveItem{{d={do_drops};e={do_effect};t={use_tool};fl={do_fake_looting};i={items}}}"
    )]
    BreakBlockAndGiveItem {
        /// Whether or not to drop the block.
        do_drops: bool,
        /// Whether or not to play the break block particles.
        do_effect: bool,
        /// Whether or not to use the tool in the players hands.
        use_tool: bool,
        /// Plays the pickup-item animation from the origin.
        do_fake_looting: bool,
        /// An array of item materials, or droptables.
        items: ItemArray,
    },

    /// Clears the experience for the targeted players
    #[strum(to_string = "- clearexperience")]
    ClearExperience,

    /// Clears the experience levels for the targeted players
    #[strum(to_string = "- clearexperiencelevels")]
    ClearExperienceLevels,

    /// Gives experience levels to the targeted players
    #[strum(to_string = "- giveexperiencelevels{{a={amount}}}")]
    GiveExperienceLevels {
        /// The amount of levels to give.
        amount: u32,
    },

    /// Takes experience levels from the targeted players
    #[strum(to_string = "- takeexperiencelevels{{a={amount}}}")]
    TakeExperienceLevels {
        /// The amount of levels to take.
        amount: u32,
    },

    /// Closes the target player's inventory
    #[strum(to_string = "- closeinventory")]
    CloseInventory,

    /// Executes a command for each target
    #[strum(
        to_string = "- command{{c={command};ac={as_caster};op={as_op};at={as_target};rt={require_target}}}"
    )]
    Command {
        /// The command to execute.
        command: String,
        /// If true the command will execute from the caster instead of the console.
        as_caster: bool,
        /// Whether to execute the command with all permissions.
        as_op: bool,
        /// Will execute the command as the targeted entity.
        as_target: bool,
        /// Only executes if the skill has a target.
        require_target: bool,
    },

    /// Deals damage and restores health per target hit
    #[strum(to_string = "- consume{{d={damage};h={heal}}}")]
    Consume {
        /// The amount of damage to deal.
        damage: f32,
        /// The amount of healing per mob damaged.
        heal: f32,
    },

    /// Removes an item from a specific slot of the player's inventory
    #[strum(to_string = "- consumeslot{{s={slot};a={amount}}}")]
    ConsumeSlot {
        /// The inventory slot to remove the item from.
        slot: String,
        /// The amount of items to remove.
        amount: u32,
    },

    /// Changes the velocity on the target entity on a specific vector
    #[strum(to_string = "- directionalvelocity{{yaw={yaw};pitch={pitch};v={velocity};m={mode}}}")]
    DirectionalVelocity {
        /// The yaw of the vector for the velocity change.
        yaw: f32,
        /// The pitch of the vector for the velocity change.
        pitch: f32,
        /// The magnitude of the velocity change.
        velocity: f32,
        /// The mode to use.
        mode: String,
    },

    /// Causes the caster to leap backwards away from the target entity
    #[strum(to_string = "- disengage{{v={velocity};vy={velocity_y}}}")]
    Disengage {
        /// The velocity of the leap.
        velocity: f32,
        /// The y component of the velocity of the leap.
        velocity_y: f32,
    },

    /// Changes the caster's disguise
    #[strum(to_string = "- disguise{{d={disguise}}}")]
    Disguise {
        /// The disguise to apply to the mob.
        disguise: String,
    },

    /// Modifies the caster's already applied disguise
    #[strum(to_string = "- disguisemodify{{d={disguise}}}")]
    DisguiseModify {
        /// The options to modify in the disguise.
        disguise: String,
    },

    /// Changes the target's disguise
    #[strum(to_string = "- disguisetarget{{d={disguise}}}")]
    DisguiseTarget {
        /// The disguise to apply to the target.
        disguise: String,
    },

    /// Removes the caster's disguise
    #[strum(to_string = "- undisguise")]
    Undisguise,

    /// Makes the caster dismount whatever they're riding
    #[strum(to_string = "- dismount")]
    Dismount,

    /// Sets the targeted display entity's transformations
    #[strum(
        to_string = "- displaytransformation{{a={action};tt={transformation_type};val={value}}}"
    )]
    DisplayTransformation {
        /// The action to use.
        action: String,
        /// The type of the transformation.
        transformation_type: String,
        /// The value of the transformation.
        value: String,
    },

    /// Makes a mob clear its threat table
    #[strum(to_string = "- clearThreat")]
    ClearThreat,

    /// Gives money to a player. Requires Vault and a currency plugin
    #[strum(to_string = "- currencygive{{a={amount}}}")]
    CurrencyGive {
        /// The amount of money.
        amount: f64,
    },

    /// Takes money from a player. Requires Vault and a currency plugin
    #[strum(to_string = "- currencytake{{a={amount}}}")]
    CurrencyTake {
        /// The amount of money taken from player.
        amount: f64,
    },

    /// Damages the target for an amount
    #[strum(
        to_string = "- damage{{a={amount};ia={ignore_armor};pkb={prevent_knockback};pi={prevent_immunity};dc={damage_cause};ie={ignore_enchantments};na={no_anger};ii={ignore_invulnerability};is={ignore_shield};dh={damage_helmet};ieff={ignore_effects};ir={ignore_resistance};pad={power_affects_damage};tags={tags};rtag={raw_tags};e={element};ts={trigger_skills}}}"
    )]
    Damage {
        /// The amount of damage to deal.
        amount: f32,
        /// Whether or not to ignore armor.
        ignore_armor: bool,
        /// Whether or not to prevent knockback.
        prevent_knockback: bool,
        /// Whether or not to prevent the damage immunity ticks.
        prevent_immunity: bool,
        /// Sets the damage cause for this damage mechanic.
        damage_cause: String,
        /// Whether or not to ignore enchantments.
        ignore_enchantments: bool,
        /// Whether or not to generate anger when damaging the entity.
        no_anger: bool,
        /// Whether or not to ignore the damage immunity ticks.
        ignore_invulnerability: bool,
        /// Whether or not to ignore the shield blocking on the target.
        ignore_shield: bool,
        /// Whether or not the helmet should be damaged.
        damage_helmet: bool,
        /// Whether or not effects should be ignored.
        ignore_effects: bool,
        /// Whether or not resistance should be ignored.
        ignore_resistance: bool,
        /// Should the skill's power affect the damage inflicted.
        power_affects_damage: bool,
        /// Allows you to specify any number of arbitrary tags for the damage mechanic.
        tags: Tags,
        /// Works the same as tags and what is put here will also qualify as a tag, but it will not be UPPERCASED like tags.
        raw_tags: Tags,
        /// Becomes one of the Tags.
        element: String,
        /// Whether the damage mechanic should also be able to trigger onAttack related triggers.
        trigger_skills: bool,
    },

    /// Damages the target for a percent of the mob's damage stat
    #[strum(to_string = "- basedamage{{m={multiplier};attr={use_attribute}}}")]
    BaseDamage {
        /// The percentage of damage to deal.
        multiplier: f32,
        /// Whether the damage should use the real entity's attack attribute.
        use_attribute: bool,
    },

    /// Damages the target for a percent of their health
    #[strum(to_string = "- percentdamage{{p={percent};c={current_health}}}")]
    PercentDamage {
        /// The percentage to damage the target.
        percent: f32,
        /// Whether it calculates the percent from your original or current health.
        current_health: bool,
    },

    /// Drops a player head item based on target
    #[strum(to_string = "- decapitate")]
    Decapitate,

    /// Copies the appearance of the target player
    #[strum(to_string = "- doppleganger{{nameplate={has_nameplate};upn={use_player_name}}}")]
    Doppleganger {
        /// Whether the disguise should have a nameplate.
        has_nameplate: bool,
        /// Uses the player name as the nameplate.
        use_player_name: MythicOption<String>,
    },

    /// Drops an item or droptable at the target location
    #[strum(to_string = "- dropitem{{i={items};n={naturally};onDrop={on_drop_skill}}}")]
    DropItem {
        /// Items to drop.
        items: String,
        /// Whether the items should be dropped naturally.
        naturally: bool,
        /// Metaskill to be executed when the item drops.
        on_drop_skill: MythicOption<String>,
    },

    /// Ejects anything riding the caster
    #[strum(to_string = "- ejectpassenger")]
    EjectPassenger,

    /// Causes the "Ender" effect
    #[strum(to_string = "- ender")]
    Ender,

    /// Creates an EnderCrystal's beam effect to the target
    #[strum(to_string = "- effect:enderbeam{{d={duration};y={y_offset}}}")]
    EnderBeam {
        /// The time (in ticks) that the effect is active.
        duration: u32,
        /// The default vertical offset from the casting mob.
        y_offset: f32,
    },

    /// Generates the EnderDragon crystals
    #[strum(to_string = "- enderDragonResetCrystals")]
    EnderDragonResetCrystals,

    /// Sets the EnderDragon phase
    #[strum(to_string = "- enderDragonSetPhase{{p={phase}}}")]
    EnderDragonSetPhase {
        /// The phase to set.
        phase: String,
    },

    /// Sets the EnderDragon respawn phase
    #[strum(to_string = "- enderDragonSetRespawnPhase{{p={phase}}}")]
    EnderDragonSetRespawnPhase {
        /// The phase to set.
        phase: String,
    },

    /// Generates the portal of the EnderDragon battle
    #[strum(to_string = "- enderDragonSpawnPortal{{wp={with_portals}}}")]
    EnderDragonSpawnPortal {
        /// Whether to generate the portal of the EnderDragon battle.
        with_portals: bool,
    },

    /// Causes the casting mob to equip an item
    #[strum(to_string = "- equip{{item={item}}}")]
    Equip {
        /// The item config string to run on the mob.
        item: EquipmentItem,
    },

    /// Causes the caster to equip a copy of the target's equipment
    #[strum(to_string = "- equipcopy{{s={slots}}}")]
    EquipCopy {
        /// The slots to copy.
        slots: String,
    },

    /// Causes an explosion
    #[strum(to_string = "- explosion{{y={power_explosion};bd={block_damage};f={fire}}}")]
    Explosion {
        /// The yield (power) of the explosion.
        power_explosion: f32,
        /// Whether the explosion will damage blocks.
        block_damage: bool,
        /// Whether the explosion leaves fire behind.
        fire: bool,
    },

    /// Causes a fake explosion
    #[strum(to_string = "- fakeexplosion")]
    FakeExplosion,

    /// Removes fire ticks from the target entity
    #[strum(to_string = "- extinguish")]
    Extinguish,

    /// Pastes a Schematic using FAWE (Fast Async World Edit)
    #[strum(
        to_string = "- fawePaste{{s={schematic};pid={paste_id};a={paste_air};x={x_offset};y={y_offset};z={z_offset};rot={rotation};c={center};cdt={chest_drop_table};tcdt={trap_chest_drop_table};bpt={blocks_per_tick};d={duration}}}"
    )]
    FawePaste {
        /// Which schematic to load.
        schematic: String,
        /// The paste's id.
        paste_id: MythicOption<String>,
        /// Should air be pasted?
        paste_air: bool,
        /// The X offset of pasting the Schematic from the target.
        x_offset: i32,
        /// The Y offset of pasting the Schematic from the target.
        y_offset: i32,
        /// The Z offset of pasting the Schematic from the target.
        z_offset: i32,
        /// The rotation of the pasted schematic, in degrees.
        rotation: f32,
        /// Whether or not to center the schematic.
        center: bool,
        /// Which MythicMob Drop Tables to supply the chests within the Schematic with.
        chest_drop_table: MythicOption<String>,
        /// Which MythicMob Drop Tables to supply the Trapped Chests within the Schematic with.
        trap_chest_drop_table: MythicOption<String>,
        /// The number of blocks that are placed every tick.
        blocks_per_tick: u32,
        /// If greater than 0, will undo the paste operation after that amount of ticks has elapsed.
        duration: u32,
    },

    /// Feeds the target player
    #[strum(to_string = "- feed{{a={amount};s={saturation};o={overfeed}}}")]
    Feed {
        /// The amount of hunger to restore.
        amount: i32,
        /// The amount of saturation to restore.
        saturation: f32,
        /// Whether or not to overfeed.
        overfeed: bool,
    },

    /// Fills a chest with items, or a droptable
    #[strum(to_string = "- fillchest{{i={items};stack={should_stack};empty={should_empty}}}")]
    FillChest {
        /// Items to fill a chest with.
        items: String,
        /// Should the given items stack if possible.
        should_stack: MythicOption<bool>,
        /// Should the container be emptied before the items are added.
        should_empty: MythicOption<bool>,
    },

    /// Creates a firework effect at the target
    #[strum(
        to_string = "- effect:firework{{t={firework_type};p={power};f={flicker};tr={trail};c={colors};fc={fade_colors}}}"
    )]
    Firework {
        /// The type of firework.
        firework_type: String,
        /// The flight duration of the firework.
        power: u32,
        /// Whether to add the flicker effect to the explosion.
        flicker: bool,
        /// Whether to add the trail effect to the firework rocket.
        trail: bool,
        /// The color of the firework explosion, in RGB or hex.
        colors: String,
        /// The fade colors of the firework explosion, in RGB or hex.
        fade_colors: String,
    },

    /// Creates the flames effect at the location of the targeter
    #[strum(to_string = "- flames")]
    Flames,

    /// Applies an aura that allows the targeted player to fly
    #[strum(to_string = "- fly")]
    Fly,

    /// Teleports the target to the caster
    #[strum(to_string = "- forcepull{{s={spread};vs={v_spread}}}")]
    ForcePull {
        /// How spread out players will be from the casting mob.
        spread: u32,
        /// Lets you override the vertical spread value.
        v_spread: MythicOption<u32>,
    },

    /// Freezes the target for the given number of ticks using the Powdered Snow freezing effect
    #[strum(to_string = "- freeze{{t={ticks}}}")]
    Freeze {
        /// Ticks frozen in powdered snow.
        ticks: u32,
    },

    /// Creates a "geyser" of water or lava
    #[strum(to_string = "- geyser{{t={liquid_type};h={height};i={interval}}}")]
    Geyser {
        /// The type of liquid.
        liquid_type: String,
        /// How high the geyser will go.
        height: u32,
        /// The interval (in ticks) between each iteration of the geyser animation.
        interval: u32,
    },

    /// Gives an item to the target
    #[strum(to_string = "- giveitem{{i={item};fl={fake_looting}}}")]
    GiveItem {
        /// The item material.
        item: String,
        /// Plays the pickup-item animation from the origin.
        fake_looting: bool,
    },

    /// Gives an item to the target from the item in the given slot of caster
    #[strum(to_string = "- giveitemfromslot{{s={slot};fl={fake_looting}}}")]
    GiveItemFromSlot {
        /// The caster's slot.
        slot: String,
        /// Plays the pickup-item animation from the origin.
        fake_looting: bool,
    },

    /// Gives the caster an item while playing the pickup-item animation from the target entity or location
    #[strum(to_string = "- giveitemfromtarget{{i={item};fl={fake_looting}}}")]
    GiveItemFromTarget {
        /// The item material.
        item: String,
        /// Plays the pickup-item animation from the target.
        fake_looting: bool,
    },

    /// Makes the target glow
    #[strum(to_string = "- effect:glow{{color={color}}}")]
    Glow {
        /// The color with which the entity will glow.
        color: String,
    },

    /// Causes the casting goat mob to ram the targeted entity
    #[strum(to_string = "- goatram")]
    GoatRam,

    /// Move toward the location of the targeter (entity or location)
    #[strum(to_string = "- goto{{s={speed};sh={spread_h};sv={spread_v}}}")]
    GoTo {
        /// The movement speed modifier.
        speed: f32,
        /// Amount of horizontal spread it can be away from the target its moving towards.
        spread_h: u32,
        /// Amount of vertical spread it can be away from the target its moving towards.
        spread_v: u32,
    },

    /// Draws a guardian beam between the origin and the target
    #[strum(
        to_string = "- guardianbeam{{d={duration};i={interval};syo={start_y_offset};tyo={target_y_offset};fo={from_origin};oS={on_start_skill};oT={on_tick_skill};oE={on_end_skill}}}"
    )]
    GuardianBeam {
        /// The time (in ticks) for which the effect will be active.
        duration: u32,
        /// How often the effect will tick.
        interval: u32,
        /// The starting y offset of the beam.
        start_y_offset: f32,
        /// The target y offset of the beam.
        target_y_offset: f32,
        /// Whether to make the effect start from the @origin instead of from @self.
        from_origin: bool,
        /// Metaskill to execute when the effect starts.
        on_start_skill: MythicOption<String>,
        /// Metaskill to execute each interval tick.
        on_tick_skill: MythicOption<String>,
        /// Metaskill to execute when the effect ends.
        on_end_skill: MythicOption<String>,
    },

    /// Heals the target
    #[strum(to_string = "- heal{{a={amount};oh={overheal};mo={max_overheal}}}")]
    Heal {
        /// The amount to heal the target.
        amount: f32,
        /// Whether or not to apply overhealing as additional MaxHealth.
        overheal: bool,
        /// The maximum amount of overhealing that can be applied.
        max_overheal: f32,
    },

    /// Heals the target for a percentage of its max-health
    #[strum(to_string = "- healpercent{{m={multiplier};oh={overheal};mo={max_overheal}}}")]
    HealPercent {
        /// The percentage to heal, refers to the targets max-health.
        multiplier: Percentage,
        /// Whether or not to apply overhealing as additional MaxHealth.
        overheal: bool,
        /// The maximum amount of overhealing that can be applied.
        max_overheal: f32,
    },

    /// Hides the caster from the targeted player(s) for a set duration
    #[strum(to_string = "- hide{{ignoreAuraOptions={ignore_aura_options}}}")]
    Hide {
        /// This will make the mechanic ignore any aura-related option and the `duration` attribute.
        ignore_aura_options: bool,
    },

    /// Simulates a physical hit from the mob
    #[strum(
        to_string = "- hit{{m={multiplier};fd={forced_damage};ts={trigger_skills};sbac={scale_by_attack_cooldown}}}"
    )]
    Hit {
        /// The percentage of damage to deal.
        multiplier: f32,
        /// If this attribute is set, the one specified will be the amount of flat damage that will be inflicted.
        forced_damage: MythicOption<f32>,
        /// Whether the damage mechanic should also be able to trigger `onAttack` related triggers.
        trigger_skills: bool,
        /// Whether to scale the damage by the weapon's attack cooldown.
        scale_by_attack_cooldown: bool,
    },

    /// Summons a hologram to the targeted location
    #[strum(to_string = "- holo{{text={text};time={stay}}}")]
    Hologram {
        /// The text to show.
        text: String,
        /// The duration of the hologram in ticks.
        stay: u32,
    },

    /// Sets the target on fire
    #[strum(to_string = "- ignite{{t={ticks}}}")]
    Ignite {
        /// How many ticks the target should burn.
        ticks: u32,
    },

    /// Causes an explosion of temporary items at the target location
    #[strum(
        to_string = "- itemspray{{i={items};a={amount};d={duration};r={radius};v={velocity};yv={y_velocity};yo={y_offset};ap={allow_pickup};g={gravity}}}"
    )]
    ItemSpray {
        /// The list of items to drop.
        items: String,
        /// How many items will render from the spray.
        amount: u32,
        /// How long (in ticks) the items will exist.
        duration: u32,
        /// The radius/spread the items will start in.
        radius: u32,
        /// The velocity of the items.
        velocity: f32,
        /// The Y velocity of the items.
        y_velocity: MythicOption<f32>,
        /// The y offset the items will start at.
        y_offset: f32,
        /// Whether the itemspray's items should be real items, enabling players to pick them up.
        allow_pickup: bool,
        /// Whether the items should be affected by gravity.
        gravity: bool,
    },

    /// Sends a JSON-format message to the target player(s)
    #[strum(to_string = "- jsonmessage{{m={message}}}")]
    JSONMessage {
        /// The json-message to send.
        message: String,
    },

    /// Causes the caster to jump
    #[strum(to_string = "- jump{{v={velocity}}}")]
    Jump {
        /// The velocity of the mob's jump.
        velocity: f32,
    },

    /// Causes the caster to leap towards the target
    #[strum(to_string = "- leap{{v={velocity};n={noise}}}")]
    Leap {
        /// The max velocity of the leap.
        velocity: f32,
        /// Added variance to where the mob will land.
        noise: f32,
    },

    /// Strikes lightning at the target
    #[strum(to_string = "- lightning{{damage={damage}}}")]
    Lightning {
        /// The amount of damage the strike will deal
        damage: f32,
    },

    /// Strikes a fake lightning at the target
    #[strum(
        to_string = "- fakelightning{{localized={localized};localizedradius={localized_radius}}}"
    )]
    FakeLightning {
        /// Whether the lightning should only be seen/heard by players in radius
        localized: bool,
        /// The radius of the localized effect
        localized_radius: u32,
    },

    /// Logs a message to console
    #[strum(to_string = "- log{{message={message}}}")]
    Log {
        /// The message to log
        message: String,
    },

    /// Causes the caster to look at the target
    #[strum(
        to_string = "- look{{headOnly={head_only};force={force};forcepaper={force_paper};immediately={immediately}}}"
    )]
    Look {
        /// Only the mob's head is facing the target
        head_only: bool,
        /// Forces the mob to look at the target (even works with no AI)
        force: bool,
        /// Whether to use Paper's method to force the mob to look at the target
        force_paper: bool,
        /// Immediately causes the mob to turn towards the target with no turning animation
        immediately: bool,
    },

    /// Causes the caster to lunge forward at the target
    #[strum(to_string = "- lunge{{velocity={velocity};velocityY={velocity_y};oldmath={old_math}}}")]
    Lunge {
        /// The horizontal velocity at which the entity is moved forward
        velocity: f32,
        /// The vertical velocity at which the entity is moved forward
        velocity_y: f32,
        /// If the lunge mechanic should use the old math formula
        old_math: bool,
    },

    /// Sets the caster's yaw and pitch to the same value of the target's
    #[strum(to_string = "- matchrotation{{target={target}}}")]
    MatchRotation {
        /// The targeter
        target: String,
    },

    /// Sends a message to the target player(s)
    #[strum(to_string = "- message{{message={message};audience={audience}}}")]
    Message {
        /// The message to send
        message: String,
        /// The audience of the message
        audience: String,
    },

    /// Modifies the damage event that triggered the skill
    #[strum(
        to_string = "- modifyDamage{{amount={amount};damagetype={damage_type};action={action}}}"
    )]
    ModifyDamage {
        /// The amount of the operation
        amount: f32,
        /// The type of the damage to evaluate
        damage_type: String,
        /// The modifier to use
        action: String,
    },

    /// Modifies a scoreboard value of the fake player: __GLOBAL__
    #[strum(
        to_string = "- modifyglobalscore{{objective={objective};action={action};value={value}}}"
    )]
    ModifyGlobalScore {
        /// Specifies the scoreboard objective to be changed
        objective: String,
        /// The operation to perform
        action: String,
        /// The value to perform the operation with
        value: f32,
    },

    /// Modifies a scoreboard value of the target
    #[strum(
        to_string = "- modifytargetscore{{objective={objective};action={action};value={value}}}"
    )]
    ModifyTargetScore {
        /// Specifies the scoreboard objective to be changed
        objective: String,
        /// The operation to perform
        action: String,
        /// The value to perform the operation with
        value: f32,
    },

    /// Modifies a scoreboard value of the casting mob
    #[strum(to_string = "- modifymobscore{{objective={objective};action={action};value={value}}}")]
    ModifyMobScore {
        /// Specifies the scoreboard objective to be changed
        objective: String,
        /// The operation to perform
        action: String,
        /// The value to perform the operation with
        value: f32,
    },

    /// Modifies the score of a dummy player
    #[strum(to_string = "- modifyscore{{objective={objective};action={action};value={value}}}")]
    ModifyScore {
        /// Specifies the scoreboard objective to be changed
        objective: String,
        /// The operation to perform
        action: String,
        /// The value to perform the operation with
        value: f32,
    },

    /// Summons a mob for the caster and mounts it
    #[strum(to_string = "- mount{{entity={entity}}}")]
    Mount {
        /// The entity to mount
        entity: String,
    },

    /// Forces the targeted entity to mount the caster
    #[strum(to_string = "- mountme{{entity={entity}}}")]
    MountMe {
        /// The entity to mount
        entity: String,
    },

    /// Mounts the target
    #[strum(to_string = "- mounttarget{{entity={entity}}}")]
    MountTarget {
        /// The entity to mount
        entity: String,
    },

    /// Moves the given pin to the target location
    #[strum(to_string = "- movepin{{x={x};y={y};z={z}}}")]
    MovePin {
        /// The x-coordinate to move the pin to
        x: f32,
        /// The y-coordinate to move the pin to
        y: f32,
        /// The z-coordinate to move the pin to
        z: f32,
    },

    /// Opens the trades of the casting villager to the target player
    #[strum(to_string = "- opentrades{{}}")]
    OpenTrades,

    /// Gives oxygen to a player target
    #[strum(to_string = "- oxygen{{amount={amount}}}")]
    Oxygen {
        /// The amount of oxygen to set
        amount: i32,
    },

    /// Creates particle effects around the target
    #[strum(
        to_string = "- particle{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z}}}"
    )]
    Particle {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
    },

    /// Draws a box of particles around the target
    #[strum(
        to_string = "- particlebox{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};width={width};height={height}}}"
    )]
    ParticleBox {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The width of the box
        width: f32,
        /// The height of the box
        height: f32,
    },

    /// Generates particles based on equations
    #[strum(
        to_string = "- particleequation{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};equation={equation}}}"
    )]
    ParticleEquation {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The equation to use for particle distribution
        equation: String,
    },

    /// Draws a line of particle effects to the target
    #[strum(
        to_string = "- particleline{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};length={length}}}"
    )]
    ParticleLine {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The length of the line
        length: f32,
    },

    /// Draws a line based helix effect
    #[strum(
        to_string = "- particlelinehelix{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};length={length};radius={radius}}}"
    )]
    ParticleLineHelix {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The length of the helix
        length: f32,
        /// The radius of the helix
        radius: f32,
    },

    /// Draws a particle ring connected by lines
    #[strum(
        to_string = "- particlelinering{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};radius={radius}}}"
    )]
    ParticleLineRing {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The radius of the ring
        radius: f32,
    },

    /// Draws orbiting particle effects around the target
    #[strum(
        to_string = "- particleorbital{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};radius={radius}}}"
    )]
    ParticleOrbital {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The radius of the orbital
        radius: f32,
    },

    /// Draws a ring of particles around the target
    #[strum(
        to_string = "- particlering{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};radius={radius}}}"
    )]
    ParticleRing {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The radius of the ring
        radius: f32,
    },

    /// Draws a sphere of particles around the target
    #[strum(
        to_string = "- particlesphere{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};radius={radius}}}"
    )]
    ParticleSphere {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The radius of the sphere
        radius: f32,
    },

    /// Draws a persistent "tornado" of particles at the target
    #[strum(
        to_string = "- particletornado{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};height={height};radius={radius}}}"
    )]
    ParticleTornado {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The height of the tornado
        height: f32,
        /// The radius of the tornado
        radius: f32,
    },

    /// Creates some particles in the shape of an atom
    #[strum(
        to_string = "- atom{{particle={particle};amount={amount};speed={speed};x={x};y={y};z={z};radius={radius}}}"
    )]
    Atom {
        /// The type of particle to spawn
        particle: String,
        /// The amount of particles to spawn
        amount: u32,
        /// The speed of the particles
        speed: f32,
        /// The x-offset of the particles
        x: f32,
        /// The y-offset of the particles
        y: f32,
        /// The z-offset of the particles
        z: f32,
        /// The radius of the atom effect
        radius: f32,
    },

    /// Picks up the targeted item
    #[strum(to_string = "- pickupitem{{item={item}}}")]
    PickUpItem {
        /// The item to pick up
        item: String,
    },

    /// Forces the entity to play an animation
    #[strum(to_string = "- playanimation{{animation={animation}}}")]
    PlayAnimation {
        /// The animation to play
        animation: String,
    },

    /// Plays a block breaking sound
    #[strum(to_string = "- playblockbreaksound{{block={block}}}")]
    PlayBlockBreakSound {
        /// The block type to play the break sound for
        block: String,
    },

    /// Plays a block falling sound
    #[strum(to_string = "- playblockfallsound{{block={block}}}")]
    PlayBlockFallSound {
        /// The block type to play the fall sound for
        block: String,
    },

    /// Plays a block hit sound
    #[strum(to_string = "- playblockhitsound{{block={block}}}")]
    PlayBlockHitSound {
        /// The block type to play the hit sound for
        block: String,
    },

    /// Plays a block place sound
    #[strum(to_string = "- playblockplacesound{{block={block}}}")]
    PlayBlockPlaceSound {
        /// The block type to play the place sound for
        block: String,
    },

    /// Plays a block step sound
    #[strum(to_string = "- playblockstepsound{{block={block}}}")]
    PlayBlockStepSound {
        /// The block type to play the step sound for
        block: String,
    },

    /// Changes the pose of the target ArmorStand
    #[strum(to_string = "- posearmorstand{{pose={pose}}}")]
    PoseArmorStand {
        /// The pose to set for the armor stand
        pose: String,
    },

    /// Applies a potion effect to the target
    #[strum(to_string = "- potion{{potion={potion};duration={duration};level={level}}}")]
    Potion {
        /// The type of potion effect to apply
        potion: String,
        /// The duration of the potion effect in seconds
        duration: u32,
        /// The level of the potion effect
        level: u8,
    },

    /// Removes all potion effects from target entity
    #[strum(to_string = "- potionclear{{}}")]
    PotionClear,

    /// Imprisons the target inside a block
    #[strum(to_string = "- prison{{}}")]
    Prison,

    /// Prints debug information regarding the Metaskill executing the mechanic and its SkillTree
    #[strum(to_string = "- printparenttree{{}}")]
    PrintParentTree,

    /// Propels the caster towards the target
    #[strum(to_string = "- propel{{velocity={velocity}}}")]
    Propel {
        /// The velocity at which to propel the entity
        velocity: f32,
    },

    /// Pulls the target towards the mob
    #[strum(to_string = "- pull{{velocity={velocity}}}")]
    Pull {
        /// The velocity at which to pull the entity
        velocity: f32,
    },

    /// Pushes the block at the target location in the given direction
    #[strum(to_string = "- pushblock{{velocity={velocity}}}")]
    PushBlock {
        /// The velocity at which to push the block
        velocity: f32,
    },

    /// Pushes a button at the target location
    #[strum(to_string = "- pushbutton{{}}")]
    PushButton,

    /// Traces a straight line to the target
    #[strum(to_string = "- raytrace{{}}")]
    RayTrace,

    /// Executes a skill with the result of a raytrace to the target location
    #[strum(to_string = "- raytraceto{{}}")]
    RayTraceTo,

    /// Causes other nearby mobs to attack the target
    #[strum(to_string = "- rally{{radius={radius}}}")]
    Rally {
        /// The radius within which to rally entities
        radius: f32,
    },

    /// Sends a random message to the target player
    #[strum(to_string = "- randommessage{{messages={messages}}}")]
    RandomMessage {
        /// The list of possible messages to send
        messages: RandomMessages,
    },

    /// Kicks the target's screen in order to simulate a recoil
    #[strum(to_string = "- recoil{{velocity={velocity}}}")]
    Recoil {
        /// The velocity at which to recoil
        velocity: f32,
    },

    /// Remounts the mob the caster originally spawned riding, if it is still alive
    #[strum(to_string = "- remount{{}}")]
    Remount,

    /// Removes the target mob
    #[strum(to_string = "- remove{{}}")]
    Remove,

    /// Removes some of the item the target player is holding
    #[strum(to_string = "- removehelditem{{}}")]
    RemoveHeldItem,

    /// Removes the ownership of the target mob
    #[strum(to_string = "- removeowner{{}}")]
    RemoveOwner,

    /// Attempts to reset the AI of a casting mob to the base type's default
    #[strum(to_string = "- resetai{{}}")]
    ResetAI,

    /// Rotates the caster towards the target location
    #[strum(to_string = "- rotatetowards{{target={target}}}")]
    RotateTowards {
        /// The target to rotate towards
        target: String,
    },

    /// Change the caster's AIGoalSelectors
    #[strum(to_string = "- runaigoalselector{{}}")]
    RunAIGoalSelector,

    /// Change the caster's AITargetSelectors
    #[strum(to_string = "- runaitargetselector{{}}")]
    RunAITargetSelector,

    /// Equips or removes a saddle on the target entity
    #[strum(to_string = "- saddle{{}}")]
    Saddle,

    /// Sends an Actionbar Message to the target player
    #[strum(to_string = "- sendactionmessage{{message={message}}}")]
    SendActionMessage {
        /// The action message to send
        message: String,
    },

    /// Sends a Resource Pack to the target player
    #[strum(to_string = "- sendresourcepack{{url={url}}}")]
    SendResourcePack {
        /// The URL of the resource pack to send
        url: String,
    },

    /// Sends a Title/Subtitle Message to the target player
    #[strum(
        to_string = "- sendtitle{{title={title};subtitle={subtitle};fadein={fade_in};stay={stay};fadeout={fade_out}}}"
    )]
    SendTitle {
        /// The title to send
        title: String,
        /// The subtitle to send
        subtitle: String,
        /// The fade-in time for the title
        fade_in: u32,
        /// The time the title stays on screen
        stay: u32,
        /// The fade-out time for the title
        fade_out: u32,
    },

    /// Sends an achievement toast to the target player
    #[strum(to_string = "- sendtoast{{title={title};message={message}}}")]
    SendToast {
        /// The title of the toast
        title: String,
        /// The message of the toast
        message: String,
    },

    /// Disables/enables the AI of the target mob
    #[strum(to_string = "- setai{{ai={ai}}}")]
    SetAI {
        /// The AI to set
        ai: String,
    },

    /// Sets the target block's open state
    #[strum(to_string = "- setblockopen{{open={open}}}")]
    SetBlockOpen {
        /// Whether to set the block as open
        open: bool,
    },

    /// Change block type at target location
    #[strum(to_string = "- setblocktype{{block={block}}}")]
    SetBlockType {
        /// The type of block to set
        block: String,
    },

    /// Sets the force-loaded status of a location's chunk
    #[strum(to_string = "- setchunkforceloaded{{loaded={loaded}}}")]
    SetChunkForceLoaded {
        /// Whether to force load the chunk
        loaded: bool,
    },

    /// Sets if the target should have a collidable hitbox or not
    #[strum(to_string = "- setcollidable{{collidable={collidable}}}")]
    SetCollidable {
        /// Whether the entity should be collidable
        collidable: bool,
    },

    /// Sets the position of the dragon's podium at the target location
    #[strum(to_string = "- setdragonpodium{{podium={podium}}}")]
    SetDragonPodium {
        /// Whether to set the dragon podium
        podium: bool,
    },

    /// Sets the Game Mode of the target player
    #[strum(to_string = "- setgamemode{{gamemode={gamemode}}}")]
    SetGameMode {
        /// The gamemode to set
        gamemode: String,
    },

    /// Makes the target glide if they have elytra
    #[strum(to_string = "- setgliding{{gliding={gliding}}}")]
    SetGliding {
        /// Whether the entity should be gliding
        gliding: bool,
    },

    /// Sets a scoreboard value on the fake player: __GLOBAL__
    #[strum(to_string = "- setglobalscore{{objective={objective};action={action};value={value}}}")]
    SetGlobalScore {
        /// Specifies the scoreboard objective to be changed
        objective: String,
        /// The operation to perform
        action: String,
        /// The value to perform the operation with
        value: f32,
    },

    /// Sets whether gravity affects the target entity
    #[strum(to_string = "- setgravity{{gravity={gravity}}}")]
    SetGravity {
        /// The gravity to set
        gravity: f32,
    },

    /// Sets the health of the target entity
    #[strum(to_string = "- sethealth{{health={health}}}")]
    SetHealth {
        /// The health to set
        health: f32,
    },

    /// Sets the size of the target `INTERACTION` entity
    #[strum(to_string = "- setinteractionsize{{width={width};height={height}}}")]
    SetInteractionSize {
        /// The width of the interaction size
        width: f32,
        /// The height of the interaction size
        height: f32,
    },

    /// Sets the cooldown on an item group for the target player
    #[strum(to_string = "- setitemgroupcooldown{{group={group};cooldown={cooldown}}}")]
    SetItemGroupCooldown {
        /// The item group to set the cooldown for
        group: String,
        /// The cooldown time to set
        cooldown: u32,
    },

    /// Sets the item component of `ITEM_DISPLAY` entities
    #[strum(to_string = "- setdisplayentityitem{{item={item}}}")]
    SetDisplayEntityItem {
        /// The item to display
        item: String,
    },

    /// Changes the holder of a mob's lead
    #[strum(to_string = "- setleashholder{{holder={holder}}}")]
    SetLeashHolder {
        /// The entity to set as the leash holder
        holder: String,
    },

    /// Changes the casting mob's level
    #[strum(to_string = "- setlevel{{level={level}}}")]
    SetLevel {
        /// The level to set
        level: u32,
    },

    /// Sets a cooldown for usable materials like ender pearls, chorus fruit, etc
    #[strum(to_string = "- setmaterialcooldown{{material={material};cooldown={cooldown}}}")]
    SetMaterialCooldown {
        /// The material to set the cooldown for
        material: String,
        /// The cooldown time to set
        cooldown: u32,
    },

    /// Sets the max health of the target entity
    #[strum(to_string = "- setmaxhealth{{health={health}}}")]
    SetMaxHealth {
        /// The maximum health to set
        health: f32,
    },

    /// Changes the color of the target if it is a colorable mob
    #[strum(to_string = "- setmobcolor{{color={color}}}")]
    SetMobColor {
        /// The color to set for the mob
        color: String,
    },

    /// Sets a scoreboard value on the casting mob
    #[strum(to_string = "- setmobscore{{objective={objective};score={score}}}")]
    SetMobScore {
        /// The objective to set the score for
        objective: String,
        /// The score to set
        score: i32,
    },

    /// Changes the caster entity's name
    #[strum(to_string = "- setname{{name={name}}}")]
    SetName {
        /// The name to set
        name: String,
    },

    /// Sets if the target raider entity can join a raid or not
    #[strum(to_string = "- setraidercanjoinraid{{can_join={can_join}}}")]
    SetRaiderCanJoinRaid {
        /// Whether the raider can join the raid
        can_join: bool,
    },

    /// Sets the target raider to patrol a location
    #[strum(to_string = "- setraiderpatrolblock{{block={block}}}")]
    SetRaiderPatrolBlock {
        /// The block to set as the patrol block
        block: String,
    },

    /// Sets the raider patrol leader
    #[strum(to_string = "- setraiderpatrolleader{{leader={leader}}}")]
    SetRaiderPatrolLeader {
        /// The entity to set as the patrol leader
        leader: String,
    },

    /// Changes the target entity's faction
    #[strum(to_string = "- setfaction{{faction={faction}}}")]
    SetFaction {
        /// The faction to set
        faction: String,
    },

    /// Sets whether the target player is flying
    #[strum(to_string = "- setflying{{flying={flying}}}")]
    SetFlying {
        /// Whether the entity should be flying
        flying: bool,
    },

    /// Sets the no damage ticks of the target
    #[strum(to_string = "- setnodamageticks{{ticks={ticks}}}")]
    SetNoDamageTicks {
        /// The number of no damage ticks to set
        ticks: u32,
    },

    /// Makes the target the owner of the casting mob
    #[strum(to_string = "- setowner{{owner={owner}}}")]
    SetOwner {
        /// The owner to set
        owner: String,
    },

    /// Makes the target the parent of the casting mob
    #[strum(to_string = "- setparent{{parent={parent}}}")]
    SetParent {
        /// The parent to set
        parent: String,
    },

    /// Sets the pathfinding malus of a mob for given terrain types
    #[strum(to_string = "- setpathfindingmalus{{malus={malus}}}")]
    SetPathfindingMalus {
        /// The pathfinding malus to set
        malus: f32,
    },

    /// Sets the head pitch of the target entity
    #[strum(to_string = "- setpitch{{pitch={pitch}}}")]
    SetPitch {
        /// The pitch to set
        pitch: f32,
    },

    /// Sets the entity's pose
    #[strum(to_string = "- setpose{{pose={pose}}}")]
    SetPose {
        /// The pose to set
        pose: String,
    },

    /// Sets the rotation of the target
    #[strum(to_string = "- setrotation{{yaw={yaw};pitch={pitch}}}")]
    SetRotation {
        /// The yaw to set
        yaw: f32,
        /// The pitch to set
        pitch: f32,
    },

    /// Sets the caster's target
    #[strum(to_string = "- settarget{{target={target}}}")]
    SetTarget {
        /// The target to set
        target: String,
    },

    /// Sets the score of the target
    #[strum(to_string = "- settargetscore{{objective={objective};score={score}}}")]
    SetTargetScore {
        /// The objective to set the score for
        objective: String,
        /// The score to set
        score: i32,
    },

    /// Sets the text component of target Text Display entity
    #[strum(to_string = "- settextdisplay{{text={text}}}")]
    SetTextDisplay {
        /// The text to display
        text: String,
    },

    /// Sets the tongue target for a frog caster to the target entity
    #[strum(to_string = "- settonguetarget{{target={target}}}")]
    SetTongueTarget {
        /// The target to set for the tongue
        target: String,
    },

    /// Sets the scoreboard value of a dummy player
    #[strum(to_string = "- setscore{{objective={objective};score={score}}}")]
    SetScore {
        /// The objective to set the score for
        objective: String,
        /// The score to set
        score: i32,
    },

    /// Sets the target entity's speed attribute
    #[strum(to_string = "- setspeed{{speed={speed}}}")]
    SetSpeed {
        /// The speed to set
        speed: f32,
    },

    /// Sets the stance of the target mob
    #[strum(to_string = "- setstance{{stance={stance}}}")]
    SetStance {
        /// The stance to set
        stance: String,
    },

    /// Applies an absorb shield to the target entity
    #[strum(to_string = "- shield{{}}")]
    Shield,

    /// Forces the player to lower their shield and puts it on cooldown
    #[strum(to_string = "- shieldbreak{{}}")]
    ShieldBreak,

    /// Applies an absorb shield to the target entity for a percentage of their max health
    #[strum(to_string = "- shieldpercent{{percent={percent}}}")]
    ShieldPercent {
        /// The percentage of the shield to set
        percent: f32,
    },

    /// Shoots a fireball at the target
    #[strum(to_string = "- shootfireball{{velocity={velocity}}}")]
    ShootFireball {
        /// The velocity at which to shoot the fireball
        velocity: f32,
    },

    /// Throws a potion at the target
    #[strum(to_string = "- shootpotion{{potion={potion};velocity={velocity}}}")]
    ShootPotion {
        /// The type of potion to shoot
        potion: String,
        /// The velocity at which to shoot the potion
        velocity: f32,
    },

    /// Shoots a wither skull at the target
    #[strum(to_string = "- shootskull{{velocity={velocity}}}")]
    ShootSkull {
        /// The velocity at which to shoot the skull
        velocity: f32,
    },

    /// Shoots a shulker bullet at the target entity
    #[strum(to_string = "- shootshulkerbullet{{velocity={velocity}}}")]
    ShootShulkerBullet {
        /// The velocity at which to shoot the shulker bullet
        velocity: f32,
    },

    /// Shows the hidden caster to the targeted players
    #[strum(to_string = "- showentity{{entity={entity}}}")]
    ShowEntity {
        /// The entity to show
        entity: String,
    },

    /// Sends a signal to a mob
    #[strum(to_string = "- signal{{signal={signal}}}")]
    Signal {
        /// The signal to send
        signal: String,
    },

    /// Alters the target player's skybox
    #[strum(to_string = "- skybox{{skybox={skybox}}}")]
    Skybox {
        /// The skybox to set
        skybox: String,
    },

    /// Creates a puff of smoke
    #[strum(to_string = "- smoke{{}}")]
    Smoke,

    /// Creates a persistent "swirl" of smoke
    #[strum(to_string = "- smokeswirl{{}}")]
    SmokeSwirl,

    /// Plays a sound effect
    #[strum(to_string = "- sound{{sound={sound};volume={volume};pitch={pitch}}}")]
    Sound {
        /// The sound to play
        sound: String,
        /// The volume at which to play the sound
        volume: f32,
        /// The pitch at which to play the sound
        pitch: f32,
    },

    /// Steals an item from the target player's inventory
    #[strum(to_string = "- stealitem{{item={item}}}")]
    StealItem {
        /// The item to steal from the target player's inventory.
        item: String,
    },

    /// Stops a sound effect from playing
    #[strum(to_string = "- stopsound{{s={sound}}}")]
    StopSound { sound: String },
    #[strum(to_string = "- stopsound{{s={sound};source={sound_category}}}")]
    StopSoundWithCategory {
        sound: String,
        sound_category: SoundCategory,
    },

    /// Causes the mob to speak in chat, with options for speech bubbles
    #[strum(to_string = "- speak{{}}")]
    Speak {
        /// The y offset for the hologram.
        offset: f32,
        /// The radius of entities which will see the chat message
        radius: u8,
        /// The maximum length of the hologram
        max_line_lenght: u8,
        /// The prefix for the hologram.
        line_prefix: String,
        /// The message to be displayed (affects both hologram and chat)
        message: String,
        /// The prefix for the chat message
        chat_prefix: String,
        /// The amount of time the hologram will be displayed for.
        duration: u16,
        /// Whether the message shows up in chat
        send_chat_message: bool,
        // The [Audience](https://git.mythiccraft.io/mythiccraft/MythicMobs/-/wikis/Skills/Audience) of the mechanic
        //audience: todo!()
    },

    /// Causes the target to spin
    #[strum(to_string = "- spin{{velocity={velocity};{aura}}}")]
    Spin {
        ///The velocity the target spins at
        velocity: u8,
        aura: AuraMechanic,
    },

    #[strum(to_string = "- spring{{}}")]
    /// Creates a temporary spring of liquid at the target
    Spring {
        /// The type of spring. Can be water or lava
        spring_type: SpringType,
        /// The duration (in ticks) the spring will last
        duration: u32,
    },

    /// Stuns the target entity for a specified duration
    #[strum(to_string = "- stun{{duration={duration}}}")]
    Stun {
        /// The duration for which the target entity is stunned.
        duration: u32,
    },

    /// Stops the targeted entity from using an item
    #[strum(to_string = "- stopusingitem")]
    StopUsingItem,

    /// Causes the caster to die
    #[strum(to_string = "- suicide")]
    Suicide,

    /// Summons a mob at the specified location
    #[strum(to_string = "- summon{{mob={mob};location={location}}}")]
    Summon {
        /// The type of mob to summon.
        mob: String,
        /// The location where the mob should be summoned.
        location: String,
    },

    /// Summons a cloud of particles at the target
    #[strum(
        to_string = "- summonareaeffectcloud{{{particle}{effect_type}{potion_duration}{level}{duration}{duration_reduction_on_use}{radius}{radius_reduction_on_use}{radius_reduction_on_tick}}}"
    )]
    SummonAreaEffectCloud {
        /// The particle effects to use
        particle: MythicOption<Particle>,
        /// The type of the effect given by the cloud
        effect_type: MythicOption<EffectType>,
        /// The duration of the potion effect
        potion_duration: MythicOption<u32>,
        /// The level of the potion effect
        level: MythicOption<u8>,
        /// The duration of the particle cloud
        duration: MythicOption<u32>,
        /// The duration reduction for the cloud on use
        duration_reduction_on_use: MythicOption<DurationReduction>,
        /// The radius of the cloud
        radius: MythicOption<u8>,
        /// The radius reduction for the cloud on use
        radius_reduction_on_use: MythicOption<RadiusReductionOnUse>,
        ///The radius reduction for the cloud per tick
        radius_reduction_on_tick: MythicOption<RadiusReductionOnTick>,
    },

    /// Summons a falling block
    #[strum(to_string = "- summonfallingblock{{m={material}}}")]
    SummonFallingBlock {
        ///The [material](https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/Material.html) of the falling block
        material: String,
    },

    #[strum(to_string = "- summonpassenger{{type={passenger}{stack}}}")]
    /// Summons a mob to ride the target.
    SummonPassenger {
        /// The type of the mob to set as the passenger
        passenger: String,
        /// Sets whether to mount the summoned entity to the current passenger of the caster
        stack: MythicOption<bool>,
    },

    #[strum(to_string = "- swap")]
    /// Swaps locations with the target
    Swap,

    #[strum(to_string = "- swingoffhand")]
    /// Makes the casting player swing their offhand
    SwingOffHand,

    #[strum(to_string = "- addtag{{t={0}}}")]
    /// Adds a scoreboard tag to the target
    AddTag(String),

    #[strum(to_string = "- removetag{{t={0}}}")]
    /// Removes a scoreboard tag from the target
    RemoveTag(String),

    #[strum(to_string = "- takeitem{{i={item};a={amount}{exact}{vanilla_only}}}")]
    /// Removes an item from the targeted player's inventory
    TakeItem {
        /// The item, or material, to remove
        item: String,
        /// The amount to remove
        amount: u8,
        /// Whether the name of the item should match exactly to the specified one
        exact: MythicOption<bool>,
        /// Whether the matched item can only be a vanilla one
        vanilla_only: MythicOption<bool>,
    },

    #[strum(to_string = "- taunt")]
    /// Modifies the threat level that the caster holds with the target entities
    Taunt,

    #[strum(to_string = "- teleport{{}}")]
    /// Teleports to the target
    Teleport {
        /// The horizontal spread of the landing location.
        spreadh: MythicOption<u8>,
        /// The vertical spread of the landing location.
        spreadv: MythicOption<u8>,
        /// Whether the pitch value should be carried over
        preserve_pitch: MythicOption<bool>,
        /// Whether the yaw value should be carried over
        preserve_yaw: MythicOption<bool>,
        /// Avoids finding a safe teleport (will ignore sH and sV)
        safe_teleport: MythicOption<bool>,
    },

    #[strum(to_string = "- teleport{{y={y}}}")]
    /// Teleports the caster vertically
    TeleportY {
        /// Where to teleport on the Y axis
        y: u32,
    },

    #[strum(to_string = "- teleportin{{{vector}{yaw}}}")]
    /// Teleports the target relative to the caster's yaw
    TeleportIn {
        /// The direction to where the mob will be teleported
        vector: MythicOption<Xyz>,
        /// Yaw modifier
        yaw: MythicOption<u32>,
        /// Will use the target's location as the origin instead of the caster's
        target_as_origin: MythicOption<bool>,
    },

    #[strum(
        to_string = "- teleportto{{loc={location};w={world}{yaw}{pitch}{relative}{target_as_origin}}}"
    )]
    /// Teleports the target to a specified location
    TeleportTo {
        /// The coordinates of the teleport's destination, or a targeter
        location: Xyz,
        /// The destination-world. Optional attribute if "location" is given
        world: String,
        /// The yaw that the affected entities should assume
        yaw: MythicOption<u8>,
        /// The pitch that the affected entities should assume
        pitch: MythicOption<u8>,
        /// Whether the location is relative or directional
        relative: MythicOption<bool>,
        /// Will use the target's location as the origin instead of the caster
        target_as_origin: MythicOption<bool>,
    },

    #[strum(to_string = "- time{{mode={mode};amount={amount}{personal}{relative}}}")]
    /// Changes the time
    Time {
        /// The mode used in the time mechanic. Can be ADD/SET/RESET
        mode: AddSetReset,
        /// The amount of ticks by which the time will be changed
        amount: u32,
        /// Sets whether to change the global time or the player's client time
        personal: MythicOption<bool>,
        /// Sets whether to keep the player's time synchronized to its world time with an offset
        relative: MythicOption<bool>,
    },

    #[strum(to_string = "- threat{{amount={amount}{mode}}}")]
    /// Modifies the mob's threat towards the target
    Threat {
        amount: i32,
        mode: MythicOption<ThreatMode>,
    },

    #[strum(to_string = "- throw{{v={velocity};vy={velocity_y}{from_origin}}}")]
    /// Throws the target entity
    Throw {
        /// The horizontal velocity at which the entity is throw
        velocity: u8,
        velocity_y: u8,
        from_origin: MythicOption<bool>,
    },

    #[strum(to_string = "- thunderlevel{{l={level}}}")]
    /// Creates a client-side, per-player rainless storm
    ThunderLevel {
        /// The type of the effect. Either 0 or 1
        level: ThunderLevel,
    },

    #[strum(to_string = "- togglelever{{duration={duration};{location}{x}{y}{z}}}")]
    /// Toggles a lever at the target location
    ToggleLever {
        /// Location of the action, in a x,y,z syntax. If set, other location attributes are ignored
        location: MythicOption<Xyz>,
        /// The duration (in ticks) the lever should remain toggled on.
        duration: u32,
        /// The X coordinate of the button.
        x: MythicOption<i32>,
        /// The Y coordinate of the button.
        y: MythicOption<i32>,
        /// The Z coordinate of the button.
        z: MythicOption<i32>,
    },

    #[strum(to_string = "- togglepiston")]
    /// Toggles a piston at the target location
    TogglePiston,

    #[strum(to_string = "- sit{{state={0}}}")]
    /// Toggles the sitting state for cats, dogs, foxes, and parrots.
    /// Sets the sitting state
    ToggleSitting(bool),

    #[strum(to_string = "- totemofundying{{mode={model}}}")]
    /// Plays the effect of a totem resurrecting a player with options to specify CustomModelData to use from resource packs.
    TotemOfUndying {
        /// The CustomModelData to use for the shown totem
        model: String,
    },

    #[strum(to_string = "- tracklocation")]
    /// Sets the mob's tracked location to the targeted location
    TrackLocation,

    #[strum(to_string = "- undopaste{{id={paste_id}}}")]
    /// Undoes a previous paste done via the fawePaste mechanic, based on its id or on the schematic used
    UndoPaste { paste_id: String },

    #[strum(
        to_string = "- velocity{{m={mode};x={velocity_x};y={velocity_y};z={velocity_z}{relative}}}"
    )]
    /// Modifies the velocity of the targeted entity(s). May be used on players,
    /// too. Useful for all sorts of things like true knockback resistance,
    /// force-skills or simulated wind.
    Velocity {
        /// The operation to perform. Can be SET, ADD, REMOVE, DIVIDE, or MULTIPLY.
        mode: VelocityMode,
        /// Velocity on the x-axis. Can be negative.
        velocity_x: i32,
        /// Velocity on the y-axis. Can be negative.
        velocity_y: i32,
        /// Velocity on the z-axis. Can be negative.
        velocity_z: i32,
        ///If the change in velocity should be relative to the target's facing direction. In this instance, the z axis becomes forward/backward, y becomes up/down and x becomes left/right
        relative: MythicOption<bool>,
    },

    #[strum(to_string = "- wolfsit{{state={0}}}")]
    /// Forces a targeted wolf to sit.
    /// The state the wolf is in. True = sitting and False = standing
    WolfSit(bool),

    #[strum(to_string = "- worldEditReplace{{from={from};to={to}}}")]
    /// Replaces blocks in a region using WorldEdit
    WorldEditReplace {
        /// The material to replace
        from: String,
        /// The material to set in place of the replaced one
        to: String,
    },

    /// Changes the weather for the target player
    #[strum(to_string = "- weather{{type={weather_type};duration={duration}}}")]
    Weather {
        /// The type of weather to set.
        weather_type: WeatherType,
        /// How long (in ticks) the weather will be forced to last
        duration: u32,
    },

    /// Links to mechanics added by addon plugins. Any mechanics from these links will not work without that plugin installed.
    #[strum(to_string = "- {mechanic_type}")]
    AdditionalMecanics { mechanic_type: MechanicType },
}

impl Mechanic {
    pub fn get_desc(&self) -> impl Into<String> {
        match self {
            Mechanic::ActivateSpawner { spawner: _ } => {
                "Activates a MythicMobs spawner at the targeted location"
            }
            Mechanic::AddTrade {
                action: _,
                slot: _,
                ingredient: _,
                ingredient_2: _,
                result: _,
                max_uses: _,
                experience_reward: _,
                villager_exp: _,
                price_multiplier: _,
                demand: _,
                special_price: _,
                ignore_discounts: _,
            } => "Changes the trades of a villager",
            Mechanic::AnimateArmorStand {
                pose: _,
                speed: _,
                duration: _,
                ignore_empty: _,
                smart: _,
            } => "Animates an armor stand",
            Mechanic::ArmAnimation => "Makes the caster swing their arm",
            Mechanic::ArrowVolley {
                amount: _,
                spread: _,
                velocity: _,
                fire_ticks: _,
                remove_delay: _,
                can_pickup: _,
            } => "Fires a volley of arrows",
            Mechanic::AuraRemove {
                aura_name: _,
                stacks: _,
            } => "Adds an attribute modifier to the attributable target",
            Mechanic::BlackScreen { duration: _ } => "Creates a custom boss bar on the casting mob",
            Mechanic::BlockDestabilize => {
                "Causes the targeted blocks to fall, as if affected by gravity"
            }
            Mechanic::BlockMask {
                material: _,
                radius: _,
                radius_y: _,
                noise: _,
                duration: _,
                shape: _,
                no_air: _,
                only_air: _,
                occlude: _,
            } => "Temporarily masks a block as a different block",
            Mechanic::BlockUnmask {
                radius: _,
                shape: _,
            } => "Unmasks blocks that have been masked",
            Mechanic::BlockPhysics => "Triggers a block physics update at the target location",
            Mechanic::BlockWave {
                material: _,
                radius: _,
                radius_y: _,
                duration: _,
                shape: _,
                velocity: _,
                horizontal_velocity: _,
                specific_velocities: _,
                velocity_x: _,
                velocity_y: _,
                velocity_z: _,
                noise: _,
                hide_source_block: _,
                ignore_air: _,
            } => "Creates a wave of blocks at the target location",
            Mechanic::BloodyScreen {
                duration: _,
                cancel: _,
            } => "Makes the target's screen glow red",
            Mechanic::BoneMeal { block_face: _ } => {
                "Applies a bone meal effect to the target blocks"
            }
            Mechanic::BossBorder { radius: _ } => "Creates an inescapable border around the mob",
            Mechanic::Bouncy {
                aura_name: _,
                on_bounce_skill: _,
                cancel_event: _,
            } => "Creates an inescapable border around the mob",
            Mechanic::BreakBlock {
                do_drops: _,
                do_effect: _,
                use_tool: _,
            } => "Breaks the block at the target location",
            Mechanic::BreakBlockAndGiveItem {
                do_drops: _,
                do_effect: _,
                use_tool: _,
                do_fake_looting: _,
                items: _,
            } => "Breaks the block at the target location and gives an item/droptable",
            Mechanic::ClearExperience => "Clears the experience for the targeted players",
            Mechanic::ClearExperienceLevels => {
                " Clears the experience levels for the targeted players"
            }
            Mechanic::GiveExperienceLevels { amount: _ } => {
                "Gives experience levels to the targeted players"
            }
            Mechanic::TakeExperienceLevels { amount: _ } => {
                "Takes experience levels from the targeted players"
            }
            Mechanic::CloseInventory => "Closes the target player's inventory",
            Mechanic::Command {
                command: _,
                as_caster: _,
                as_op: _,
                as_target: _,
                require_target: _,
            } => "Executes a command for each target",
            Mechanic::Consume { damage: _, heal: _ } => {
                "Deals damage and restores health per target hit"
            }
            Mechanic::ConsumeSlot { slot: _, amount: _ } => {
                "Removes an item from a specific slot of the player's inventory"
            }
            Mechanic::DirectionalVelocity {
                yaw: _,
                pitch: _,
                velocity: _,
                mode: _,
            } => "Changes the velocity on the target entity on a specific vector",
            Mechanic::Disengage {
                velocity: _,
                velocity_y: _,
            } => "Causes the caster to leap backwards away from the target entity",
            Mechanic::Disguise { disguise: _ } => "Changes the caster's disguise",
            Mechanic::DisguiseModify { disguise: _ } => {
                "Modifies the caster's already applied disguise"
            }
            Mechanic::DisguiseTarget { disguise: _ } => "Changes the target's disguise",
            Mechanic::Undisguise => "Removes the caster's disguise",
            Mechanic::Dismount => "Makes the caster dismount whatever they're riding",
            Mechanic::DisplayTransformation {
                action: _,
                transformation_type: _,
                value: _,
            } => "Sets the targeted display entity's transformations",
            Mechanic::ClearThreat => "Makes a mob clear its threat table",
            Mechanic::CurrencyGive { amount: _ } => {
                "Gives money to a player. Requires Vault and a currency plugin"
            }
            Mechanic::CurrencyTake { amount: _ } => {
                "Takes money from a player. Requires Vault and a currency plugin"
            }
            Mechanic::Damage {
                amount: _,
                ignore_armor: _,
                prevent_knockback: _,
                prevent_immunity: _,
                damage_cause: _,
                ignore_enchantments: _,
                no_anger: _,
                ignore_invulnerability: _,
                ignore_shield: _,
                damage_helmet: _,
                ignore_effects: _,
                ignore_resistance: _,
                power_affects_damage: _,
                tags: _,
                raw_tags: _,
                element: _,
                trigger_skills: _,
            } => "Damages the target for an amount",
            Mechanic::BaseDamage {
                multiplier: _,
                use_attribute: _,
            } => "Damages the target for a percent of the mob's damage stat",
            Mechanic::PercentDamage {
                percent: _,
                current_health: _,
            } => "Damages the target for a percent of their health",
            Mechanic::Decapitate => "Drops a player head item based on target",
            Mechanic::Doppleganger {
                has_nameplate: _,
                use_player_name: _,
            } => "Copies the appearance of the target player",
            Mechanic::DropItem {
                items: _,
                naturally: _,
                on_drop_skill: _,
            } => "Drops an item or droptable at the target location",
            Mechanic::EjectPassenger => "Ejects anything riding the caster",
            Mechanic::Ender => "Causes the \"Ender\" effect",
            Mechanic::EnderBeam {
                duration: _,
                y_offset: _,
            } => "Creates an EnderCrystal's beam effect to the target",
            Mechanic::EnderDragonResetCrystals => "Generates the EnderDragon crystals",
            Mechanic::EnderDragonSetPhase { phase: _ } => "Sets the EnderDragon phase",
            Mechanic::EnderDragonSetRespawnPhase { phase: _ } => {
                "Sets the EnderDragon respawn phase"
            }
            Mechanic::EnderDragonSpawnPortal { with_portals: _ } => {
                "Generates the portal of the EnderDragon battle"
            }
            Mechanic::Equip { item: _ } => "Causes the casting mob to equip an item",
            Mechanic::EquipCopy { slots: _ } => {
                "Causes the caster to equip a copy of the target's equipment"
            }
            Mechanic::Explosion {
                power_explosion: _,
                block_damage: _,
                fire: _,
            } => "Causes an explosion",
            Mechanic::FakeExplosion => "Causes a fake explosion",
            Mechanic::Extinguish => "Removes fire ticks from the target entity",
            Mechanic::FawePaste {
                schematic: _,
                paste_id: _,
                paste_air: _,
                x_offset: _,
                y_offset: _,
                z_offset: _,
                rotation: _,
                center: _,
                chest_drop_table: _,
                trap_chest_drop_table: _,
                blocks_per_tick: _,
                duration: _,
            } => "Pastes a Schematic using FAWE (Fast Async World Edit)",
            Mechanic::Feed {
                amount: _,
                saturation: _,
                overfeed: _,
            } => "Feeds the target player",
            Mechanic::FillChest {
                items: _,
                should_stack: _,
                should_empty: _,
            } => "Fills a chest with items, or a droptable",
            Mechanic::Firework {
                firework_type: _,
                power: _,
                flicker: _,
                trail: _,
                colors: _,
                fade_colors: _,
            } => "Creates a firework effect at the target",
            Mechanic::Flames => "Creates the flames effect at the location of the targeter",
            Mechanic::Fly => "Applies an aura that allows the targeted player to fly",
            Mechanic::ForcePull {
                spread: _,
                v_spread: _,
            } => "Teleports the target to the caster",
            Mechanic::Freeze { ticks: _ } => {
                "Freezes the target for the given number of ticks using the Powdered Snow freezing effect"
            }
            Mechanic::Geyser {
                liquid_type: _,
                height: _,
                interval: _,
            } => "Creates a \"geyser\" of water or lava",
            Mechanic::GiveItem {
                item: _,
                fake_looting: _,
            } => "Gives an item to the target",
            Mechanic::GiveItemFromSlot {
                slot: _,
                fake_looting: _,
            } => "Gives an item to the target from the item in the given slot of caster",
            Mechanic::GiveItemFromTarget {
                item: _,
                fake_looting: _,
            } => {
                "Gives the caster an item while playing the pickup-item animation from the target entity or location"
            }
            Mechanic::Glow { color: _ } => "Makes the target glow",
            Mechanic::GoatRam => "Causes the casting goat mob to ram the targeted entity",
            Mechanic::GoTo {
                speed: _,
                spread_h: _,
                spread_v: _,
            } => "Move toward the location of the targeter (entity or location)",
            Mechanic::GuardianBeam {
                duration: _,
                interval: _,
                start_y_offset: _,
                target_y_offset: _,
                from_origin: _,
                on_start_skill: _,
                on_tick_skill: _,
                on_end_skill: _,
            } => "Draws a guardian beam between the origin and the target",
            Mechanic::Heal {
                amount: _,
                overheal: _,
                max_overheal: _,
            } => "Heals the target",
            Mechanic::HealPercent {
                multiplier: _,
                overheal: _,
                max_overheal: _,
            } => "Heals the target for a percentage of its max-health",
            Mechanic::Hide {
                ignore_aura_options: _,
            } => "Hides the caster from the targeted player(s) for a set duration",
            Mechanic::Hit {
                multiplier: _,
                forced_damage: _,
                trigger_skills: _,
                scale_by_attack_cooldown: _,
            } => "Simulates a physical hit from the mob",
            Mechanic::Hologram { text: _, stay: _ } => {
                "Summons a hologram to the targeted location"
            }
            Mechanic::Ignite { ticks: _ } => "Sets the target on fire",
            Mechanic::ItemSpray {
                items: _,
                amount: _,
                duration: _,
                radius: _,
                velocity: _,
                y_velocity: _,
                y_offset: _,
                allow_pickup: _,
                gravity: _,
            } => "Causes an explosion of temporary items at the target location",
            Mechanic::JSONMessage { message: _ } => {
                "Sends a JSON-format message to the target player(s)"
            }
            Mechanic::Jump { velocity: _ } => "Causes the caster to jump",
            Mechanic::Leap {
                velocity: _,
                noise: _,
            } => "Causes the caster to leap towards the target",
            Mechanic::Lightning { damage: _ } => "Strikes lightning at the target",
            Mechanic::FakeLightning {
                localized: _,
                localized_radius: _,
            } => "Strikes a fake lightning at the target",
            Mechanic::Log { message: _ } => "Logs a message to console",
            Mechanic::Look {
                head_only: _,
                force: _,
                force_paper: _,
                immediately: _,
            } => "Causes the caster to look at the target",
            Mechanic::Lunge {
                velocity: _,
                velocity_y: _,
                old_math: _,
            } => "Causes the caster to lunge forward at the target",
            Mechanic::MatchRotation { target: _ } => {
                "Sets the caster's yaw and pitch to the same value of the target's"
            }
            Mechanic::Message {
                message: _,
                audience: _,
            } => "Sends a message to the target player(s)",
            Mechanic::ModifyDamage {
                amount: _,
                damage_type: _,
                action: _,
            } => "Modifies the damage event that triggered the skill",
            Mechanic::ModifyGlobalScore {
                objective: _,
                action: _,
                value: _,
            } => "Modifies a scoreboard value of the fake player: GLOBAL",
            Mechanic::ModifyTargetScore {
                objective: _,
                action: _,
                value: _,
            } => "Modifies a scoreboard value of the target",
            Mechanic::ModifyMobScore {
                objective: _,
                action: _,
                value: _,
            } => "Modifies a scoreboard value of the casting mob",
            Mechanic::ModifyScore {
                objective: _,
                action: _,
                value: _,
            } => "Modifies the score of a dummy player",
            Mechanic::Mount { entity: _ } => "Summons a mob for the caster and mounts it",
            Mechanic::MountMe { entity: _ } => "Forces the targeted entity to mount the caster",
            Mechanic::MountTarget { entity: _ } => "Mounts the target",
            Mechanic::MovePin { x: _, y: _, z: _ } => "Moves the given pin to the target location",
            Mechanic::OpenTrades => "Opens the trades of the casting villager to the target player",
            Mechanic::Oxygen { amount: _ } => "Gives oxygen to a player target",
            Mechanic::Particle {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
            } => "Creates particle effects around the target",
            Mechanic::ParticleBox {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                width: _,
                height: _,
            } => "Draws a box of particles around the target",
            Mechanic::ParticleEquation {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                equation: _,
            } => "Generates particles based on equations",
            Mechanic::ParticleLine {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                length: _,
            } => "Draws a line of particle effects to the target",
            Mechanic::ParticleLineHelix {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                length: _,
                radius: _,
            } => "Draws a line based helix effect",
            Mechanic::ParticleLineRing {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Draws a particle ring connected by lines",
            Mechanic::ParticleOrbital {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Draws orbiting particle effects around the target",
            Mechanic::ParticleRing {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Draws a ring of particles around the target",
            Mechanic::ParticleSphere {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Draws a sphere of particles around the target",
            Mechanic::ParticleTornado {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                height: _,
                radius: _,
            } => "Draws a persistent \"tornado\" of particles at the target",
            Mechanic::Atom {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Creates some particles in the shape of an atom",
            Mechanic::PickUpItem { item: _ } => "Picks up the targeted item",
            Mechanic::PlayAnimation { animation: _ } => "Forces the entity to play an animation",
            Mechanic::PlayBlockBreakSound { block: _ } => "Plays a block breaking sound",
            Mechanic::PlayBlockFallSound { block: _ } => "Plays a block falling sound",
            Mechanic::PlayBlockHitSound { block: _ } => "Plays a block hit sound",
            Mechanic::PlayBlockPlaceSound { block: _ } => "Plays a block place sound",
            Mechanic::PlayBlockStepSound { block: _ } => "Plays a block step sound",
            Mechanic::PoseArmorStand { pose: _ } => "Changes the pose of the target ArmorStand",
            Mechanic::Potion {
                potion: _,
                duration: _,
                level: _,
            } => "Applies a potion effect to the target",
            Mechanic::PotionClear => "Removes all potion effects from target entity",
            Mechanic::Prison => "Imprisons the target inside a block",
            Mechanic::PrintParentTree => {
                "Prints debug information regarding the Metaskill executing the mechanic and its SkillTree"
            }
            Mechanic::Propel { velocity: _ } => "Propels the caster towards the target",
            Mechanic::Pull { velocity: _ } => "Pulls the target towards the mob",
            Mechanic::PushBlock { velocity: _ } => {
                "Pushes the block at the target location in the given direction"
            }
            Mechanic::PushButton => "Pushes a button at the target location",
            Mechanic::RayTrace => "Traces a straight line to the target",
            Mechanic::RayTraceTo => {
                "Executes a skill with the result of a raytrace to the target location"
            }
            Mechanic::Rally { radius: _ } => "Causes other nearby mobs to attack the target",
            Mechanic::RandomMessage { messages: _ } => {
                "Sends a random message to the target player"
            }
            Mechanic::Recoil { velocity: _ } => {
                "Kicks the target's screen in order to simulate a recoil"
            }
            Mechanic::Remount => {
                "Remounts the mob the caster originally spawned riding, if it is still alive"
            }
            Mechanic::Remove => "Removes the target mob",
            Mechanic::RemoveHeldItem => "Removes some of the item the target player is holding",
            Mechanic::RemoveOwner => "Removes the ownership of the target mob",
            Mechanic::ResetAI => {
                "Attempts to reset the AI of a casting mob to the base type's default"
            }
            Mechanic::RotateTowards { target: _ } => {
                "Rotates the caster towards the target location"
            }
            Mechanic::RunAIGoalSelector => "Change the caster's AIGoalSelectors",
            Mechanic::RunAITargetSelector => "Change the caster's AITargetSelectors",
            Mechanic::Saddle => "Equips or removes a saddle on the target entity",
            Mechanic::SendActionMessage { message: _ } => {
                "Sends an Actionbar Message to the target player"
            }
            Mechanic::SendResourcePack { url: _ } => "Sends a Resource Pack to the target player",
            Mechanic::SendTitle {
                title: _,
                subtitle: _,
                fade_in: _,
                stay: _,
                fade_out: _,
            } => "Sends a Title/Subtitle Message to the target player",
            Mechanic::SendToast {
                title: _,
                message: _,
            } => "Sends an achievement toast to the target player",
            Mechanic::SetAI { ai: _ } => "Disables/enables the AI of the target mob",
            Mechanic::SetBlockOpen { open: _ } => "Sets the target block's open state",
            Mechanic::SetBlockType { block: _ } => "Change block type at target location",
            Mechanic::SetChunkForceLoaded { loaded: _ } => {
                "Sets the force-loaded status of a location's chunk"
            }
            Mechanic::SetCollidable { collidable: _ } => {
                "Sets if the target should have a collidable hitbox or not"
            }
            Mechanic::SetDragonPodium { podium: _ } => {
                "Sets the position of the dragon's podium at the target location"
            }
            Mechanic::SetGameMode { gamemode: _ } => "Sets the Game Mode of the target player",
            Mechanic::SetGliding { gliding: _ } => "Makes the target glide if they have elytra",
            Mechanic::SetGlobalScore {
                objective: _,
                action: _,
                value: _,
            } => "Sets a scoreboard value on the fake player: GLOBAL",
            Mechanic::SetGravity { gravity: _ } => "Sets whether gravity affects the target entity",
            Mechanic::SetHealth { health: _ } => "Sets the health of the target entity",
            Mechanic::SetInteractionSize {
                width: _,
                height: _,
            } => "Sets the size of the target INTERACTION entity",
            Mechanic::SetItemGroupCooldown {
                group: _,
                cooldown: _,
            } => "Sets the cooldown on an item group for the target player",
            Mechanic::SetDisplayEntityItem { item: _ } => {
                "Sets the item component of ITEM_DISPLAY entities"
            }
            Mechanic::SetLeashHolder { holder: _ } => "Changes the holder of a mob's lead",
            Mechanic::SetLevel { level: _ } => "Changes the casting mob's level",
            Mechanic::SetMaterialCooldown {
                material: _,
                cooldown: _,
            } => "Sets a cooldown for usable materials like ender pearls, chorus fruit, etc",
            Mechanic::SetMaxHealth { health: _ } => "Sets the max health of the target entity",
            Mechanic::SetMobColor { color: _ } => {
                "Changes the color of the target if it is a colorable mob"
            }
            Mechanic::SetMobScore {
                objective: _,
                score: _,
            } => "Sets a scoreboard value on the casting mob",
            Mechanic::SetName { name: _ } => "Changes the caster entity's name",
            Mechanic::SetRaiderCanJoinRaid { can_join: _ } => {
                "Sets if the target raider entity can join a raid or not"
            }
            Mechanic::SetRaiderPatrolBlock { block: _ } => {
                "Sets the target raider to patrol a location"
            }
            Mechanic::SetRaiderPatrolLeader { leader: _ } => "Sets the raider patrol leader",
            Mechanic::SetFaction { faction: _ } => "Changes the target entity's faction",
            Mechanic::SetFlying { flying: _ } => "Sets whether the target player is flying",
            Mechanic::SetNoDamageTicks { ticks: _ } => "Sets the no damage ticks of the target",
            Mechanic::SetOwner { owner: _ } => "Makes the target the owner of the casting mob",
            Mechanic::SetParent { parent: _ } => "Makes the target the parent of the casting mob",
            Mechanic::SetPathfindingMalus { malus: _ } => {
                "Sets the pathfinding malus of a mob for given terrain types"
            }
            Mechanic::SetPitch { pitch: _ } => "Sets the head pitch of the target entity",
            Mechanic::SetPose { pose: _ } => "Sets the entity's pose",
            Mechanic::SetRotation { yaw: _, pitch: _ } => "Sets the rotation of the target",
            Mechanic::SetTarget { target: _ } => "Sets the caster's target",
            Mechanic::SetTargetScore {
                objective: _,
                score: _,
            } => "Sets the score of the target",
            Mechanic::SetTextDisplay { text: _ } => {
                "Sets the text component of target Text Display entity"
            }
            Mechanic::SetTongueTarget { target: _ } => {
                "Sets the tongue target for a frog caster to the target entity"
            }
            Mechanic::SetScore {
                objective: _,
                score: _,
            } => "Sets the scoreboard value of a dummy player",
            Mechanic::SetSpeed { speed: _ } => "Sets the target entity's speed attribute",
            Mechanic::SetStance { stance: _ } => "Sets the stance of the target mob",
            Mechanic::Shield => "Applies an absorb shield to the target entity",
            Mechanic::ShieldBreak => {
                "Forces the player to lower their shield and puts it on cooldown"
            }
            Mechanic::ShieldPercent { percent: _ } => {
                "Applies an absorb shield to the target entity for a percentage of their max health"
            }
            Mechanic::ShootFireball { velocity: _ } => "Shoots a fireball at the target",
            Mechanic::ShootPotion {
                potion: _,
                velocity: _,
            } => "Throws a potion at the target",
            Mechanic::ShootSkull { velocity: _ } => "Shoots a wither skull at the target",
            Mechanic::ShootShulkerBullet { velocity: _ } => {
                "Shoots a shulker bullet at the target entity"
            }
            Mechanic::ShowEntity { entity: _ } => "Shows the hidden caster to the targeted players",
            Mechanic::Signal { signal: _ } => "Sends a signal to a mob",
            Mechanic::Skybox { skybox: _ } => "Alters the target player's skybox",
            Mechanic::Smoke => "Creates a puff of smoke",
            Mechanic::SmokeSwirl => "Creates a persistent \"swirl\" of smoke",
            Mechanic::Sound {
                sound: _,
                volume: _,
                pitch: _,
            } => "Plays a sound effect",
            Mechanic::StealItem { item: _ } => "Steals an item from the target player's inventory",
            Mechanic::StopSound { sound: _ } => "Stops a sound effect from playing",
            Mechanic::StopSoundWithCategory {
                sound: _,
                sound_category: _,
            } => "Stops a sound effect from playing (Specify a category)",
            Mechanic::Speak {
                offset: _,
                radius: _,
                max_line_lenght: _,
                line_prefix: _,
                message: _,
                chat_prefix: _,
                duration: _,
                send_chat_message: _,
            } => "Causes the mob to speak in chat, with options for speech bubbles",
            Mechanic::Spin {
                velocity: _,
                aura: _,
            } => "Causes the target to spin",
            Mechanic::Spring {
                spring_type: _,
                duration: _,
            } => "Creates a temporary spring of liquid at the target",
            Mechanic::Stun { duration: _ } => "Stuns the target entity for a specified duration",
            Mechanic::StopUsingItem => "Stops the targeted entity from using an item",
            Mechanic::Suicide => "Causes the caster to die",
            Mechanic::Summon {
                mob: _,
                location: _,
            } => "Summons a mob at the specified location",
            Mechanic::SummonAreaEffectCloud {
                particle: _,
                effect_type: _,
                potion_duration: _,
                level: _,
                duration: _,
                duration_reduction_on_use: _,
                radius: _,
                radius_reduction_on_use: _,
                radius_reduction_on_tick: _,
            } => "Summons a cloud of particles at the target",
            Mechanic::SummonFallingBlock { material: _ } => "Summons a falling block",
            Mechanic::SummonPassenger {
                passenger: _,
                stack: _,
            } => "Summons a mob to ride the target.",
            Mechanic::Swap => "Swaps locations with the target",
            Mechanic::SwingOffHand => "Makes the casting player swing their offhand",
            Mechanic::AddTag(_) => "Adds a scoreboard tag to the target",
            Mechanic::RemoveTag(_) => "Removes a scoreboard tag from the target",
            Mechanic::TakeItem {
                item: _,
                amount: _,
                exact: _,
                vanilla_only: _,
            } => "Removes an item from the targeted player's inventory",
            Mechanic::Taunt => {
                "Modifies the threat level that the caster holds with the target entities"
            }
            Mechanic::Teleport {
                spreadh: _,
                spreadv: _,
                preserve_pitch: _,
                preserve_yaw: _,
                safe_teleport: _,
            } => "Teleports to the target",
            Mechanic::TeleportY { y: _ } => "Teleports the caster vertically",
            Mechanic::TeleportIn {
                vector: _,
                yaw: _,
                target_as_origin: _,
            } => "Teleports the target relative to the caster's yaw",
            Mechanic::TeleportTo {
                location: _,
                world: _,
                yaw: _,
                pitch: _,
                relative: _,
                target_as_origin: _,
            } => "Teleports the target to a specified location",
            Mechanic::Time {
                mode: _,
                amount: _,
                personal: _,
                relative: _,
            } => "Changes the time",
            Mechanic::Threat { amount: _, mode: _ } => {
                "Modifies the mob's threat towards the target"
            }
            Mechanic::Throw {
                velocity: _,
                velocity_y: _,
                from_origin: _,
            } => "Throws the target entity",
            Mechanic::ThunderLevel { level: _ } => {
                "Creates a client-side, per-player rainless storm"
            }
            Mechanic::ToggleLever {
                location: _,
                duration: _,
                x: _,
                y: _,
                z: _,
            } => "Toggles a lever at the target location",
            Mechanic::TogglePiston => "Toggles a piston at the target location",
            Mechanic::ToggleSitting(_) => {
                "Toggles the sitting state for cats, dogs, foxes, and parrots. Sets the sitting state"
            }
            Mechanic::TotemOfUndying { model: _ } => {
                "Plays the effect of a totem resurrecting a player with options to specify CustomModelData to use from resource packs."
            }
            Mechanic::TrackLocation => "Sets the mob's tracked location to the targeted location",
            Mechanic::UndoPaste { paste_id: _ } => {
                "Undoes a previous paste done via the fawePaste mechanic, based on its id or on the schematic used"
            }
            Mechanic::Velocity {
                mode: _,
                velocity_x: _,
                velocity_y: _,
                velocity_z: _,
                relative: _,
            } => {
                "Modifies the velocity of the targeted entity(s). May be used on players, too. Useful for all sorts of things like true knockback resistance,force-skills or simulated wind."
            }
            Mechanic::WolfSit(_) => {
                "Forces a targeted wolf to sit. The state the wolf is in. True = sitting and False = standing"
            }
            Mechanic::WorldEditReplace { from: _, to: _ } => {
                "Replaces blocks in a region using WorldEdit"
            }
            Mechanic::Weather {
                weather_type: _,
                duration: _,
            } => "Changes the weather for the target player",
            Mechanic::AdditionalMecanics { mechanic_type: _ } => {
                "Links to mechanics added by addon plugins. Any mechanics from these links will not work without that plugin installed."
            }
        }
    }
    pub fn get_fields(&self) -> impl Into<String> {
        match self {
            Mechanic::ActivateSpawner { spawner: _ } => "Activate Spawner",

            Mechanic::AddTrade {
                action: _,
                slot: _,
                ingredient: _,
                ingredient_2: _,
                result: _,
                max_uses: _,
                experience_reward: _,
                villager_exp: _,
                price_multiplier: _,
                demand: _,
                special_price: _,
                ignore_discounts: _,
            } => "Add Trade",
            Mechanic::AnimateArmorStand {
                pose: _,
                speed: _,
                duration: _,
                ignore_empty: _,
                smart: _,
            } => "Animates an armor stand",
            Mechanic::ArmAnimation => "Arm Animation",
            Mechanic::ArrowVolley {
                amount: _,
                spread: _,
                velocity: _,
                fire_ticks: _,
                remove_delay: _,
                can_pickup: _,
            } => "Arrow Volley",
            Mechanic::AuraRemove {
                aura_name: _,
                stacks: _,
            } => "Aura Remove",
            Mechanic::BlackScreen { duration: _ } => "BlackScreen",
            Mechanic::BlockDestabilize => "BlockDestabilize",
            Mechanic::BlockMask {
                material: _,
                radius: _,
                radius_y: _,
                noise: _,
                duration: _,
                shape: _,
                no_air: _,
                only_air: _,
                occlude: _,
            } => "BlockMask",
            Mechanic::BlockUnmask {
                radius: _,
                shape: _,
            } => "BlockUnmask",
            Mechanic::BlockPhysics => "BlockPhysics",
            Mechanic::BlockWave {
                material: _,
                radius: _,
                radius_y: _,
                duration: _,
                shape: _,
                velocity: _,
                horizontal_velocity: _,
                specific_velocities: _,
                velocity_x: _,
                velocity_y: _,
                velocity_z: _,
                noise: _,
                hide_source_block: _,
                ignore_air: _,
            } => "BlockWave",
            Mechanic::BloodyScreen {
                duration: _,
                cancel: _,
            } => "BloodyScreen",
            Mechanic::BoneMeal { block_face: _ } => "BoneMeal",
            Mechanic::BossBorder { radius: _ } => "BossBorder",
            Mechanic::Bouncy {
                aura_name: _,
                on_bounce_skill: _,
                cancel_event: _,
            } => "Bouncy",
            Mechanic::BreakBlock {
                do_drops: _,
                do_effect: _,
                use_tool: _,
            } => "BreakBlock",
            Mechanic::BreakBlockAndGiveItem {
                do_drops: _,
                do_effect: _,
                use_tool: _,
                do_fake_looting: _,
                items: _,
            } => "BreakBlockAndGiveItem",
            Mechanic::ClearExperience => "ClearExperience",
            Mechanic::ClearExperienceLevels => "ClearExperienceLevels",
            Mechanic::GiveExperienceLevels { amount: _ } => "GiveExperienceLevels",
            Mechanic::TakeExperienceLevels { amount: _ } => "TakeExperienceLevels",
            Mechanic::CloseInventory => "CloseInventory",
            Mechanic::Command {
                command: _,
                as_caster: _,
                as_op: _,
                as_target: _,
                require_target: _,
            } => "Command",
            Mechanic::Consume { damage: _, heal: _ } => "Consume",
            Mechanic::ConsumeSlot { slot: _, amount: _ } => "ConsumeSlot",
            Mechanic::DirectionalVelocity {
                yaw: _,
                pitch: _,
                velocity: _,
                mode: _,
            } => "DirectionalVelocity",
            Mechanic::Disengage {
                velocity: _,
                velocity_y: _,
            } => "Disengage",
            Mechanic::Disguise { disguise: _ } => "Disguise",
            Mechanic::DisguiseModify { disguise: _ } => "DisguiseModify",
            Mechanic::DisguiseTarget { disguise: _ } => "DisguiseTarget",
            Mechanic::Undisguise => "Undisguise",
            Mechanic::Dismount => "Dismount",
            Mechanic::DisplayTransformation {
                action: _,
                transformation_type: _,
                value: _,
            } => "DisplayTransformation",
            Mechanic::ClearThreat => "ClearThreat",
            Mechanic::CurrencyGive { amount: _ } => "CurrencyGive",
            Mechanic::CurrencyTake { amount: _ } => "CurrencyTake",
            Mechanic::Damage {
                amount: _,
                ignore_armor: _,
                prevent_knockback: _,
                prevent_immunity: _,
                damage_cause: _,
                ignore_enchantments: _,
                no_anger: _,
                ignore_invulnerability: _,
                ignore_shield: _,
                damage_helmet: _,
                ignore_effects: _,
                ignore_resistance: _,
                power_affects_damage: _,
                tags: _,
                raw_tags: _,
                element: _,
                trigger_skills: _,
            } => "Damage",
            Mechanic::BaseDamage {
                multiplier: _,
                use_attribute: _,
            } => "BaseDamage",
            Mechanic::PercentDamage {
                percent: _,
                current_health: _,
            } => "PercentDamage",
            Mechanic::Decapitate => "Decapitate",
            Mechanic::Doppleganger {
                has_nameplate: _,
                use_player_name: _,
            } => "Doppleganger",
            Mechanic::DropItem {
                items: _,
                naturally: _,
                on_drop_skill: _,
            } => "DropItem",
            Mechanic::EjectPassenger => "EjectPassenger",
            Mechanic::Ender => "Ender",
            Mechanic::EnderBeam {
                duration: _,
                y_offset: _,
            } => "EnderBeam",
            Mechanic::EnderDragonResetCrystals => "EnderDragonResetCrystals",
            Mechanic::EnderDragonSetPhase { phase: _ } => "EnderDragonSetPhase",
            Mechanic::EnderDragonSetRespawnPhase { phase: _ } => "EnderDragonSetRespawnPhase",
            Mechanic::EnderDragonSpawnPortal { with_portals: _ } => "EnderDragonSpawnPortal",
            Mechanic::Equip { item: _ } => "Equip",
            Mechanic::EquipCopy { slots: _ } => "EquipCopy",
            Mechanic::Explosion {
                power_explosion: _,
                block_damage: _,
                fire: _,
            } => "Explosion",
            Mechanic::FakeExplosion => "FakeExplosion",
            Mechanic::Extinguish => "Extinguish",
            Mechanic::FawePaste {
                schematic: _,
                paste_id: _,
                paste_air: _,
                x_offset: _,
                y_offset: _,
                z_offset: _,
                rotation: _,
                center: _,
                chest_drop_table: _,
                trap_chest_drop_table: _,
                blocks_per_tick: _,
                duration: _,
            } => "FawePaste",
            Mechanic::Feed {
                amount: _,
                saturation: _,
                overfeed: _,
            } => "Feed",
            Mechanic::FillChest {
                items: _,
                should_stack: _,
                should_empty: _,
            } => "FillChest",
            Mechanic::Firework {
                firework_type: _,
                power: _,
                flicker: _,
                trail: _,
                colors: _,
                fade_colors: _,
            } => "Firework",
            Mechanic::Flames => "Flames",
            Mechanic::Fly => "Fly",
            Mechanic::ForcePull {
                spread: _,
                v_spread: _,
            } => "ForcePull",
            Mechanic::Freeze { ticks: _ } => "Freeze",
            Mechanic::Geyser {
                liquid_type: _,
                height: _,
                interval: _,
            } => "Geyser",
            Mechanic::GiveItem {
                item: _,
                fake_looting: _,
            } => "GiveItem",
            Mechanic::GiveItemFromSlot {
                slot: _,
                fake_looting: _,
            } => "GiveItemFromSlot",
            Mechanic::GiveItemFromTarget {
                item: _,
                fake_looting: _,
            } => "GiveItemFromTarget",
            Mechanic::Glow { color: _ } => "Glow",
            Mechanic::GoatRam => "GoatRam",
            Mechanic::GoTo {
                speed: _,
                spread_h: _,
                spread_v: _,
            } => "GoTo",
            Mechanic::GuardianBeam {
                duration: _,
                interval: _,
                start_y_offset: _,
                target_y_offset: _,
                from_origin: _,
                on_start_skill: _,
                on_tick_skill: _,
                on_end_skill: _,
            } => "GuardianBeam",
            Mechanic::Heal {
                amount: _,
                overheal: _,
                max_overheal: _,
            } => "Heal",
            Mechanic::HealPercent {
                multiplier: _,
                overheal: _,
                max_overheal: _,
            } => "HealPercent",
            Mechanic::Hide {
                ignore_aura_options: _,
            } => "Hide",
            Mechanic::Hit {
                multiplier: _,
                forced_damage: _,
                trigger_skills: _,
                scale_by_attack_cooldown: _,
            } => "Hit",
            Mechanic::Hologram { text: _, stay: _ } => "Hologram",
            Mechanic::Ignite { ticks: _ } => "Ignite",
            Mechanic::ItemSpray {
                items: _,
                amount: _,
                duration: _,
                radius: _,
                velocity: _,
                y_velocity: _,
                y_offset: _,
                allow_pickup: _,
                gravity: _,
            } => "ItemSpray",
            Mechanic::JSONMessage { message: _ } => "JSONMessage",
            Mechanic::Jump { velocity: _ } => "Jump",
            Mechanic::Leap {
                velocity: _,
                noise: _,
            } => "Leap",
            Mechanic::Lightning { damage: _ } => "Lightning",
            Mechanic::FakeLightning {
                localized: _,
                localized_radius: _,
            } => "FakeLightning",
            Mechanic::Log { message: _ } => "Log",
            Mechanic::Look {
                head_only: _,
                force: _,
                force_paper: _,
                immediately: _,
            } => "Look",
            Mechanic::Lunge {
                velocity: _,
                velocity_y: _,
                old_math: _,
            } => "Lunge",
            Mechanic::MatchRotation { target: _ } => "MatchRotation",
            Mechanic::Message {
                message: _,
                audience: _,
            } => "Message",
            Mechanic::ModifyDamage {
                amount: _,
                damage_type: _,
                action: _,
            } => "ModifyDamage",
            Mechanic::ModifyGlobalScore {
                objective: _,
                action: _,
                value: _,
            } => "ModifyGlobalScore",
            Mechanic::ModifyTargetScore {
                objective: _,
                action: _,
                value: _,
            } => "ModifyTargetScore",
            Mechanic::ModifyMobScore {
                objective: _,
                action: _,
                value: _,
            } => "ModifyMobScore",
            Mechanic::ModifyScore {
                objective: _,
                action: _,
                value: _,
            } => "ModifyScore",
            Mechanic::Mount { entity: _ } => "Mount",
            Mechanic::MountMe { entity: _ } => "MountMe",
            Mechanic::MountTarget { entity: _ } => "MountTarget",
            Mechanic::MovePin { x: _, y: _, z: _ } => "MovePin",
            Mechanic::OpenTrades => "OpenTrades",
            Mechanic::Oxygen { amount: _ } => "Oxygen",
            Mechanic::Particle {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
            } => "Particle",
            Mechanic::ParticleBox {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                width: _,
                height: _,
            } => "ParticleBox",
            Mechanic::ParticleEquation {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                equation: _,
            } => "ParticleEquation",
            Mechanic::ParticleLine {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                length: _,
            } => "ParticleLine",
            Mechanic::ParticleLineHelix {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                length: _,
                radius: _,
            } => "ParticleLineHelix",
            Mechanic::ParticleLineRing {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "ParticleLineRing",
            Mechanic::ParticleOrbital {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "ParticleOrbital",
            Mechanic::ParticleRing {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "ParticleRing",
            Mechanic::ParticleSphere {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "ParticleSphere",
            Mechanic::ParticleTornado {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                height: _,
                radius: _,
            } => "ParticleTornado",
            Mechanic::Atom {
                particle: _,
                amount: _,
                speed: _,
                x: _,
                y: _,
                z: _,
                radius: _,
            } => "Atom",
            Mechanic::PickUpItem { item: _ } => "PickUpItem",
            Mechanic::PlayAnimation { animation: _ } => "PlayAnimation",
            Mechanic::PlayBlockBreakSound { block: _ } => "PlayBlockBreakSound",
            Mechanic::PlayBlockFallSound { block: _ } => "PlayBlockFallSound",
            Mechanic::PlayBlockHitSound { block: _ } => "PlayBlockHitSound",
            Mechanic::PlayBlockPlaceSound { block: _ } => "PlayBlockPlaceSound",
            Mechanic::PlayBlockStepSound { block: _ } => "PlayBlockStepSound",
            Mechanic::PoseArmorStand { pose: _ } => "PoseArmorStand",
            Mechanic::Potion {
                potion: _,
                duration: _,
                level: _,
            } => "Potion",
            Mechanic::PotionClear => "PotionClear",
            Mechanic::Prison => "Prison",
            Mechanic::PrintParentTree => "PrintParentTree",
            Mechanic::Propel { velocity: _ } => "Propel",
            Mechanic::Pull { velocity: _ } => "Pull",
            Mechanic::PushBlock { velocity: _ } => "PushBlock",
            Mechanic::PushButton => "PushButton",
            Mechanic::RayTrace => "RayTrace",
            Mechanic::RayTraceTo => "RayTraceTo",
            Mechanic::Rally { radius: _ } => "Rally",
            Mechanic::RandomMessage { messages: _ } => "RandomMessage",
            Mechanic::Recoil { velocity: _ } => "Recoil",
            Mechanic::Remount => "Remount",
            Mechanic::Remove => "Remove",
            Mechanic::RemoveHeldItem => "RemoveHeldItem",
            Mechanic::RemoveOwner => "RemoveOwner",
            Mechanic::ResetAI => "ResetAI",
            Mechanic::RotateTowards { target: _ } => "RotateTowards",
            Mechanic::RunAIGoalSelector => "RunAIGoalSelector",
            Mechanic::RunAITargetSelector => "RunAITargetSelector",
            Mechanic::Saddle => "Saddle",
            Mechanic::SendActionMessage { message: _ } => "SendActionMessage",
            Mechanic::SendResourcePack { url: _ } => "SendResourcePack",
            Mechanic::SendTitle {
                title: _,
                subtitle: _,
                fade_in: _,
                stay: _,
                fade_out: _,
            } => "SendTitle",
            Mechanic::SendToast {
                title: _,
                message: _,
            } => "SendToast",
            Mechanic::SetAI { ai: _ } => "SetAI",
            Mechanic::SetBlockOpen { open: _ } => "SetBlockOpen",
            Mechanic::SetBlockType { block: _ } => "SetBlockType",
            Mechanic::SetChunkForceLoaded { loaded: _ } => "SetChunkForceLoaded",
            Mechanic::SetCollidable { collidable: _ } => "SetCollidable",
            Mechanic::SetDragonPodium { podium: _ } => "SetDragonPodium",
            Mechanic::SetGameMode { gamemode: _ } => "SetGameMode",
            Mechanic::SetGliding { gliding: _ } => "SetGliding",
            Mechanic::SetGlobalScore {
                objective: _,
                action: _,
                value: _,
            } => "SetGlobalScore",
            Mechanic::SetGravity { gravity: _ } => "SetGravity",
            Mechanic::SetHealth { health: _ } => "SetHealth",
            Mechanic::SetInteractionSize {
                width: _,
                height: _,
            } => "SetInteractionSize",
            Mechanic::SetItemGroupCooldown {
                group: _,
                cooldown: _,
            } => "SetItemGroupCooldown",
            Mechanic::SetDisplayEntityItem { item: _ } => "SetDisplayEntityItem",
            Mechanic::SetLeashHolder { holder: _ } => "SetLeashHolder",
            Mechanic::SetLevel { level: _ } => "SetLevel",
            Mechanic::SetMaterialCooldown {
                material: _,
                cooldown: _,
            } => "SetMaterialCooldown",
            Mechanic::SetMaxHealth { health: _ } => "SetMaxHealth",
            Mechanic::SetMobColor { color: _ } => "SetMobColor",
            Mechanic::SetMobScore {
                objective: _,
                score: _,
            } => "SetMobScore",
            Mechanic::SetName { name: _ } => "SetName",
            Mechanic::SetRaiderCanJoinRaid { can_join: _ } => "SetRaiderCanJoinRaid",
            Mechanic::SetRaiderPatrolBlock { block: _ } => "SetRaiderPatrolBlock",
            Mechanic::SetRaiderPatrolLeader { leader: _ } => "SetRaiderPatrolLeader",
            Mechanic::SetFaction { faction: _ } => "SetFaction",
            Mechanic::SetFlying { flying: _ } => "SetFlying",
            Mechanic::SetNoDamageTicks { ticks: _ } => "SetNoDamageTicks",
            Mechanic::SetOwner { owner: _ } => "SetOwner",
            Mechanic::SetParent { parent: _ } => "SetParent",
            Mechanic::SetPathfindingMalus { malus: _ } => "SetPathfindingMalus",
            Mechanic::SetPitch { pitch: _ } => "SetPitch",
            Mechanic::SetPose { pose: _ } => "SetPose",
            Mechanic::SetRotation { yaw: _, pitch: _ } => "SetRotation",
            Mechanic::SetTarget { target: _ } => "SetTarget",
            Mechanic::SetTargetScore {
                objective: _,
                score: _,
            } => "SetTargetScore",
            Mechanic::SetTextDisplay { text: _ } => "SetTextDisplay",
            Mechanic::SetTongueTarget { target: _ } => "SetTongueTarget",
            Mechanic::SetScore {
                objective: _,
                score: _,
            } => "SetScore",
            Mechanic::SetSpeed { speed: _ } => "SetSpeed",
            Mechanic::SetStance { stance: _ } => "SetStance",
            Mechanic::Shield => "Shield",
            Mechanic::ShieldBreak => "ShieldBreak",
            Mechanic::ShieldPercent { percent: _ } => "ShieldPercent",
            Mechanic::ShootFireball { velocity: _ } => "ShootFireball",
            Mechanic::ShootPotion {
                potion: _,
                velocity: _,
            } => "ShootPotion",
            Mechanic::ShootSkull { velocity: _ } => "ShootSkull",
            Mechanic::ShootShulkerBullet { velocity: _ } => "ShootShulkerBullet",
            Mechanic::ShowEntity { entity: _ } => "ShowEntity",
            Mechanic::Signal { signal: _ } => "Signal",
            Mechanic::Skybox { skybox: _ } => "Skybox",
            Mechanic::Smoke => "Smoke",
            Mechanic::SmokeSwirl => "SmokeSwirl",
            Mechanic::Sound {
                sound: _,
                volume: _,
                pitch: _,
            } => "Sound",
            Mechanic::StealItem { item: _ } => "StealItem",
            Mechanic::StopSound { sound: _ } => "StopSound",
            Mechanic::StopSoundWithCategory {
                sound: _,
                sound_category: _,
            } => "StopSoundWithCategory",
            Mechanic::Speak {
                offset: _,
                radius: _,
                max_line_lenght: _,
                line_prefix: _,
                message: _,
                chat_prefix: _,
                duration: _,
                send_chat_message: _,
            } => "Speak",
            Mechanic::Spin {
                velocity: _,
                aura: _,
            } => "Spin",
            Mechanic::Spring {
                spring_type: _,
                duration: _,
            } => "Spring",
            Mechanic::Stun { duration: _ } => "Stun",
            Mechanic::StopUsingItem => "StopUsingItem",
            Mechanic::Suicide => "Suicide",
            Mechanic::Summon {
                mob: _,
                location: _,
            } => "Summon",
            Mechanic::SummonAreaEffectCloud {
                particle: _,
                effect_type: _,
                potion_duration: _,
                level: _,
                duration: _,
                duration_reduction_on_use: _,
                radius: _,
                radius_reduction_on_use: _,
                radius_reduction_on_tick: _,
            } => "SummonAreaEffectCloud",
            Mechanic::SummonFallingBlock { material: _ } => "SummonFallingBlock",
            Mechanic::SummonPassenger {
                passenger: _,
                stack: _,
            } => "SummonPassenger",
            Mechanic::Swap => "Swap",
            Mechanic::SwingOffHand => "SwingOffHand",
            Mechanic::AddTag(_) => "AddTa",
            Mechanic::RemoveTag(_) => "RemoveTa",
            Mechanic::TakeItem {
                item: _,
                amount: _,
                exact: _,
                vanilla_only: _,
            } => "TakeItem",
            Mechanic::Taunt => "Taunt",
            Mechanic::Teleport {
                spreadh: _,
                spreadv: _,
                preserve_pitch: _,
                preserve_yaw: _,
                safe_teleport: _,
            } => "Teleport",
            Mechanic::TeleportY { y: _ } => "TeleportY",
            Mechanic::TeleportIn {
                vector: _,
                yaw: _,
                target_as_origin: _,
            } => "TeleportIn",
            Mechanic::TeleportTo {
                location: _,
                world: _,
                yaw: _,
                pitch: _,
                relative: _,
                target_as_origin: _,
            } => "TeleportTo",
            Mechanic::Time {
                mode: _,
                amount: _,
                personal: _,
                relative: _,
            } => "Time",
            Mechanic::Threat { amount: _, mode: _ } => "Threat",
            Mechanic::Throw {
                velocity: _,
                velocity_y: _,
                from_origin: _,
            } => "Throw",
            Mechanic::ThunderLevel { level: _ } => "ThunderLevel",
            Mechanic::ToggleLever {
                location: _,
                duration: _,
                x: _,
                y: _,
                z: _,
            } => "ToggleLever",
            Mechanic::TogglePiston => "TogglePiston",
            Mechanic::ToggleSitting(_) => "ToggleSittin",
            Mechanic::TotemOfUndying { model: _ } => "TotemOfUndying",
            Mechanic::TrackLocation => "TrackLocation",
            Mechanic::UndoPaste { paste_id: _ } => "UndoPaste",
            Mechanic::Velocity {
                mode: _,
                velocity_x: _,
                velocity_y: _,
                velocity_z: _,
                relative: _,
            } => "Velocity",
            Mechanic::WolfSit(_) => "WolfSi",
            Mechanic::WorldEditReplace { from: _, to: _ } => "WorldEditReplace",
            Mechanic::Weather {
                weather_type: _,
                duration: _,
            } => "Weather",
            Mechanic::AdditionalMecanics { mechanic_type: _ } => "AdditionalMecanics",
        }
    }
}

impl Default for Mechanic {
    fn default() -> Self {
        Self::ActivateSpawner {
            spawner: SpawnerSelect::SpawnerName("".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq, strum::EnumIter)]
pub enum SpawnerSelect {
    #[strum(to_string = "{0}")]
    SpawnerName(String),
    #[strum(to_string = "g:{0}")]
    SpawnerGroup(String),
    #[strum(to_string = "{0}*")]
    SpawnerIncrementName(String),
}

impl Default for SpawnerSelect {
    fn default() -> Self {
        Self::SpawnerName(String::new())
    }
}

impl SpawnerSelect {
    pub fn modify_inner(&mut self, v: impl Into<String>) {
        match &mut *self {
            SpawnerSelect::SpawnerName(name) => *name = v.into(),
            SpawnerSelect::SpawnerGroup(group) => *group = v.into(),
            SpawnerSelect::SpawnerIncrementName(iname) => *iname = v.into(),
        }
    }
    pub fn get_fields(&self) -> impl Into<String> {
        match self {
            SpawnerSelect::SpawnerName(_) => "Name",
            SpawnerSelect::SpawnerGroup(_) => "Group",
            SpawnerSelect::SpawnerIncrementName(_) => "Increment Name",
        }
    }

    pub fn handle_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Spawner").on_hover_ui(|ui| {
            ui.label("The name of the spawner(s) to activate. This can accept groups and wildcards also using the appropriate syntax");
        });
        ui.horizontal(|ui| {
            egui::ComboBox::new("spawner_select", "")
                .selected_text(self.get_fields().into())
                .show_ui(ui, |ui| {
                    [
                        Self::SpawnerName(String::new()),
                        Self::SpawnerGroup(String::new()),
                        Self::SpawnerIncrementName(String::new()),
                    ]
                    .iter()
                    .for_each(|v| {
                        ui.selectable_value(self, v.clone(), v.get_fields().into());
                    })
                });
            match self {
                SpawnerSelect::SpawnerName(name) => {
                    ui.label("Name");
                    ui.text_edit_singleline(name)
                }
                SpawnerSelect::SpawnerGroup(name) => {
                    ui.label("Group");
                    ui.text_edit_singleline(name)
                }
                SpawnerSelect::SpawnerIncrementName(name) => {
                    ui.label("Incremented Name");
                    ui.text_edit_singleline(name)
                }
            }
        });
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq, strum::EnumIter)]
pub enum ActionMode {
    #[strum(to_string = "ADD")]
    #[default]
    Add,
    #[strum(to_string = "REMOVE")]
    Remove,
    #[strum(to_string = "REPLACE")]
    Replace,
}
impl ActionMode {
    pub fn handle_ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::new("Add_Action_Mode_Dropdown_Menu", "")
            .selected_text(self.to_string())
            .show_ui(ui, |ui| {
                ActionMode::iter().for_each(|v| {
                    ui.selectable_value(self, v.clone(), v.to_string());
                });
            });
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TradeIngredient {
    name: String,
    count: u8,
}
impl TradeIngredient {
    pub fn handle_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Ingredient").on_hover_ui(|ui| {
                ui.label("The minecraft ingredient");
            });
            ui.text_edit_singleline(&mut self.name);
            ui.label("Count");
            ui.add(egui::DragValue::new(&mut self.count).range(0.0..=64.0));
        });
    }
}

impl std::fmt::Display for TradeIngredient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.count)
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ArmorStandPose {
    /// The position of the head.
    pub head: [f32; 3],
    /// The position of the body.
    pub body: [f32; 3],
    /// The position of the left arm.
    pub left_arm: [f32; 3],
    /// The position of the right arm.
    pub right_arm: [f32; 3],
    /// The position of the left leg.
    pub left_leg: [f32; 3],
    /// The position of the right leg.
    pub right_leg: [f32; 3],
}

impl std::fmt::Display for ArmorStandPose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1}]",
            self.head[0],
            self.head[1],
            self.head[2],
            self.body[0],
            self.body[1],
            self.body[2],
            self.left_arm[0],
            self.left_arm[1],
            self.left_arm[2],
            self.right_arm[0],
            self.right_arm[1],
            self.right_arm[2],
            self.left_leg[0],
            self.left_leg[1],
            self.left_leg[2],
            self.right_leg[0],
            self.right_leg[1],
            self.right_leg[2]
        )
    }
}
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct ItemArray {
    items: Vec<String>,
}
impl std::fmt::Display for ItemArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.items.iter().for_each(|item| {
            s.push_str(item);
        });
        write!(f, "{s}")
    }
}
#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct Tags {
    tags: Vec<String>,
}
impl Tags {
    pub fn add_tag(mut self, t: impl Into<String>) -> Self {
        self.tags.push(t.into());
        self
    }
}
impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.tags.iter().for_each(|t| {
            s.push_str(t);
        });
        write!(f, "{s}")
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Percentage {
    value: f32,
}

impl Percentage {
    pub fn new(value: f32) -> Result<Self, String> {
        if (0.0..=1.0).contains(&value) {
            Ok(Percentage { value })
        } else {
            Err("Percentage must be between 0.0 and 1.0".to_string())
        }
    }
}

impl std::fmt::Display for Percentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct EquipmentItem {
    item: String,
    slot: EquipmentSlot,
}

impl EquipmentItem {
    pub fn add_item(mut self, item_name: impl Into<String>) -> Self {
        self.item = item_name.into();
        self
    }

    pub fn select_slot(mut self, slot: EquipmentSlot) -> Self {
        self.slot = slot;
        self
    }
}

impl std::fmt::Display for EquipmentItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.item, self.slot)
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq)]
pub enum EquipmentSlot {
    #[strum(to_string = "HEAD")]
    ///The head slot. Accepts regular helmets, playerheads, and even blocktypes.
    Head,

    #[strum(to_string = "CHEST")]
    ///The chest slot. Will only render chestplates, but will carry any items.
    Chest,

    #[strum(to_string = "LEGS")]
    ///The leg slot. Will only render leggings, but will carry any items.
    Legs,

    #[strum(to_string = "FEET")]
    ///The feet slot. Will only render boots, but will carry any items.
    Feet,

    #[strum(to_string = "HAND")]
    ///The mainhand (right) hand slot.
    Hand,

    #[strum(to_string = "OFFHAND")]
    ///The offhand (left) hand slot.
    OffHand,
}

impl Default for EquipmentSlot {
    fn default() -> Self {
        Self::Hand
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct RandomMessages {
    messages: Vec<String>,
}

impl std::fmt::Display for RandomMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.messages.iter().for_each(|m| {
            s.push_str(&format!("\"{m}\","));
        });
        if let Some(s) = s.strip_suffix(",") {
            s.to_string().push(';');
            return write!(f, "{s}");
        }
        write!(f, "{s}")
    }
}

#[derive(
    Serialize, Deserialize, strum::Display, Clone, strum::VariantArray, Default, PartialEq,
)]
pub enum SoundCategory {
    #[strum(to_string = "AMBIANT")]
    Ambiant,
    #[strum(to_string = "BLOCKS")]
    Blocks,
    #[strum(to_string = "HOSTILE")]
    Hostile,
    #[strum(to_string = "MASTER")]
    #[default]
    Master,
    #[strum(to_string = "MUSIC")]
    Music,
    #[strum(to_string = "NEUTRAL")]
    Neutral,
    #[strum(to_string = "PLAYERS")]
    Players,
    #[strum(to_string = "RECORDS")]
    Records,
    #[strum(to_string = "UI")]
    Ui,
    #[strum(to_string = "VOICE")]
    Voice,
    #[strum(to_string = "WEATHER")]
    Weather,
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AuraMechanic(String);
impl std::fmt::Display for AuraMechanic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq)]
pub enum SpringType {
    #[strum(to_string = "water")]
    #[default]
    Water,
    #[strum(to_string = "lava")]
    Lava,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DurationReduction {
    duration: u32,
}
impl std::fmt::Display for MythicOption<DurationReduction> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let MythicOption::Some(pre, d, suf) = self {
            // pre: drou=
            // suf: ;
            return write!(f, "{}{}{}", pre, d.duration, suf);
        }
        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RadiusReductionOnUse {
    radius: u32,
}
impl std::fmt::Display for MythicOption<RadiusReductionOnUse> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let MythicOption::Some(pre, r, suf) = self {
            // pre: rrou=
            // suf: ;
            return write!(f, "{}{}{}", pre, r.radius, suf);
        }
        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RadiusReductionOnTick {
    radius: u32,
}
impl std::fmt::Display for MythicOption<RadiusReductionOnTick> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let MythicOption::Some(pre, r, suf) = self {
            // pre: rrot=
            // suf: ;
            return write!(f, "{}{}{}", pre, r.radius, suf);
        }
        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, strum::VariantArray, PartialEq)]
pub enum Particle {
    #[strum(to_string = "ANGRY_VILLAGER")]
    AngryVillager,
    #[strum(to_string = "ASH")]
    Ash,
    #[strum(to_string = "BLOCK")]
    Block,
    #[strum(to_string = "BLOCK_CRUMBLE")]
    BlockCrumble,
    #[strum(to_string = "BLOCK_MARKER")]
    BlockMarker,
    #[strum(to_string = "BUBBLE")]
    Bubble,
    #[strum(to_string = "BUBBLE_COLUMN_UP")]
    BubbleColumnUp,
    #[strum(to_string = "BUBBLE_POP")]
    BubblePop,
    #[strum(to_string = "CAMPFIRE_COSY_SMOKE")]
    CampfireCosySmoke,
    #[strum(to_string = "CAMPFIRE_SIGNAL_SMOKE")]
    CampfireSignalSmoke,
    #[strum(to_string = "CHERRY_LEAVES")]
    CherryLeaves,
    #[strum(to_string = "CLOUD")]
    Cloud,
    #[strum(to_string = "COMPOSTER")]
    Composter,
    #[strum(to_string = "CRIMSON_SPORE")]
    CrimsonSpore,
    #[strum(to_string = "CRIT")]
    Crit,
    #[strum(to_string = "CURRENT_DOWN")]
    CurrentDown,
    #[strum(to_string = "DAMAGE_INDICATOR")]
    DamageIndicator,
    #[strum(to_string = "DOLPHIN")]
    Dolphin,
    #[strum(to_string = "DRAGON_BREATH")]
    DragonBreath,
    #[strum(to_string = "DRIPPING_DRIPSTONE_LAVA")]
    DrippingDripstoneLava,
    #[strum(to_string = "DRIPPING_DRIPSTONE_WATER")]
    DrippingDripstoneWater,
    #[strum(to_string = "DRIPPING_HONEY")]
    DrippingHoney,
    #[strum(to_string = "DRIPPING_LAVA")]
    DrippingLava,
    #[strum(to_string = "DRIPPING_OBSIDIAN_TEAR")]
    DrippingObsidianTear,
    #[strum(to_string = "DRIPPING_WATER")]
    DrippingWater,
    #[strum(to_string = "DUST")]
    Dust,
    #[strum(to_string = "DUST_COLOR_TRANSITION")]
    DustColorTransition,
    #[strum(to_string = "DUST_PILLAR")]
    DustPillar,
    #[strum(to_string = "DUST_PLUME")]
    DustPlume,
    #[strum(to_string = "EFFECT")]
    Effect,
    #[strum(to_string = "EGG_CRACK")]
    EggCrack,
    #[strum(to_string = "ELDER_GUARDIAN")]
    ElderGuardian,
    #[strum(to_string = "ELECTRIC_SPARK")]
    ElectricSpark,
    #[strum(to_string = "ENCHANT")]
    Enchant,
    #[strum(to_string = "ENCHANTED_HIT")]
    EnchantedHit,
    #[strum(to_string = "END_ROD")]
    EndRod,
    #[strum(to_string = "ENTITY_EFFECT")]
    EntityEffect,
    #[strum(to_string = "EXPLOSION")]
    Explosion,
    #[strum(to_string = "EXPLOSION_EMITTER")]
    ExplosionEmitter,
    #[strum(to_string = "FALLING_DRIPSTONE_LAVA")]
    FallingDripstoneLava,
    #[strum(to_string = "FALLING_DRIPSTONE_WATER")]
    FallingDripstoneWater,
    #[strum(to_string = "FALLING_DUST")]
    FallingDust,
    #[strum(to_string = "FALLING_HONEY")]
    FallingHoney,
    #[strum(to_string = "FALLING_LAVA")]
    FallingLava,
    #[strum(to_string = "FALLING_NECTAR")]
    FallingNectar,
    #[strum(to_string = "FALLING_OBSIDIAN_TEAR")]
    FallingObsidianTear,
    #[strum(to_string = "FALLING_SPORE_BLOSSOM")]
    FallingSporeBlossom,
    #[strum(to_string = "FALLING_WATER")]
    FallingWater,
    #[strum(to_string = "FIREFLY")]
    Firefly,
    #[strum(to_string = "FIREWORK")]
    Firework,
    #[strum(to_string = "FISHING")]
    Fishing,
    #[strum(to_string = "FLAME")]
    Flame,
    #[strum(to_string = "FLASH")]
    Flash,
    #[strum(to_string = "GLOW")]
    Glow,
    #[strum(to_string = "GLOW_SQUID_INK")]
    GlowSquidInk,
    #[strum(to_string = "GUST")]
    Gust,
    #[strum(to_string = "GUST_EMITTER_LARGE")]
    GustEmitterLarge,
    #[strum(to_string = "GUST_EMITTER_SMALL")]
    GustEmitterSmall,
    #[strum(to_string = "HAPPY_VILLAGER")]
    HappyVillager,
    #[strum(to_string = "HEART")]
    Heart,
    #[strum(to_string = "INFESTED")]
    Infested,
    #[strum(to_string = "INSTANT_EFFECT")]
    InstantEffect,
    #[strum(to_string = "ITEM")]
    Item,
    #[strum(to_string = "ITEM_COBWEB")]
    ItemCobweb,
    #[strum(to_string = "ITEM_SLIME")]
    ItemSlime,
    #[strum(to_string = "ITEM_SNOWBALL")]
    ItemSnowball,
    #[strum(to_string = "LANDING_HONEY")]
    LandingHoney,
    #[strum(to_string = "LANDING_LAVA")]
    LandingLava,
    #[strum(to_string = "LANDING_OBSIDIAN_TEAR")]
    LandingObsidianTear,
    #[strum(to_string = "LARGE_SMOKE")]
    LargeSmoke,
    #[strum(to_string = "LAVA")]
    Lava,
    #[strum(to_string = "MYCELIUM")]
    Mycelium,
    #[strum(to_string = "NAUTILUS")]
    Nautilus,
    #[strum(to_string = "NOTE")]
    Note,
    #[strum(to_string = "OMINOUS_SPAWNING")]
    OminousSpawning,
    #[strum(to_string = "PALE_OAK_LEAVES")]
    PaleOakLeaves,
    #[strum(to_string = "POOF")]
    Poof,
    #[strum(to_string = "PORTAL")]
    Portal,
    #[strum(to_string = "RAID_OMEN")]
    RaidOmen,
    #[strum(to_string = "RAIN")]
    Rain,
    #[strum(to_string = "REVERSE_PORTAL")]
    ReversePortal,
    #[strum(to_string = "SCRAPE")]
    Scrape,
    #[strum(to_string = "SCULK_CHARGE")]
    SculkCharge,
    #[strum(to_string = "SCULK_CHARGE_POP")]
    SculkChargePop,
    #[strum(to_string = "SCULK_SOUL")]
    SculkSoul,
    #[strum(to_string = "SHRIEK")]
    Shriek,
    #[strum(to_string = "SMALL_FLAME")]
    SmallFlame,
    #[strum(to_string = "SMALL_GUST")]
    SmallGust,
    #[strum(to_string = "SMOKE")]
    Smoke,
    #[strum(to_string = "SNEEZE")]
    Sneeze,
    #[strum(to_string = "SNOWFLAKE")]
    Snowflake,
    #[strum(to_string = "SONIC_BOOM")]
    SonicBoom,
    #[strum(to_string = "SOUL")]
    Soul,
    #[strum(to_string = "SOUL_FIRE_FLAME")]
    SoulFireFlame,
    #[strum(to_string = "SPIT")]
    Spit,
    #[strum(to_string = "SPLASH")]
    Splash,
    #[strum(to_string = "SPORE_BLOSSOM_AIR")]
    SporeBlossomAir,
    #[strum(to_string = "SQUID_INK")]
    SquidInk,
    #[strum(to_string = "SWEEP_ATTACK")]
    SweepAttack,
    #[strum(to_string = "TINTED_LEAVES")]
    TintedLeaves,
    #[strum(to_string = "TOTEM_OF_UNDYING")]
    TotemOfUndying,
    #[strum(to_string = "TRAIL")]
    Trail,
    #[strum(to_string = "TRIAL_OMEN")]
    TrialOmen,
    #[strum(to_string = "TRIAL_SPAWNER_DETECTION")]
    TrialSpawnerDetection,
    #[strum(to_string = "TRIAL_SPAWNER_DETECTION_OMINOUS")]
    TrialSpawnerDetectionOminous,
    #[strum(to_string = "UNDERWATER")]
    Underwater,
    #[strum(to_string = "VAULT_CONNECTION")]
    VaultConnection,
    #[strum(to_string = "VIBRATION")]
    Vibration,
    #[strum(to_string = "WARPED_SPORE")]
    WarpedSpore,
    #[strum(to_string = "WAX_OFF")]
    WaxOff,
    #[strum(to_string = "WAX_ON")]
    WaxOn,
    #[strum(to_string = "WHITE_ASH")]
    WhiteAsh,
    #[strum(to_string = "WHITE_SMOKE")]
    WhiteSmoke,
    #[strum(to_string = "WITCH")]
    Witch,
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq)]
pub enum EffectType {
    #[strum(to_string = "SLOW")]
    Slow,
}

#[derive(
    Serialize, Deserialize, strum::Display, Clone, strum::VariantArray, Default, PartialEq,
)]
pub enum AddSetReset {
    #[strum(to_string = "ADD")]
    #[default]
    Add,
    #[strum(to_string = "SET")]
    Set,
    #[strum(to_string = "RESET")]
    Reset,
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq)]
pub enum ThreatMode {
    /// Adds amount threat to the target entity
    #[strum(to_string = "add")]
    Add,
    /// Removes amount threat from the target entity
    #[strum(to_string = "remove")]
    Remove,
    /// Multiplies the threat against the target entity by amount
    #[strum(to_string = "multiply")]
    Multiply,
    /// Divides the threat against the target entity by amount
    #[strum(to_string = "divide")]
    Divide,
    ///Sets the threat against the target entity to amount
    #[strum(to_string = "set")]
    Set,
    ///Remove all threat from the target entity
    #[strum(to_string = "reset")]
    Reset,
    ///Gives the target enough threat to be moved to the top of the threat list
    #[strum(to_string = "forcetop")]
    ForceTop,
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq)]
pub enum ThunderLevel {
    #[strum(to_string = "0")]
    #[default]
    Level0,
    #[strum(to_string = "1")]
    Level1,
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq)]
pub enum VelocityMode {
    #[strum(to_string = "set")]
    #[default]
    Set,
    #[strum(to_string = "add")]
    Add,
    #[strum(to_string = "remove")]
    Remove,
    #[strum(to_string = "divide")]
    Divide,
    #[strum(to_string = " multiply")]
    Multiply,
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq)]
pub enum WeatherType {
    #[strum(to_string = "sun")]
    #[default]
    Sunny,
    #[strum(to_string = "rain")]
    Rainy,
    #[strum(to_string = "storm")]
    Stormy,
}

#[derive(
    Serialize, Deserialize, strum::Display, Clone, Default, PartialEq, strum::VariantArray,
)]
pub enum Shape {
    #[strum(to_string = "sphere")]
    Sphere,
    #[strum(to_string = "Cube")]
    #[default]
    Cube,
}

impl Shape {
    pub fn sphere() -> Self {
        Self::Sphere
    }
    pub fn cube() -> Self {
        Self::Cube
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq)]
pub enum MechanicType {
    #[strum(to_string = "ModelEngineMechanic", serialize = "{mechanic}")]
    ModelEngine4 {
        mechanic: ModelEngineMechanic,
    },
    MythicCrucible,
    MythicEnchantments,
    MCPets,
}
impl Default for MechanicType {
    fn default() -> Self {
        Self::ModelEngine4 {
            mechanic: ModelEngineMechanic::default(),
        }
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, Default, PartialEq)]
pub enum ModelEngineMechanic {
    ///Entity type mechanics controls and configures options used by the base entity.
    #[strum(to_string = "{model}")]
    Entity {
        model: MythicOption<ModelEngineModel>,
    },
    #[default]
    Model,
    Bone,
    Mounting,
    Controller,
    Hitbox,
    Segment,
    Misc,
    VFXMechanics,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ModelEngineModel {
    /// The model id of the model
    model_id: String,
    /// Is this mechanic used for removing model
    remove: RemoveAttributeBool,
}

impl std::fmt::Display for MythicOption<ModelEngineModel> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let MythicOption::Some(_, s, _) = self {
            return write!(f, "m={};{}", s.model_id, s.remove);
        }
        write!(f, "")
    }
}

#[derive(Serialize, Deserialize, strum::Display, Clone, PartialEq)]
pub enum RemoveAttributeBool {
    #[strum(to_string = "")]
    True,
    #[strum(to_string = "{{attribute}}")]
    False { attribute: RemoveAttribute },
}

impl Default for RemoveAttributeBool {
    fn default() -> Self {
        Self::False {
            attribute: RemoveAttribute::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RemoveAttribute {
    /// Should the model override the hitbox
    hitbox: MythicOption<bool>,
    /// Should the base entity be invisible
    invisible: MythicOption<bool>,
    /// Should the model flash red when damaged
    damagetint: MythicOption<bool>,
    /// The tag bone used to display the name
    nametag: MythicOption<String>,
    /// Can this model be driven
    drive: MythicOption<bool>,
    /// Can this model have passengers
    ride: MythicOption<bool>,
    /// Should the model's pitch be locked
    lockpitch: MythicOption<bool>,
    /// Should the model's yaw be locked
    lockyaw: MythicOption<bool>,
    /// The step height of the model
    step: MythicOption<f32>,
    /// Visible radius of the model
    radius: MythicOption<i8>,
    /// Visible radius of the model. Values less than or equal to 0 will not be read
    scale: MythicOption<f32>,
    /// Collision hitbox scale of the model. Caution: Large hitboxes might cause server lag
    hitbox_scale: MythicOption<f32>,
    /// Use the new animation systemHighly recommended to switch to this system
    use_state_machine: MythicOption<bool>,
    /// Whether the model should immediately be shown to players. If false, this allows you to further customize your model before showing with [RenderInit](https://git.mythiccraft.io/mythiccraft/model-engine-4/-/wikis/Skills/Mechanics/RenderInit)
    init_render: MythicOption<bool>,
    /// Should the model's hitbox be visible.If hidden, the player can no longer interact with the model.
    show_hitbox: MythicOption<bool>,
    /// Should the shadow be visible
    show_shadow: MythicOption<bool>,
    /// Should the body rotation be synced to the head rotation
    sync_body: MythicOption<bool>,
    /// Should the model be saved on entity unload
    save: MythicOption<bool>,
}

impl Default for RemoveAttribute {
    fn default() -> Self {
        Self {
            hitbox: MythicOption::Some("h=".to_owned(), true, ";".to_owned()),
            invisible: MythicOption::Some("i=".to_owned(), true, ";".to_owned()),
            damagetint: MythicOption::Some("d=".to_owned(), true, ";".to_owned()),
            nametag: MythicOption::Some("d=".to_owned(), String::new(), ";".to_owned()),
            drive: MythicOption::Some("drive=".to_owned(), false, ";".to_owned()),
            ride: MythicOption::Some("ride=".to_owned(), false, ";".to_owned()),
            lockpitch: MythicOption::Some("lp=".to_owned(), false, ";".to_owned()),
            lockyaw: MythicOption::Some("ly=".to_owned(), false, ";".to_owned()),
            step: MythicOption::Some("s=".to_owned(), 0.5, ";".to_owned()),
            radius: MythicOption::Some("rad=".to_string(), 0, ";".to_owned()),
            scale: MythicOption::Some("scale=".to_owned(), 1.0, ";".to_owned()),
            hitbox_scale: MythicOption::None,
            use_state_machine: MythicOption::Some("usm=".to_owned(), false, ";".to_owned()),
            init_render: MythicOption::Some("init=".to_owned(), true, ";".to_owned()),
            show_hitbox: MythicOption::Some("showhitbox".to_owned(), true, ";".to_owned()),
            show_shadow: MythicOption::Some("showshadow=".to_owned(), true, ";".to_owned()),
            sync_body: MythicOption::Some("syncbody=".to_owned(), true, ";".to_owned()),
            save: MythicOption::Some("save=".to_owned(), false, ";".to_owned()),
        }
    }
}

impl RemoveAttribute {
    /// Should the model's hitbox be visible.If hidden, the player can no longer interact with the model.
    pub fn show_hitebox(mut self, b: bool) -> Self {
        self.show_hitbox = MythicOption::Some(String::from("showhitbox="), b, String::from(";"));
        self
    }
    /// Should the model override the hitbox
    pub fn hitbox(mut self, b: bool) -> Self {
        self.hitbox = MythicOption::Some("h=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the base entity be invisible
    pub fn invisible(mut self, b: bool) -> Self {
        self.invisible = MythicOption::Some("i=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the model flash red when damaged
    pub fn damage_tint(mut self) -> Self {
        self.damagetint = MythicOption::Some("d=".to_owned(), true, ";".to_owned());
        self
    }
    /// The tag bone used to display the name
    pub fn nametag(mut self, s: String) -> Self {
        self.nametag = MythicOption::Some("d=".to_owned(), s, ";".to_owned());
        self
    }
    /// Can this model be driven
    pub fn drive(mut self, b: bool) -> Self {
        self.drive = MythicOption::Some("drive=".to_owned(), b, ";".to_owned());
        self
    }
    /// Can this model have passengers
    pub fn ride(mut self, b: bool) -> Self {
        self.drive = MythicOption::Some("drive=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the model's pitch be locked
    pub fn lock_pitch(mut self, b: bool) -> Self {
        self.lockpitch = MythicOption::Some("lp=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the model's yaw be locked
    pub fn lock_yaw(mut self, b: bool) -> Self {
        self.lockyaw = MythicOption::Some("ly=".to_owned(), b, ";".to_owned());
        self
    }
    /// The step height of the model
    pub fn step(mut self, f: f32) -> Self {
        self.step = MythicOption::Some("s=".to_owned(), f, ";".to_owned());
        self
    }
    /// Visible radius of the model
    pub fn radius(mut self, i: i8) -> Self {
        self.radius = MythicOption::Some("rad=".to_string(), i, ";".to_owned());
        self
    }
    /// Visible radius of the model. Values less than or equal to 0 will not be read
    pub fn scale(mut self, f: f32) -> Self {
        self.scale = MythicOption::Some("scale=".to_owned(), f, ";".to_owned());
        self
    }
    /// Collision hitbox scale of the model. Caution: Large hitboxes might cause server lag
    pub fn hitbox_scale(mut self, f: f32) -> Self {
        self.hitbox_scale = MythicOption::Some(String::from("hitboxscale="), f, String::from(";"));
        self
    }
    /// Use the new animation systemHighly recommended to switch to this system
    pub fn state_machine(mut self) -> Self {
        self.use_state_machine = MythicOption::Some("usm=".to_owned(), false, ";".to_owned());
        self
    }
    /// Whether the model should immediately be shown to players. If false, this allows you to further customize your model before showing with [RenderInit](https://git.mythiccraft.io/mythiccraft/model-engine-4/-/wikis/Skills/Mechanics/RenderInit)
    pub fn init_rend(mut self, b: bool) -> Self {
        self.init_render = MythicOption::Some("init=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the shadow be visible
    pub fn show_shadow(mut self, b: bool) -> Self {
        self.show_shadow = MythicOption::Some("showshadow=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the body rotation be synced to the head rotation
    pub fn sync_body(mut self, b: bool) -> Self {
        self.sync_body = MythicOption::Some("syncbody=".to_owned(), b, ";".to_owned());
        self
    }
    /// Should the model be saved on entity unload
    pub fn save(mut self, b: bool) -> Self {
        self.save = MythicOption::Some("save=".to_owned(), b, ";".to_owned());
        self
    }
}
