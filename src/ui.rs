use crate::{
    skill::{mechanics::{Mechanic, MythicOption, TradeIngredient}, minecraft_lib::MinecraftMob, targeters::{MultyEntity, SingleEntityTarget, Targeter}, triggers::Trigger},
    states::AppState,
};
use eframe::egui;
use egui::{Button, Color32};
use strum::{IntoEnumIterator, VariantArray};

pub struct Ui {
    pub state: AppState,
}

impl Ui {
    pub fn new(_cc: &eframe::CreationContext<'_>, state: AppState) -> Self {
        Self { state }
    }
}

impl eframe::App for Ui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Top").show(ctx, |ui| {
            ui.heading("MythicMobs Skill Editor");
            if ui.add(Button::new("Preview")).clicked() {
                self.state.create_config();
            }
        });

        egui::SidePanel::left("SidePanel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(if let MinecraftMob::MetaSkill = self.state.mob_type {"Internal Skill Name"} else {"Internal Name"}).on_hover_ui(|ui| {
                    if let MinecraftMob::MetaSkill = self.state.mob_type {ui.label("It's the string that will identify the metaskill inside mythicmobs, exactly how the [Internal Name] works for mobs.
A valid Internal SkillName must be unique (aka, there cannot exists two skills that shares the skillname) and not containt any space character.
If you want to execute a specific metaskill in any way, you will have to use its Internal SkillName in some way");} else {ui.label("This string will be how your mob will be referenced internally in MythicMobs and can be any name you like.
Must be a unique name and does not clash with other internal mob names, NO SPACES ALLOWED.");}
                });
                    ui.text_edit_singleline(&mut self.state.mob_name);
            });
            ui.horizontal(|ui| {
                if let MinecraftMob::MetaSkill = self.state.mob_type {
                   self.state.mob_display_name.clear();
                   self.state.mob_health = 0;
                   self.state.damage = 0;
                   self.state.armor = 0; 
                } else {
                ui.label("Mob Display Name").on_hover_ui(|ui| {ui.label("Sets the display name of the mob.
This option supports color codes and placeholders.
The mob's name will not change or update on its own, you have to use setname mechanic to change or update it.");});
                ui.text_edit_singleline(&mut self.state.mob_display_name);
                    
                }
            });
            ui.horizontal(|ui| {
                ui.label("Type").on_hover_ui(|ui| {
                    if let MinecraftMob::MetaSkill = self.state.mob_type {
                        ui.label("A Metaskill is, in essence, a list of mechanics to execute once the metaskill is called via a Meta Mechanic.
They are located in ../plugins/MythicMobs/Skills inside .yml files, just like their mobs counterpart.");
                    }else {
                    ui.label("This field determines which entity type your creation will be based upon.
A complete list of available entity types can be found on spigot javadocs, while here you can find a list of types that are explicitly implemented.

Several mob options for new entity types that Minecraft adds to the base game will not function until Mythic adds support for said entity types


Some entity types can have negative, hard-to-discover quirks. It is advised that you refer to the unstable entity types page in order to form a better opinion on what entity you should be using for your current endeavor");}
});
                egui::ComboBox::new("Mob_Type", "")
                    .selected_text(self.state.mob_type.to_string())
                    .show_ui(ui, |ui| {
                        MinecraftMob::VARIANTS.iter().for_each(|v| {
                            ui.selectable_value(&mut self.state.mob_type, v.clone(), v.to_string());
                        });
                    });
            });
            ui.horizontal(|ui| {
                if let MinecraftMob::MetaSkill = self.state.mob_type {} else {
                ui.add(
                    egui::Slider::new(&mut self.state.mob_health, 0..=2048)
                        .text("Mob Health")
                ).on_hover_ui(|ui| {
                        ui.label(r"Sets the base value of the mob's max health attribute.
Mythic doesn't have any limitations on max health but Spigot, however, caps the max health at 2048.
This can easily be changed in spigot's configuration file, server_root\spigot.yml.
Whenever the mob is holding or wearing an item with attribute modifiers will also affect the total max health.");
                    });
                    
                }
            });
            ui.horizontal(|ui| {
                if let MinecraftMob::MetaSkill = self.state.mob_type {

                } else {
                                    ui.add(
                    egui::Slider::new(&mut self.state.damage, 0..=255)
                        .text("Mob Damage")
                        .step_by(1.0),
                ).on_hover_ui(|ui| {
                        ui.label("Sets the base value of the mob's melee attack damage attribute.
1 damage equals to 0.5 hearts, so a mob with 6 damage will deal 3 full hearts of damage.
This attribute will never affect damage done by ranged attacks, like arrows or potions.
Whenever the mob is holding or wearing an item with attribute modifiers will also affect the mob's melee damage.");
                    });                    
                }
            });
            ui.horizontal(|ui| {
                if let MinecraftMob::MetaSkill = self.state.mob_type {} else {
                ui.add(egui::Slider::new(&mut self.state.armor, 0..=255).text("Mob Armor").step_by(1.0)).on_hover_ui(|ui| {
                    ui.label("Sets the base value of the mob's armor attribute.
Minecraft caps the max armor value to 30.
Whenever the mob is holding or wearing an item with attribute modifiers will also affect the total armor.");
                });
                    
                }
            });
            ui.separator();
            if ui.add(Button::new("Add Skill")).clicked() {
                self.state.add_skill();
            }
            if ui.add(Button::new("Remove Skill")).clicked() {
                self.state.remove_selected_skill();
            }
            if let Some(sk) = self.state.get_selected_skill() {
                ui.horizontal(|ui| {
                    ui.label("Skill name");
                    ui.add(egui::TextEdit::singleline(&mut sk.name));
                });
            }

            ui.separator();
            self.state
                .skills
                .clone()
                .iter()
                .enumerate()
                .for_each(|(id, sk)| {
                    if let Some(selected_id) = self.state.selected_skill {
                        if id == selected_id {
                            ui.horizontal(|ui| {
                                ui.add(
                                    Button::new(sk.name.clone())
                                        .fill(Color32::from_rgb(127, 127, 127)),
                                );
                                if selected_id != 0 && ui.button("Move Skill Up").clicked() {
                                    self.state.skills.insert(selected_id - 1, sk.clone());
                                    self.state.remove_skill(selected_id + 1);
                                    self.state.select_skill(selected_id - 1);
                                }
                                if selected_id != self.state.skills.len() - 1
                                    && ui.button("Move Skill Down").clicked()
                                {
                                    self.state.skills.insert(selected_id + 2, sk.clone());
                                    self.state.remove_skill(selected_id);
                                    self.state.select_skill(selected_id + 1);
                                }
                            });
                        } else if ui.add(Button::new(sk.name.clone())).clicked() {
                            self.state.select_skill(id);
                        };
                    }
                });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(selected_id) = self.state.selected_skill {
                if let Some(sk) = self.state.skills.get_mut(selected_id) {
                    ui.horizontal(|ui| {
                        ui.label("Mechanic").on_hover_ui(|ui| {ui.label("Skill Mechanics (or base skills) are simple skills that are built into
MythicMobs. You can call these basic skills by themselves in your mob's
Skill List, or you can create your own meta-skill by combining these
mechanics together.
Some Mechanics are able to target Entities, Locations, or both! Some
don't target anything. You control what your skill targets using a
Targeter.");});
                    egui::ComboBox::new("Mechanic", "")
                        .selected_text(if let Some(mech) = &sk.mechanic {
                            mech.get_fields().into()
                        } else {
                            "None".to_owned()
                        })
                        .show_ui(ui, |ui| {
                            Mechanic::iter().for_each(|v| {
                                ui.selectable_value(
                                    &mut sk.mechanic,
                                    Some(v.clone()),
                                    v.get_fields().into(),
                                );
                            });
                            ui.selectable_value(&mut sk.mechanic, None, "None".to_string());
                            ui.separator();
                        });                        
                    });

                    // Mechanics
                    if let Some(mechanic) = &mut sk.mechanic {
                        process_mechanic_ui(ui, mechanic);
                    } else {
                        ui.separator();
                        ui.label("Raw Args");
                        ui.text_edit_singleline(&mut sk.raw_args);
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Targeters");
                    egui::ComboBox::new("Targeters_Selector", "").selected_text(sk.targeter.get_fields().into()).show_ui(ui, |ui| {
                        Targeter::iter().for_each(|v| {
                       ui.selectable_value(&mut sk.targeter, v.clone(), v.get_fields().into()); 
                        });
                    });
                        
                    });
                    
                    // Targeter
                    process_targeter_ui(ui, &mut sk.targeter);

                    // Trigger
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Trigger").on_hover_ui(|ui| {
                            ui.label("Triggers are used to determine how a skill is triggered from within the
mobs skill configuration section.
TRIGGERS CANNOT BE USED IN META-SKILLS AND SHOULD NOT BE INCLUDED
IN THEM.
Triggers can only be used to activate the meta-skill
itself (in the mob's configuration file).

Each trigger starts with a on string. That string is case sensitive, so make sure to write it correctly or the trigger will not work");
                        });
                    egui::ComboBox::new("Trigger_Dropdown_Menu", "").selected_text(sk.trigger.get_fields().into()).show_ui(ui, |ui| {
                        Trigger::VARIANTS.iter().for_each(|v| {
                            ui.selectable_value(&mut sk.trigger, v.clone(), v.get_fields().into());
                        });
                    });
 
                    });

                }
            }
        });
        egui::TopBottomPanel::bottom("output_yaml_panel").show(ctx, |ui| {
            if self.state.output.is_empty() {
            } else {
                ui.add(egui::TextEdit::multiline(&mut self.state.output.clone()).code_editor());
            }
        });
    }
}

fn process_trigger_ui(ui: &mut egui::Ui, trigger: &mut crate::skill::triggers::Trigger)  {
}

fn process_targeter_ui(ui: &mut egui::Ui, targeter: &mut crate::skill::targeters::Targeter){
    ui.horizontal(|ui| {

    match targeter {
        crate::skill::targeters::Targeter::SingleEntity(single_entity_target) => {
            ui.label("Single Entity:");

            egui::ComboBox::new("Single_Entity_Target_Dropdown_Menu", "").selected_text(single_entity_target.to_string()).show_ui(ui, |ui| {
                SingleEntityTarget::VARIANTS.iter().for_each(|v| {
                ui.selectable_value(single_entity_target, v.clone(), v.to_string());
                    
            })
                
            });
        },
        crate::skill::targeters::Targeter::MultyEntity(multy_entity) => {
            ui.label("Mutli Entity:");
            egui::ComboBox::new("Multi_Entity_Target_Dropdown_Menu", "").selected_text(multy_entity.to_string()).show_ui(ui, |ui| {
                MultyEntity::VARIANTS.iter().for_each(|v| {
                    ui.selectable_value(multy_entity, v.clone(), v.to_string());
                });
            });
        },
        crate::skill::targeters::Targeter::ThreatTable => {},
        crate::skill::targeters::Targeter::None => {},
        }
    });
}

/// Display the mechanic parameters as ui
fn process_mechanic_ui(ui: &mut egui::Ui, mechanic: &mut Mechanic) {
    ui.heading("Description");
    ui.label(mechanic.get_desc().into());
    ui.separator();
    match mechanic {
        Mechanic::ActivateSpawner { spawner } => {
            spawner.handle_ui(ui);
        }
        Mechanic::AddTrade {
            action,
            slot,
            ingredient,
            ingredient_2,
            result,
            max_uses,
            experience_reward,
            villager_exp,
            price_multiplier,
            demand,
            special_price,
            ignore_discounts,
        } => {
            ui.horizontal(|ui| {
                action.handle_ui(ui);
                ui.horizontal(|ui| {
                    ui.label("Slot").on_hover_ui(|ui| {
                        ui.label("The slot to be selected for the action. Slot starts at 0, so if a villager has 3 trades, the middle trade would be slot 1");
                    });
                    ui.add(egui::DragValue::new(slot).range(0.0..=3.0));
                });
            });
            ui.vertical(|ui| {
                ingredient.handle_ui(ui);
                if let MythicOption::None = ingredient_2 {
                    if ui.button("Add Ingredient").clicked() {
                        *ingredient_2 = MythicOption::Some(";item2=".to_owned(),TradeIngredient::default(),"".to_owned());
                    }
                } else if let MythicOption::Some(pre,ingredient_2, suf) = ingredient_2 {
                ingredient_2.handle_ui(ui);
                    
                }
            });
            ui.horizontal(|ui| {
                ui.label("Result").on_hover_ui(|ui| {
                    ui.label("The result item of the trade");
                });
                ui.text_edit_singleline(result);
            });
            ui.horizontal(|ui| {
                ui.label("Max Uses").on_hover_ui(|ui| {
                    ui.label("The uses of the trade");
                });
                if let Some((_,max_uses,_)) = max_uses.some() {
                    ui.add(egui::DragValue::new(max_uses));
                    
                } else if ui.button("+").clicked() {
                    *max_uses = MythicOption::Some(";uses=".to_string(), 0, "".to_string());
                }
            });
            if let Some((_,experience_reward,_)) = experience_reward.some() {
            ui.checkbox(experience_reward, "Experience Reward")
                .on_hover_ui(|ui| {
                    ui.label("If the trade should drop experience");
                });                
            } else if ui.button("+").clicked() {
                *experience_reward = MythicOption::Some(";expReward=".to_owned(), false, "".to_owned());
            }

            ui.horizontal(|ui| {
                ui.label("Villager Experience").on_hover_ui(|ui| {
                    ui.label(
                        "The amount of experience to give to the villager upon successful trade",
                    );
                });
                if let Some((_,villager_exp,_)) = villager_exp.some() {
                ui.add(egui::DragValue::new(villager_exp));} else if ui.button("Add Villager Exp").clicked() {
                    *villager_exp = MythicOption::Some(";villExp=".to_owned(), 0, "".to_owned());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Price Multiplier").on_hover_ui(|ui| {
                    ui.label(
                        "The multiplier for the price when the player has made the villager angry",
                    );
                });
                if let Some((_,price_multiplier,_)) = price_multiplier.some() {
                ui.add(egui::DragValue::new(price_multiplier));} else if ui.button("+").clicked() {
                    *price_multiplier = MythicOption::Some(";multiplier=".to_owned(), 0, "".to_owned());
                }
            });

            ui.horizontal(|ui| {
                ui.label("Demand").on_hover_ui(|ui| {
                    ui.label("The demand of the trade");
                });
                if let Some((_,demand,_)) = demand.some() {ui.add(egui::DragValue::new(demand));} else if ui.button("+").clicked() {
                    *demand = MythicOption::Some(";d=".to_owned(), 1, "".to_ascii_lowercase());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Special Price").on_hover_ui(|ui| {ui.label("The special price for when the villager is friendly to the player (player reputation or hero of the village effect)");});
                if let Some((_,special_price,_)) = special_price.some() {
                ui.add(egui::DragValue::new(special_price));} else if ui.button("+").clicked() {
                    *special_price = MythicOption::Some(String::from(";special="), 1, String::new());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Ignore Discount").on_hover_ui(|ui| {
                    ui.label("If the discounts should be ignored");
                });
                if let Some((_,ignore_discounts,_)) = ignore_discounts.some() {
                ui.checkbox(ignore_discounts, "");} else if ui.button("+").clicked() {
                    *ignore_discounts = MythicOption::Some(";discounts=".to_string(), false, String::new());
                }
            });
        }
        Mechanic::AnimateArmorStand {
            pose,
            speed,
            duration,
            ignore_empty,
            smart,
        } => {
            ui.horizontal(|ui| {
                ui.label("Head").on_hover_ui(|ui| {ui.label("The position of the head.");});
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.head[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.head[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.head[2]));
            });
            ui.horizontal(|ui| {
                ui.label("Body").on_hover_ui(|ui| {ui.label("The position of the body.");});
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.body[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.body[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.body[2]));
            });
            ui.horizontal(|ui| {
                ui.label("Left Arm").on_hover_ui(|ui| {ui.label("The position of the left arm.");}) ;
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.left_arm[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.left_arm[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.left_arm[2]));
            });
            ui.horizontal(|ui| {
                ui.label("Right Arm").on_hover_ui(|ui|{ui.label("The position of the right arm.");});
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.right_arm[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.right_arm[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.right_arm[2]));
            });
            ui.horizontal(|ui| {
                ui.label("Left Leg").on_hover_ui(|ui| {ui.label("The position of the left leg.");});
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.left_leg[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.left_leg[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.left_leg[2]));
            });
            ui.horizontal(|ui|{
                ui.label("Right Leg").on_hover_ui(|ui| {ui.label("The position of the right leg.");});
                ui.label("X");
                ui.add(egui::DragValue::new(&mut pose.right_leg[0]));
                ui.label("Y");
                ui.add(egui::DragValue::new(&mut pose.right_leg[1]));
                ui.label("Z");
                ui.add(egui::DragValue::new(&mut pose.right_leg[2]));
            });
            ui.horizontal(|ui|{
                ui.label("Speed").on_hover_ui(|ui| {ui.label("The speed of the animation.");} );
                ui.add(egui::DragValue::new(speed));
            });
            ui.horizontal(|ui| {
                ui.label("Duration").on_hover_ui(|ui|{ui.label("The duration of the animation in ticks.");});
                ui.add(egui::DragValue::new(duration));
            }); 
            ui.horizontal(|ui| {
                ui.label("Ignore Empty").on_hover_ui(|ui| {ui.label("Whether to ignore empty pose values.");});
                ui.checkbox(ignore_empty, "Ignore_Empty_Checkbox");
            });
            ui.horizontal(|ui| {
                ui.label("Smart").on_hover_ui(|ui| {ui.label("Whether to use smart animation.");});
                ui.checkbox(smart, "Smart_Checkbox");
            });

        },
        Mechanic::ArmAnimation => (),
        Mechanic::ArrowVolley {
            amount,
            spread,
            velocity,
            fire_ticks,
            remove_delay,
            can_pickup,
        } => {
            ui.horizontal(|ui| {
                ui.label("Amout").on_hover_ui(|ui| {ui.label("The number of arrows in the volley.");});
                ui.add(egui::DragValue::new(amount));
            });
            ui.horizontal(|ui| {
                ui.label("Spread").on_hover_ui(|ui| {ui.label("How spread out the arrows are.");});
                ui.add(egui::DragValue::new(spread));
            });
            ui.horizontal(|ui| {
                ui.label("Velocity").on_hover_ui(|ui| {ui.label("The velocity of the arrows.");});
                ui.add(egui::DragValue::new(velocity));
            });
            ui.horizontal(|ui| {
                ui.label("Fire Ticks").on_hover_ui(|ui| {ui.label("The duration hit entities will burn for in ticks.");});
                ui.add(egui::DragValue::new(fire_ticks));
            });
            ui.horizontal(|ui| {
                ui.label("Remove Delay").on_hover_ui(|ui| {ui.label("The time the arrows will stay before disappearing in ticks.");});
                ui.add(egui::DragValue::new(remove_delay));
            });
            ui.horizontal(|ui| {
                ui.label("Can Pickup").on_hover_ui(|ui| {ui.label("Whether the arrows can be picked up by players.");});
                ui.checkbox(can_pickup, "Can_Pickup_Checkbox");
            });
        },
        Mechanic::AuraRemove { aura_name, stacks } => {
            ui.horizontal(|ui|{
                ui.text_edit_singleline(aura_name).on_hover_ui(|ui|{ui.label("The name of the aura to remove.");});
                ui.label("Amount").on_hover_ui(|ui| {ui.label("The amount of stacks to remove.");});
                ui.add(egui::DragValue::new(stacks));
            });
        },
        Mechanic::BlackScreen { duration } => {
            ui.label("Duration").on_hover_ui(|ui| {ui.label("The duration of the black screen effect.");});
            ui.add(egui::DragValue::new(duration));
        },
        Mechanic::BlockDestabilize => (),
        Mechanic::BlockMask {
            material,
            radius,
            radius_y,
            noise,
            duration,
            shape,
            no_air,
            only_air,
            occlude,
        } => {
            ui.horizontal(|ui| {
                ui.label("Material").on_hover_ui(|ui| {ui.label("The type of block used for the blockmask.");});
                ui.text_edit_singleline(material);
            });
            ui.horizontal(|ui|{
                ui.label("Radius").on_hover_ui(|ui|{ui.label("The radius of the blockmask effect.");});
                ui.add(egui::DragValue::new(radius));
            });
            ui.horizontal(|ui| {
                ui.label("Radius Y").on_hover_ui(|ui| {ui.label("The y component of the radius.");});
                ui.add(egui::DragValue::new(radius_y));
            });
            ui.horizontal(|ui| {
                ui.label("Noise").on_hover_ui(|ui|{ui.label("Defines the randomness of the effect.");});
                ui.add(egui::DragValue::new(noise));
            });
            ui.horizontal(|ui| {
                ui.label("Duration").on_hover_ui(|ui| {ui.label("Duration of the effect in ticks.");});
                ui.add(egui::DragValue::new(duration));
            });
            ui.horizontal(|ui| {
                ui.label("Shape").on_hover_ui(|ui| {ui.label("The shape of the effect.");});
                egui::ComboBox::from_id_salt("Shape Dropdown Menu")
                    .selected_text(shape.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(shape, crate::skill::mechanics::BlockMaskShape::Sphere, "Sphere");
                        ui.selectable_value(shape, crate::skill::mechanics::BlockMaskShape::Cube, "Cube");
                });
            });
            ui.horizontal(|ui| {
                ui.label("No Air").on_hover_ui(|ui| {ui.label("Mask no air blocks only.");});
                ui.checkbox(no_air, "No_Air_Checkbox");
            });
            ui.horizontal(|ui| {
                ui.label("Only Air").on_hover_ui(|ui| {ui.label("Mask air blocks only.");});
                ui.checkbox(only_air, "Only_Air_Checkbox");
            });
            ui.horizontal(|ui| {
                ui.label("Occlude").on_hover_ui(|ui| {ui.label("If only_air is used, target transparent blocks as well.");});
                ui.checkbox(occlude, "Occlude_Checkbox");
            });
        },
        Mechanic::BlockUnmask { radius, shape } => (),
        Mechanic::BlockPhysics => (),
        Mechanic::BlockWave {
            material,
            radius,
            radius_y,
            duration,
            shape,
            velocity,
            horizontal_velocity,
            specific_velocities,
            velocity_x,
            velocity_y,
            velocity_z,
            noise,
            hide_source_block,
            ignore_air,
        } => (),
        Mechanic::BloodyScreen { duration, cancel } => (),
        Mechanic::BoneMeal { block_face } => (),
        Mechanic::BossBorder { radius } => (),
        Mechanic::Bouncy {
            aura_name,
            on_bounce_skill,
            cancel_event,
        } => (),
        Mechanic::BreakBlock {
            do_drops,
            do_effect,
            use_tool,
        } => (),
        Mechanic::BreakBlockAndGiveItem {
            do_drops,
            do_effect,
            use_tool,
            do_fake_looting,
            items,
        } => (),
        Mechanic::ClearExperience => (),
        Mechanic::ClearExperienceLevels => (),
        Mechanic::GiveExperienceLevels { amount } => (),
        Mechanic::TakeExperienceLevels { amount } => (),
        Mechanic::CloseInventory => (),
        Mechanic::Command {
            command,
            as_caster,
            as_op,
            as_target,
            require_target,
        } => (),
        Mechanic::Consume { damage, heal } => (),
        Mechanic::ConsumeSlot { slot, amount } => (),
        Mechanic::DirectionalVelocity {
            yaw,
            pitch,
            velocity,
            mode,
        } => (),
        Mechanic::Disengage {
            velocity,
            velocity_y,
        } => (),
        Mechanic::Disguise { disguise } => (),
        Mechanic::DisguiseModify { disguise } => (),
        Mechanic::DisguiseTarget { disguise } => (),
        Mechanic::Undisguise => (),
        Mechanic::Dismount => (),
        Mechanic::DisplayTransformation {
            action,
            transformation_type,
            value,
        } => (),
        Mechanic::ClearThreat => (),
        Mechanic::CurrencyGive { amount } => (),
        Mechanic::CurrencyTake { amount } => (),
        Mechanic::Damage {
            amount,
            ignore_armor,
            prevent_knockback,
            prevent_immunity,
            damage_cause,
            ignore_enchantments,
            no_anger,
            ignore_invulnerability,
            ignore_shield,
            damage_helmet,
            ignore_effects,
            ignore_resistance,
            power_affects_damage,
            tags,
            raw_tags,
            element,
            trigger_skills,
        } => (),
        Mechanic::BaseDamage {
            multiplier,
            use_attribute,
        } => (),
        Mechanic::PercentDamage {
            percent,
            current_health,
        } => (),
        Mechanic::Decapitate => (),
        Mechanic::Doppleganger {
            has_nameplate,
            use_player_name,
        } => (),
        Mechanic::DropItem {
            items,
            naturally,
            on_drop_skill,
        } => (),
        Mechanic::EjectPassenger => (),
        Mechanic::Ender => (),
        Mechanic::EnderBeam { duration, y_offset } => (),
        Mechanic::EnderDragonResetCrystals => (),
        Mechanic::EnderDragonSetPhase { phase } => (),
        Mechanic::EnderDragonSetRespawnPhase { phase } => (),
        Mechanic::EnderDragonSpawnPortal { with_portals } => (),
        Mechanic::Equip { item } => (),
        Mechanic::EquipCopy { slots } => (),
        Mechanic::Explosion {
            power_explosion,
            block_damage,
            fire,
        } => (),
        Mechanic::FakeExplosion => (),
        Mechanic::Extinguish => (),
        Mechanic::FawePaste {
            schematic,
            paste_id,
            paste_air,
            x_offset,
            y_offset,
            z_offset,
            rotation,
            center,
            chest_drop_table,
            trap_chest_drop_table,
            blocks_per_tick,
            duration,
        } => (),
        Mechanic::Feed {
            amount,
            saturation,
            overfeed,
        } => (),
        Mechanic::FillChest {
            items,
            should_stack,
            should_empty,
        } => (),
        Mechanic::Firework {
            firework_type,
            power,
            flicker,
            trail,
            colors,
            fade_colors,
        } => (),
        Mechanic::Flames => (),
        Mechanic::Fly => (),
        Mechanic::ForcePull { spread, v_spread } => (),
        Mechanic::Freeze { ticks } => (),
        Mechanic::Geyser {
            liquid_type,
            height,
            interval,
        } => (),
        Mechanic::GiveItem { item, fake_looting } => (),
        Mechanic::GiveItemFromSlot { slot, fake_looting } => (),
        Mechanic::GiveItemFromTarget { item, fake_looting } => (),
        Mechanic::Glow { color } => (),
        Mechanic::GoatRam => (),
        Mechanic::GoTo {
            speed,
            spread_h,
            spread_v,
        } => (),
        Mechanic::GuardianBeam {
            duration,
            interval,
            start_y_offset,
            target_y_offset,
            from_origin,
            on_start_skill,
            on_tick_skill,
            on_end_skill,
        } => (),
        Mechanic::Heal {
            amount,
            overheal,
            max_overheal,
        } => (),
        Mechanic::HealPercent {
            multiplier,
            overheal,
            max_overheal,
        } => (),
        Mechanic::Hide {
            ignore_aura_options,
        } => (),
        Mechanic::Hit {
            multiplier,
            forced_damage,
            trigger_skills,
            scale_by_attack_cooldown,
        } => (),
        Mechanic::Hologram { text, stay } => (),
        Mechanic::Ignite { ticks } => (),
        Mechanic::ItemSpray {
            items,
            amount,
            duration,
            radius,
            velocity,
            y_velocity,
            y_offset,
            allow_pickup,
            gravity,
        } => (),
        Mechanic::JSONMessage { message } => (),
        Mechanic::Jump { velocity } => (),
        Mechanic::Leap { velocity, noise } => (),
        Mechanic::Lightning { damage } => (),
        Mechanic::FakeLightning {
            localized,
            localized_radius,
        } => (),
        Mechanic::Log { message } => (),
        Mechanic::Look {
            head_only,
            force,
            force_paper,
            immediately,
        } => (),
        Mechanic::Lunge {
            velocity,
            velocity_y,
            old_math,
        } => (),
        Mechanic::MatchRotation { target } => (),
        Mechanic::Message { message, audience } => (),
        Mechanic::ModifyDamage {
            amount,
            damage_type,
            action,
        } => (),
        Mechanic::ModifyGlobalScore {
            objective,
            action,
            value,
        } => (),
        Mechanic::ModifyTargetScore {
            objective,
            action,
            value,
        } => (),
        Mechanic::ModifyMobScore {
            objective,
            action,
            value,
        } => (),
        Mechanic::ModifyScore {
            objective,
            action,
            value,
        } => (),
        Mechanic::Mount { entity } => (),
        Mechanic::MountMe { entity } => (),
        Mechanic::MountTarget { entity } => (),
        Mechanic::MovePin { x, y, z } => (),
        Mechanic::OpenTrades => (),
        Mechanic::Oxygen { amount } => (),
        Mechanic::Particle {
            particle,
            amount,
            speed,
            x,
            y,
            z,
        } => (),
        Mechanic::ParticleBox {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            width,
            height,
        } => (),
        Mechanic::ParticleEquation {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            equation,
        } => (),
        Mechanic::ParticleLine {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            length,
        } => (),
        Mechanic::ParticleLineHelix {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            length,
            radius,
        } => (),
        Mechanic::ParticleLineRing {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            radius,
        } => (),
        Mechanic::ParticleOrbital {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            radius,
        } => (),
        Mechanic::ParticleRing {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            radius,
        } => (),
        Mechanic::ParticleSphere {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            radius,
        } => (),
        Mechanic::ParticleTornado {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            height,
            radius,
        } => (),
        Mechanic::Atom {
            particle,
            amount,
            speed,
            x,
            y,
            z,
            radius,
        } => (),
        Mechanic::PickUpItem { item } => (),
        Mechanic::PlayAnimation { animation } => (),
        Mechanic::PlayBlockBreakSound { block } => (),
        Mechanic::PlayBlockFallSound { block } => (),
        Mechanic::PlayBlockHitSound { block } => (),
        Mechanic::PlayBlockPlaceSound { block } => (),
        Mechanic::PlayBlockStepSound { block } => (),
        Mechanic::PoseArmorStand { pose } => (),
        Mechanic::Potion {
            potion,
            duration,
            level,
        } => (),
        Mechanic::PotionClear => (),
        Mechanic::Prison => (),
        Mechanic::PrintParentTree => (),
        Mechanic::Propel { velocity } => (),
        Mechanic::Pull { velocity } => (),
        Mechanic::PushBlock { velocity } => (),
        Mechanic::PushButton => (),
        Mechanic::RayTrace => (),
        Mechanic::RayTraceTo => (),
        Mechanic::Rally { radius } => (),
        Mechanic::RandomMessage { messages } => (),
        Mechanic::Recoil { velocity } => (),
        Mechanic::Remount => (),
        Mechanic::Remove => (),
        Mechanic::RemoveHeldItem => (),
        Mechanic::RemoveOwner => (),
        Mechanic::ResetAI => (),
        Mechanic::RotateTowards { target } => (),
        Mechanic::RunAIGoalSelector => (),
        Mechanic::RunAITargetSelector => (),
        Mechanic::Saddle => (),
        Mechanic::SendActionMessage { message } => (),
        Mechanic::SendResourcePack { url } => (),
        Mechanic::SendTitle {
            title,
            subtitle,
            fade_in,
            stay,
            fade_out,
        } => (),
        Mechanic::SendToast { title, message } => (),
        Mechanic::SetAI { ai } => (),
        Mechanic::SetBlockOpen { open } => (),
        Mechanic::SetBlockType { block } => (),
        Mechanic::SetChunkForceLoaded { loaded } => (),
        Mechanic::SetCollidable { collidable } => (),
        Mechanic::SetDragonPodium { podium } => (),
        Mechanic::SetGameMode { gamemode } => (),
        Mechanic::SetGliding { gliding } => (),
        Mechanic::SetGlobalScore {
            objective,
            action,
            value,
        } => (),
        Mechanic::SetGravity { gravity } => (),
        Mechanic::SetHealth { health } => (),
        Mechanic::SetInteractionSize { width, height } => (),
        Mechanic::SetItemGroupCooldown { group, cooldown } => (),
        Mechanic::SetDisplayEntityItem { item } => (),
        Mechanic::SetLeashHolder { holder } => (),
        Mechanic::SetLevel { level } => (),
        Mechanic::SetMaterialCooldown { material, cooldown } => (),
        Mechanic::SetMaxHealth { health } => (),
        Mechanic::SetMobColor { color } => (),
        Mechanic::SetMobScore { objective, score } => (),
        Mechanic::SetName { name } => (),
        Mechanic::SetRaiderCanJoinRaid { can_join } => (),
        Mechanic::SetRaiderPatrolBlock { block } => (),
        Mechanic::SetRaiderPatrolLeader { leader } => (),
        Mechanic::SetFaction { faction } => (),
        Mechanic::SetFlying { flying } => (),
        Mechanic::SetNoDamageTicks { ticks } => (),
        Mechanic::SetOwner { owner } => (),
        Mechanic::SetParent { parent } => (),
        Mechanic::SetPathfindingMalus { malus } => (),
        Mechanic::SetPitch { pitch } => (),
        Mechanic::SetPose { pose } => (),
        Mechanic::SetRotation { yaw, pitch } => (),
        Mechanic::SetTarget { target } => (),
        Mechanic::SetTargetScore { objective, score } => (),
        Mechanic::SetTextDisplay { text } => (),
        Mechanic::SetTongueTarget { target } => (),
        Mechanic::SetScore { objective, score } => (),
        Mechanic::SetSpeed { speed } => (),
        Mechanic::SetStance { stance } => (),
        Mechanic::Shield => (),
        Mechanic::ShieldBreak => (),
        Mechanic::ShieldPercent { percent } => (),
        Mechanic::ShootFireball { velocity } => (),
        Mechanic::ShootPotion { potion, velocity } => (),
        Mechanic::ShootSkull { velocity } => (),
        Mechanic::ShootShulkerBullet { velocity } => (),
        Mechanic::ShowEntity { entity } => (),
        Mechanic::Signal { signal } => (),
        Mechanic::Skybox { skybox } => (),
        Mechanic::Smoke => (),
        Mechanic::SmokeSwirl => (),
        Mechanic::Sound {
            sound,
            volume,
            pitch,
        } => (),
        Mechanic::StealItem { item } => (),
        Mechanic::StopSound { sound } => (),
        Mechanic::StopSoundWithCategory {
            sound,
            sound_category,
        } => (),
        Mechanic::Speak {
            offset,
            radius,
            max_line_lenght,
            line_prefix,
            message,
            chat_prefix,
            duration,
            send_chat_message,
        } => (),
        Mechanic::Spin { velocity, aura } => (),
        Mechanic::Spring {
            spring_type,
            duration,
        } => (),
        Mechanic::Stun { duration } => (),
        Mechanic::StopUsingItem => (),
        Mechanic::Suicide => (),
        Mechanic::Summon { mob, location } => (),
        Mechanic::SummonAreaEffectCloud {
            particle,
            effect_type,
            potion_duration,
            level,
            duration,
            duration_reduction_on_use,
            radius,
            radius_reduction_on_use,
            radius_reduction_on_tick,
        } => (),
        Mechanic::SummonFallingBlock { material } => (),
        Mechanic::SummonPassenger { passenger, stack } => (),
        Mechanic::Swap => (),
        Mechanic::SwingOffHand => (),
        Mechanic::AddTag(_) => (),
        Mechanic::RemoveTag(_) => (),
        Mechanic::TakeItem {
            item,
            amount,
            exact,
            vanilla_only,
        } => (),
        Mechanic::Taunt => (),
        Mechanic::Teleport {
            spreadh,
            spreadv,
            preserve_pitch,
            preserve_yaw,
            safe_teleport,
        } => (),
        Mechanic::TeleportY { y } => (),
        Mechanic::TeleportIn {
            vector,
            yaw,
            target_as_origin,
        } => (),
        Mechanic::TeleportTo {
            location,
            world,
            yaw,
            pitch,
            relative,
            target_as_origin,
        } => (),
        Mechanic::Time {
            mode,
            amount,
            personal,
            relative,
        } => (),
        Mechanic::Threat { amount, mode } => (),
        Mechanic::Throw {
            velocity,
            velocity_y,
            from_origin,
        } => (),
        Mechanic::ThunderLevel { level } => (),
        Mechanic::ToggleLever {
            location,
            duration,
            x,
            y,
            z,
        } => (),
        Mechanic::TogglePiston => (),
        Mechanic::ToggleSitting(_) => (),
        Mechanic::TotemOfUndying { model } => (),
        Mechanic::TrackLocation => (),
        Mechanic::UndoPaste { paste_id } => (),
        Mechanic::Velocity {
            mode,
            velocity_x,
            velocity_y,
            velocity_z,
            relative,
        } => (),
        Mechanic::WolfSit(_) => (),
        Mechanic::WorldEditReplace { from, to } => (),
        Mechanic::Weather {
            weather_type,
            duration,
        } => (),
        Mechanic::AdditionalMecanics { mechanic_type } => (),
    }
}
