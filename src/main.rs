use bevy::prelude::*;
use cosmic_text::FontSystem;

mod components;
mod resources;
mod ui;
mod utils;
mod system;
mod ui_fonts;
mod ui_icons;
mod powerups;
mod upgrades;
mod achievements;

use system::*;
use crate::resources::{load_or_create_game_state, ClickPower, ComboSystem, GoldenCookieTimer, SaveTimer};
use crate::ui::{achievement_popup_system, mouse_scroll};

const BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.08);

fn main() {
    let save = load_or_create_game_state();
    let mut font = FontSystem::new();
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🍪 COOKIE EMPIRE DELUXE 🍪".into(),
                resolution: (1200.0, 800.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(save.clone())
        .insert_resource(SaveTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .insert_resource(ClickPower(save.cookies_per_click))
        .insert_resource(GoldenCookieTimer::default())
        .insert_resource(ComboSystem {
            clicks: 0,
            combo: 0,
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            active: false,
        })
        //.insert_resource(AchievementList::new())
        .add_systems(PreStartup, ui_fonts::load_fonts)
        .add_systems(Startup, ui::setup_ui)
        .add_systems(
            Update,
            (
                cookie_click_system,
                passive_income_system,
                update_ui_system,
                upgrade_button_system,
                powerup_button_system,
                prestige_button_system,
                auto_save_system,
                animate_popup_system,
                cleanup_popup_system,
                animate_cookie_system,
                golden_cookie_spawn_system,
                golden_cookie_lifetime_system,
                golden_cookie_click_system,
                //golden_cookie_cleanup_system,
                combo_system,
                check_achievements_system,
                particle_system,
                particle_cleanup_system,
                milestone_system,
                mouse_scroll,
                update_stats_system,
            ),
        )
        .add_systems(Update, achievement_popup_system)
        .run();
}
