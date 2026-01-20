use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const SAVE_FILE: &str = "cookie_save.json";

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub cookies: u64,
    pub total_cookies_earned: u64,
    pub cookies_per_second: f64,
    pub cookies_per_click: u64,
    pub upgrades: Vec<Upgrade>,
    pub powerups: Vec<PowerUp>,
    pub prestige_level: u32,
    pub prestige_points: u64,
    pub lifetime_cookies: u64,
    pub click_count: u64,
    pub golden_cookies_clicked: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub name: String,
    pub emoji: String,
    pub base_cost: u64,
    pub cost: u64,
    pub cps: f64,
    pub count: u32,
    pub description: String,
    pub tier: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowerUp {
    pub name: String,
    pub emoji: String,
    pub base_cost: u64,
    pub cost: u64,
    pub multiplier: u64,
    pub count: u32,
    pub description: String,
}

impl Upgrade {
    pub fn calculate_cost(&self) -> u64 {
        let base = self.base_cost as f64 * 1.15_f64.powi(self.count as i32);
        (base * (1.0 - (self.tier as f64 * 0.01))) as u64
    }
}

impl PowerUp {
    pub fn calculate_cost(&self) -> u64 {
        (self.base_cost as f64 * 1.2_f64.powi(self.count as i32)) as u64
    }
}

#[derive(Resource)]
pub struct SaveTimer(pub Timer);

#[derive(Resource)]
pub struct ClickPower(pub u64);

#[derive(Resource)]
pub struct GoldenCookieTimer(pub Timer);

#[derive(Resource)]
pub struct ComboSystem {
    pub clicks: u32,
    pub combo: u32,
    pub active: bool,
    pub timer: Timer,
}

#[derive(Resource, Clone)]
pub struct AchievementList {
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<bool>,
}

#[derive(Clone)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub requirement: AchievementRequirement,
}

#[derive(Clone)]
pub enum AchievementRequirement {
    TotalCookies(u64),
    CookiesPerSecond(u64),
    Clicks(u64),
    GoldenCookies(u32),
    BuildingCount(usize, u32),
    PrestigeLevel(u32),
}

impl AchievementList {
    pub fn new() -> Self {
        let achievements = vec![
            Achievement {
                name: "Premier cookie".into(),
                description: "Cliquez votre premier cookie".into(),
                emoji: "🍪".into(),
                requirement: AchievementRequirement::TotalCookies(1),
            },
            Achievement {
                name: "Centenaire".into(),
                description: "Gagnez 100 cookies".into(),
                emoji: "💯".into(),
                requirement: AchievementRequirement::TotalCookies(100),
            },
            Achievement {
                name: "Millionnaire".into(),
                description: "Gagnez 1 million de cookies".into(),
                emoji: "💰".into(),
                requirement: AchievementRequirement::TotalCookies(1_000_000),
            },
            Achievement {
                name: "Milliardaire".into(),
                description: "Gagnez 1 milliard de cookies".into(),
                emoji: "🏆".into(),
                requirement: AchievementRequirement::TotalCookies(1_000_000_000),
            },
            Achievement {
                name: "Clicker Pro".into(),
                description: "Cliquez 1000 fois".into(),
                emoji: "👆".into(),
                requirement: AchievementRequirement::Clicks(1000),
            },
            Achievement {
                name: "Chasseur d'or".into(),
                description: "Cliquez 10 golden cookies".into(),
                emoji: "✨".into(),
                requirement: AchievementRequirement::GoldenCookies(10),
            },
            Achievement {
                name: "Usine à cookies".into(),
                description: "Produisez 1000 cookies/sec".into(),
                emoji: "🏭".into(),
                requirement: AchievementRequirement::CookiesPerSecond(1000),
            },
            Achievement {
                name: "Ascension".into(),
                description: "Atteignez le prestige niveau 1".into(),
                emoji: "⭐".into(),
                requirement: AchievementRequirement::PrestigeLevel(1),
            },
        ];

        let unlocked = vec![false; achievements.len()];
        Self { achievements, unlocked }
    }
}

