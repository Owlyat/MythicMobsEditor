use states::AppState;

#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales", fallback = "en");

mod i18n;
mod skill;
mod states;
mod ui;

fn main() -> eframe::Result<()> {
    rust_i18n::set_locale("en");
    let native_options = eframe::NativeOptions::default();
    let state = AppState::default();
    eframe::run_native(
        "MythicMobs Skill Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::Ui::new(cc, state)))),
    )
}
