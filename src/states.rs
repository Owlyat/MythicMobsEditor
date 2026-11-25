use crate::skill::{
    Skill,
    mechanics::{MythicOption, Xyz},
    minecraft_lib::MinecraftMob,
    targeters::Targeter,
    triggers::Trigger,
};
use strum::VariantArray;

#[derive(Default)]
pub struct AppState {
    pub mob_name: String,
    pub mob_display_name: String,
    pub mob_type: MinecraftMob,
    pub mob_health: u32,
    pub damage: u8,
    pub armor: u8,
    pub selected_skill: Option<usize>,
    pub skills: Vec<Skill>,
    pub output: String,
    pub mob_type_filter: String,
    pub locale: Locale,
    /// `(is_error, message)` shown after an import/export/copy action.
    pub status_message: Option<(bool, String)>,
}

#[derive(strum::Display, Default, PartialEq, strum::EnumIter, Clone)]
pub enum Locale {
    #[default]
    #[strum(to_string = "en")]
    EN,
    #[strum(to_string = "fr")]
    FR,
}

impl AppState {
    /// Replace the whitespaces to "_" and removes \n
    pub fn sanitize_mob_name(&mut self) {
        if self.mob_name.is_empty() {
            if let MinecraftMob::MetaSkill = self.mob_type {
                self.mob_name = "Default_Skill_Name".into();
            } else {
                self.mob_name = "Default_Mob_Name".into();
            }
        }
        self.mob_name = self.mob_name.trim().replace(" ", "_");
    }
    /// Append a default skill to self.skills and select it
    pub fn add_skill(&mut self) {
        self.skills
            .push(Skill::default().set_name(format!("Skill_{}", self.skills.len())));
        self.select_last_skill();
    }
    pub fn get_selected_skill(&mut self) -> Option<&mut Skill> {
        if let Some(id) = self.selected_skill {
            return self.skills.get_mut(id);
        }
        None
    }
    /// Rename Selected Skill(self.selected_skill) name
    pub fn rename_selected_skill(&mut self, name: impl Into<String>) {
        if let Some(id) = self.selected_skill {
            if let Some(sk) = self.skills.get_mut(id) {
                sk.rename(name);
            }
        }
    }
    /// Remove the Selected Skill(self.selected_skill) from self.skills
    pub fn remove_selected_skill(&mut self) {
        if let Some(id) = self.selected_skill {
            if self.skills.get(id).is_some() {
                self.skills.remove(id);
            }
        }
    }
    /// set the selected skill to the first one in self.skills
    /// if empty, set to None
    pub fn select_first_skill(&mut self) {
        if self.skills.is_empty() {
            self.selected_skill = None;
            return;
        }
        self.select_skill(0);
    }
    /// set the selected skill to the last one in self.skills
    /// if empty, set to None
    pub fn select_last_skill(&mut self) {
        if self.skills.is_empty() {
            self.selected_skill = None;
            return;
        }
        self.select_skill(self.skills.len() - 1);
    }
    /// remove skill from self.skills at index
    pub fn remove_skill(&mut self, id: usize) {
        if (0..=self.skills.len() - 1).contains(&id) && self.skills.get(id).is_some() {
            self.skills.remove(id);
        }
    }
    pub fn select_skill(&mut self, id: usize) {
        if self.skills.is_empty() {
            self.selected_skill = None;
            return;
        }
        if (0..=self.skills.len() - 1).contains(&id) {
            self.selected_skill = Some(id);
        }
    }
    /// Import a MythicMobs mob/skill YAML file.
    ///
    /// Metadata is parsed; skill lines are kept verbatim as raw skills so they
    /// can be edited and re-exported without losing information.
    pub fn import_config(&mut self, text: &str) -> Result<(), String> {
        let root: serde_norway::Value =
            serde_norway::from_str(text).map_err(|err| err.to_string())?;

        let entries = root
            .as_mapping()
            .ok_or_else(|| "The file must contain a YAML mapping".to_string())?;
        let (name_value, config) = entries
            .iter()
            .next()
            .ok_or_else(|| "The file does not contain a mob or skill".to_string())?;

        let name = name_value
            .as_str()
            .ok_or_else(|| "The mob/skill name must be a string".to_string())?;
        if !config.is_mapping() {
            return Err("The mob/skill configuration must be a mapping".to_string());
        }

        self.mob_name = name.to_owned();
        self.mob_display_name = config
            .get("Display")
            .and_then(serde_norway::Value::as_str)
            .unwrap_or("")
            .to_owned();
        self.mob_type = match config.get("Type").and_then(serde_norway::Value::as_str) {
            Some(value) => {
                parse_mob_type(value).ok_or_else(|| format!("Unknown mob type: {value}"))?
            }
            None => MinecraftMob::MetaSkill,
        };
        self.mob_health = read_u32(config, "Health").unwrap_or(0);
        self.damage = read_u8(config, "Damage").unwrap_or(0);
        self.armor = read_u8(config, "Armor").unwrap_or(0);

        self.skills.clear();
        if let Some(skills) = config.get("Skills") {
            let list = skills
                .as_sequence()
                .ok_or_else(|| "Skills must be a list".to_string())?;
            for (index, item) in list.iter().enumerate() {
                let line = item
                    .as_str()
                    .ok_or_else(|| "Each skill must be a string".to_string())?;
                let line = line.trim().strip_prefix("- ").unwrap_or(line.trim());
                self.skills.push(Skill {
                    name: format!("Skill_{index}"),
                    mechanic: None,
                    raw_args: line.to_owned(),
                    targeter: Targeter::None,
                    trigger: Trigger::None,
                });
            }
        }

        self.mob_type_filter.clear();
        self.select_first_skill();
        self.create_config();
        Ok(())
    }

