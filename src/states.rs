use crate::skill::{Skill, minecraft_lib::MinecraftMob, targeters::Targeter, triggers::Trigger};

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
    pub fn create_config(&mut self) {
        self.sanitize_mob_name();
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
                    mech,
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
            }
        });
        skills_fmt
    }
}
