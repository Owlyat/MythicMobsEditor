use states::AppState;

mod skill;
mod states;
mod ui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    let state = AppState::default();
    eframe::run_native(
        "MythicMobs Skill Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(ui::Ui::new(cc, state)))),
    )
}