    pub fn create_config(&mut self) {        self.sanitize_mob_name();
        self.output = format!(
            "{}:{}{}{}{}{}{}{}{}{}{}{}{}",
            self.mob_name,
            if let MinecraftMob::MetaSkill = self.mob_type {
                ""
            } else {
                "\n  Type: "
            },
            if let MinecraftMob::MetaSkill = self.mob_type {
                "".to_string()
            } else {
                self.mob_type.to_string()
            },
            if self.mob_display_name.is_empty() {
                ""
            } else {
                "\n  Display: "
            },
            self.mob_display_name,
            if self.mob_health == 0 {
                ""
            } else {
                "\n  Health: "
            },
            if self.mob_health == 0 {
                "".to_owned()
            } else {
                self.mob_health.to_string()
            },
            if self.damage == 0 { "" } else { "\n  Damage: " },
            if self.damage == 0 {
                "".to_owned()
            } else {
                self.damage.to_string()
            },
            if self.armor == 0 { "" } else { "\n  Armor: " },
            if self.armor == 0 {
                "".to_owned()
            } else {
                self.armor.to_string()
            },
            if self.skills.is_empty() {
                ""
            } else {
                "\n  Skills:"
            },
            // SKILLS
            self.handle_skill_formating(),
        );
    }
    pub fn handle_skill_formating(&self) -> String {
        let mut skills_fmt = String::new();
        self.skills.iter().for_each(|sk| {
            if let Some(mech) = &sk.mechanic {
                skills_fmt.push_str(&format!(
                    "\n  {}{}{}",
                    match mech {
                        crate::skill::mechanics::Mechanic::ToggleLever {
                            location,
                            duration,
                            x,
                            y,
                            z,
                        } => {
                            if location.clone().some().is_none()
                                && x.clone().some().is_none()
                                    | y.clone().some().is_none()
                                    | z.clone().some().is_none()
                            {
                                let location = if location.clone().some().is_none() {
                                    let mut xyz = Xyz::default();
                                    if let Some((_, x, _)) = x.clone().some() {
                                        xyz.x = *x;
                                    }
                                    if let Some((_, y, _)) = y.clone().some() {
                                        xyz.y = *y;
                                    }
                                    if let Some((_, z, _)) = z.clone().some() {
                                        xyz.z = *z;
                                    }
                                    MythicOption::Some("".to_string(), xyz, "".to_string())
                                } else {
                                    location.clone()
                                };
                                crate::skill::mechanics::Mechanic::ToggleLever {
                                    location,
                                    duration: *duration,
                                    x: x.clone(),
                                    y: y.clone(),
                                    z: z.clone(),
                                }
                            } else {
                                mech.clone()
                            }
                        }
                        _ => mech.clone(),
                    },
                    if let Targeter::None = sk.targeter {
                        "".to_owned()
                    } else {
                        format!(" {}", sk.targeter)
                    },
                    if let Trigger::None = sk.trigger {
                        "".to_owned()
                    } else {
                        format!(" {}", sk.trigger)
                    },
                ));
            } else if !sk.raw_args.trim().is_empty() {
                let raw = sk
                    .raw_args
                    .trim()
                    .strip_prefix("- ")
                    .unwrap_or(sk.raw_args.trim());
                let has_targeter = raw.split_whitespace().any(|token| token.starts_with('@'));
                let has_trigger = raw.split_whitespace().any(|token| token.starts_with('~'));
                skills_fmt.push_str(&format!(
                    "\n  - {}{}{}",
                    raw,
                    if has_targeter || matches!(sk.targeter, Targeter::None) {
                        "".to_owned()
                    } else {
                        format!(" {}", sk.targeter)
                    },
                    if has_trigger || matches!(sk.trigger, Trigger::None) {
                        "".to_owned()
                    } else {
                        format!(" {}", sk.trigger)
                    },
                ));
            }
        });
        skills_fmt
    }
}

