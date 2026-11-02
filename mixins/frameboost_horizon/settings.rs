fn frameboost_horizon_settings(mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::frameboost_horizon as m;

    if mod_id != m::ID {
        return;
    }

    buttons.extend([
        "Leistung anwenden (8 Chunks)".to_owned(),
        "Maximale FPS anwenden (6 Chunks)".to_owned(),
        "Ausgewogen anwenden (12 Chunks)".to_owned(),
    ]);
}
