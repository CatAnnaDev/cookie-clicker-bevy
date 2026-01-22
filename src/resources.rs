use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::achievements::{get_achievements, AchievementList};
use crate::powerups::{get_powerups, PowerUp};
use crate::system::PRESTIGE_BUFF;
use crate::upgrades::{get_upgrade, Upgrade};
use crate::utils::random_spawn_time;

const SAVE_FILE: &str = "cookie_save.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub cookies: u128,
    pub total_cookies_earned: u128,
    pub cookies_per_second: f64,
    pub cookies_per_click: u128,
    pub upgrades: Vec<Upgrade>,
    pub powerups: Vec<PowerUp>,
    pub prestige_level: u128,
    pub prestige_points: u128,
    pub lifetime_cookies: u128,
    pub click_count: u128,
    pub golden_cookies_clicked: u128,
    pub achievements: AchievementList,
    pub combo: u128,
    pub cps_buffer: f64,
}

impl Default for GameState {
    fn default() -> Self {
        Self{
            cookies: 0,
            total_cookies_earned: 0,
            cookies_per_second: 0.0,
            cookies_per_click: 0,
            upgrades: get_upgrade(),
            powerups: get_powerups(),
            prestige_level: 0,
            prestige_points: 0,
            lifetime_cookies: 0,
            click_count: 0,
            golden_cookies_clicked: 0,
            achievements: get_achievements(),
            combo: 0,
            cps_buffer: 0.0,
        }
    }
}


#[derive(Resource)]
pub struct SaveTimer(pub Timer);

#[derive(Resource)]
pub struct ClickPower(pub u128);

#[derive(Resource)]
pub struct GoldenCookieTimer(pub Timer);

impl Default for GoldenCookieTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(random_spawn_time(), TimerMode::Once))
    }
}

#[derive(Resource)]
pub struct ComboSystem {
    pub clicks: u128,
    pub combo: u128,
    pub active: bool,
    pub timer: Timer,
}

pub fn load_or_create_game_state() -> GameState {
    let save_path = PathBuf::from(SAVE_FILE);

    if save_path.exists() {
        if let Ok(data) = fs::read_to_string(&save_path) {
            if let Ok(saved_state) = serde_json::from_str::<GameState>(&data) {
                let mut fresh_state = GameState::default();

                fresh_state.cookies = saved_state.cookies;
                fresh_state.prestige_level = saved_state.prestige_level;
                fresh_state.total_cookies_earned = saved_state.total_cookies_earned;
                fresh_state.click_count = saved_state.click_count;
                fresh_state.golden_cookies_clicked = saved_state.golden_cookies_clicked;
                fresh_state.achievements = saved_state.achievements;
                fresh_state.cookies_per_second = 1.0;
                fresh_state.cookies_per_click = 1;


                for saved_upgrade in &saved_state.upgrades {
                    if let Some(fresh_upgrade) = fresh_state.upgrades.iter_mut()
                        .find(|u| u.name == saved_upgrade.name) {
                        fresh_upgrade.count = saved_upgrade.count;
                        fresh_upgrade.cost = fresh_upgrade.calculate_cost();
                        let multiplier = 1.0 + (fresh_state.prestige_level as f64 * PRESTIGE_BUFF);
                        fresh_state.cookies_per_second += fresh_upgrade.cps * fresh_upgrade.count as f64 * multiplier;
                    }
                }

                for saved_powerup in &saved_state.powerups {
                    if let Some(fresh_powerup) = fresh_state.powerups.iter_mut()
                        .find(|p| p.name == saved_powerup.name) {
                        fresh_powerup.count = saved_powerup.count;
                        fresh_powerup.cost = fresh_powerup.calculate_cost();
                        fresh_state.cookies_per_click += fresh_powerup.multiplier * fresh_powerup.count;
                    }
                }


                println!("💾 Sauvegarde chargée et mise à jour : {} cookies, {} CPS, {} CPC",
                         fresh_state.cookies, fresh_state.cookies_per_second, fresh_state.cookies_per_click);
                println!("✨ {} nouveaux upgrades disponibles !",
                         fresh_state.upgrades.len() - saved_state.upgrades.len());
                println!("✨ {} nouveaux powerups disponibles !",
                         fresh_state.powerups.len() - saved_state.powerups.len());

                return fresh_state;
            }
        }
    }

    let powerups = get_powerups();
    let upgrades = get_upgrade();
    let achievements = get_achievements();

    println!("✨ Nouvelle partie créée: {} powerups load {} builds load", powerups.len(), upgrades.len());
    GameState {
        cookies: 0,
        total_cookies_earned: 0,
        cookies_per_second: 0.0,
        cookies_per_click: 1,
        click_count: 0,
        golden_cookies_clicked: 0,
        prestige_level: 0,
        prestige_points: 0,
        lifetime_cookies: 0,
        upgrades,
        powerups,
        achievements,
        combo: 0,
        cps_buffer: 0.0,
    }
}







pub fn save_game_state(game_state: &GameState) {
    if let Ok(data) = serde_json::to_string_pretty(game_state) {
        if let Err(e) = fs::write(SAVE_FILE, data) {
            eprintln!("❌ Erreur de sauvegarde : {}", e);
        }
    }
}