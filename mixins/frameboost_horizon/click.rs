fn frameboost_horizon_click(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    button: usize,
) {
    use crate::game::frameboost_horizon as m;
    use horizon_client::graphics::{self, Preset};

    if mod_id != m::ID {
        return;
    }

    match button {
        0 => {
            graphics::apply_preset(Preset::Performance, &mut game.settings);
            game.settings.horizon.lod_distance = 0;
            game.settings.vsync = false;
            game.settings.max_fps = 260;
            game.settings.horizon.battery = false;
        }
        1 => {
            graphics::apply_preset(Preset::Performance, &mut game.settings);
            game.settings.render_distance = 6;
            game.settings.simulation_distance = 5;
            game.settings.horizon.preset = Preset::Custom;
            game.settings.horizon.lod_distance = 0;
            game.settings.vsync = false;
            game.settings.max_fps = 260;
            game.settings.horizon.battery = false;
        }
        2 => {
            graphics::apply_preset(Preset::Balanced, &mut game.settings);
            game.settings.horizon.lod_distance = 0;
        }
        _ => return,
    }

    game.apply_settings();
    game.settings.save(&crate::game::Game::options_path());
}