fn parse_mob_type(value: &str) -> Option<MinecraftMob> {
    value.parse::<MinecraftMob>().ok().or_else(|| {
        MinecraftMob::VARIANTS
            .iter()
            .find(|mob| mob.to_string().eq_ignore_ascii_case(value))
            .cloned()
    })
}

fn read_u32(config: &serde_norway::Value, key: &str) -> Option<u32> {
    config
        .get(key)
        .and_then(serde_norway::Value::as_u64)
        .and_then(|value| value.try_into().ok())
}

fn read_u8(config: &serde_norway::Value, key: &str) -> Option<u8> {
    config
        .get(key)
        .and_then(serde_norway::Value::as_u64)
        .and_then(|value| value.try_into().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports_mob_metadata_and_raw_skills() {
        let mut state = AppState::default();
        let yaml = "TestMob:\n  Type: ZOMBIE\n  Display: '&cTest'\n  Health: 100\n  Damage: 5\n  Armor: 2\n  Skills:\n  - damage{a=10} @target ~onAttack\n  - skill{s=Foo} @self ~onTimer:100\n";

        state.import_config(yaml).unwrap();

        assert_eq!(state.mob_name, "TestMob");
        assert_eq!(state.mob_display_name, "&cTest");
        assert!(matches!(state.mob_type, MinecraftMob::Zombie));
        assert_eq!(state.mob_health, 100);
        assert_eq!(state.damage, 5);
        assert_eq!(state.armor, 2);
        assert_eq!(state.skills.len(), 2);
        assert_eq!(state.skills[0].raw_args, "damage{a=10} @target ~onAttack");
        assert!(state.output.contains("damage{a=10} @target ~onAttack"));
    }

    #[test]
    fn imports_metaskill_without_type() {
        let mut state = AppState::default();
        let yaml = "MySkill:\n  Skills:\n  - message{m=hi}\n";

        state.import_config(yaml).unwrap();

        assert!(matches!(state.mob_type, MinecraftMob::MetaSkill));
        assert_eq!(state.skills.len(), 1);
        assert_eq!(state.skills[0].raw_args, "message{m=hi}");
        assert!(state.output.contains("message{m=hi}"));
    }

    #[test]
    fn rejects_invalid_yaml() {
        let mut state = AppState::default();
        assert!(state.import_config("not a mapping").is_err());
    }
}