pub fn load_or_create_game_state() -> GameState {
    let save_path = PathBuf::from(SAVE_FILE);

    if save_path.exists() {
        if let Ok(data) = fs::read_to_string(&save_path) {
            if let Ok(mut state) = serde_json::from_str::<GameState>(&data) {
                // state.cookies_per_second = 0.0;
                // state.cookies_per_click = 1;

                for upgrade in &mut state.upgrades {
                    upgrade.cost = upgrade.calculate_cost();
                    let multiplier = 1.0 + (state.prestige_level as f64 * 0.01);
                    state.cookies_per_second += upgrade.cps * upgrade.count as f64 * multiplier;
                }

                for powerup in &mut state.powerups {
                    powerup.cost = powerup.calculate_cost();
                    state.cookies_per_click += powerup.multiplier * powerup.count as u64;
                }

                println!("💾 Sauvegarde chargée : {} cookies, {} CPS, {} CPC", state.cookies, state.cookies_per_second, state.cookies_per_click);
                return state;
            }
        }
    }

    println!("✨ Nouvelle partie créée");
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
        upgrades: vec![
            Upgrade {
                name: "Curseur".into(),
                emoji: "☝️".into(),
                base_cost: 15,
                cost: 15,
                cps: 0.1,
                count: 0,
                tier: 0,
                description: "Clique automatiquement".into(),
            },
            Upgrade {
                name: "Grand-mère".into(),
                emoji: "👵".into(),
                base_cost: 100,
                cost: 100,
                cps: 1.0,
                count: 0,
                tier: 0,
                description: "Cuisine des cookies maison".into(),
            },
            Upgrade {
                name: "Ferme".into(),
                emoji: "🌾".into(),
                base_cost: 1100,
                cost: 1100,
                cps: 8.0,
                count: 0,
                tier: 0,
                description: "Cultive des ingrédients".into(),
            },
            Upgrade {
                name: "Mine".into(),
                emoji: "⛏️".into(),
                base_cost: 12000,
                cost: 12000,
                cps: 47.0,
                count: 0,
                tier: 0,
                description: "Extrait du sucre cristallisé".into(),
            },
            Upgrade {
                name: "Usine".into(),
                emoji: "🏭".into(),
                base_cost: 130000,
                cost: 130000,
                cps: 260.0,
                count: 0,
                tier: 0,
                description: "Production industrielle".into(),
            },
            Upgrade {
                name: "Banque".into(),
                emoji: "🏦".into(),
                base_cost: 1400000,
                cost: 1400000,
                cps: 1400.0,
                count: 0,
                tier: 0,
                description: "Investit dans les cookies".into(),
            },
            Upgrade {
                name: "Temple".into(),
                emoji: "⛩️".into(),
                base_cost: 20000000,
                cost: 20000000,
                cps: 7800.0,
                count: 0,
                tier: 0,
                description: "Invoque des cookies divins".into(),
            },
            Upgrade {
                name: "Tour de magie".into(),
                emoji: "🔮".into(),
                base_cost: 330000000,
                cost: 330000000,
                cps: 44000.0,
                count: 0,
                tier: 0,
                description: "Transmute en cookies".into(),
            },
            Upgrade {
                name: "Portail".into(),
                emoji: "🌀".into(),
                base_cost: 5100000000,
                cost: 5100000000,
                cps: 260000.0,
                count: 0,
                tier: 0,
                description: "Import interdimensionnel".into(),
            },
            Upgrade {
                name: "Machine temporelle".into(),
                emoji: "⏰".into(),
                base_cost: 75000000000,
                cost: 75000000000,
                cps: 1600000.0,
                count: 0,
                tier: 0,
                description: "Cookies du futur".into(),
            },
            Upgrade {
                name: "Condensateur".into(),
                emoji: "⚛️".into(),
                base_cost: 1000000000000,
                cost: 1000000000000,
                cps: 10000000.0,
                count: 0,
                tier: 0,
                description: "Compresse la matière".into(),
            },
            Upgrade {
                name: "Prisme".into(),
                emoji: "🌈".into(),
                base_cost: 14000000000000,
                cost: 14000000000000,
                cps: 65000000.0,
                count: 0,
                tier: 0,
                description: "Convertit la lumière".into(),
            },
            Upgrade {
                name: "Chancemaker".into(),
                emoji: "🎰".into(),
                base_cost: 170000000000000,
                cost: 170000000000000,
                cps: 430000000.0,
                count: 0,
                tier: 0,
                description: "Manipule la probabilité".into(),
            },
            Upgrade {
                name: "Fractale".into(),
                emoji: "📐".into(),
                base_cost: 2100000000000000,
                cost: 2100000000000000,
                cps: 2900000000.0,
                count: 0,
                tier: 0,
                description: "Cookies infinis".into(),
            },
            Upgrade {
                name: "Console JS".into(),
                emoji: "💻".into(),
                base_cost: 26000000000000000,
                cost: 26000000000000000,
                cps: 21000000000.0,
                count: 0,
                tier: 0,
                description: "Hack la réalité".into(),
            },
        ],
        powerups: vec![
            PowerUp {
                name: "Clic renforcé".into(),
                emoji: "👆".into(),
                base_cost: 100,
                cost: 100,
                multiplier: 1,
                count: 0,
                description: "+1 cookie par clic".into(),
            },
            PowerUp {
                name: "Doigts dorés".into(),
                emoji: "✨".into(),
                base_cost: 500,
                cost: 500,
                multiplier: 5,
                count: 0,
                description: "+5 cookies par clic".into(),
            },
            PowerUp {
                name: "Main bénie".into(),
                emoji: "🙏".into(),
                base_cost: 5000,
                cost: 5000,
                multiplier: 25,
                count: 0,
                description: "+25 cookies par clic".into(),
            },
            PowerUp {
                name: "Bras cybernétique".into(),
                emoji: "🦾".into(),
                base_cost: 50000,
                cost: 50000,
                multiplier: 100,
                count: 0,
                description: "+100 cookies par clic".into(),
            },
            PowerUp {
                name: "Main divine".into(),
                emoji: "👼".into(),
                base_cost: 500000,
                cost: 500000,
                multiplier: 500,
                count: 0,
                description: "+500 cookies par clic".into(),
            },
            PowerUp {
                name: "Clic cosmique".into(),
                emoji: "🌟".into(),
                base_cost: 5000000,
                cost: 5000000,
                multiplier: 2500,
                count: 0,
                description: "+2500 cookies par clic".into(),
            },
        ],
    }
}

pub fn save_game_state(game_state: &GameState) {
    if let Ok(data) = serde_json::to_string_pretty(game_state) {
        if let Err(e) = fs::write(SAVE_FILE, data) {
            eprintln!("❌ Erreur de sauvegarde : {}", e);
        }
    }
}