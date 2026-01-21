use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Upgrade {
    pub name: String,
    pub emoji: String,
    pub base_cost: u128,
    pub cost: u128,
    pub cps: f64,
    pub count: u128,
    pub description: String,
    pub tier: u128,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowerUp {
    pub name: String,
    pub emoji: String,
    pub base_cost: u128,
    pub cost: u128,
    pub multiplier: u128,
    pub count: u128,
    pub description: String,
}

impl Upgrade {
    pub fn calculate_cost(&self) -> u128 {
        let base = self.base_cost as f64 * 1.15_f64.powi(self.count as i32);
        (base * (1.0 - (self.tier as f64 * 0.01))) as u128
    }
}

impl PowerUp {
    pub fn calculate_cost(&self) -> u128 {
        (self.base_cost as f64 * 1.2_f64.powi(self.count as i32)) as u128
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

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct AchievementList {
    pub achievements: Vec<Achievement>,
    pub unlocked: Vec<bool>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub name: String,
    pub description: String,
    pub emoji: String,
    pub requirement: AchievementRequirement,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AchievementRequirement {
    TotalCookies(u128),
    CookiesPerSecond(u128),
    Clicks(u128),
    GoldenCookies(u128),
    BuildingCount(usize, u128),
    PrestigeLevel(u128),
    Combos(u128),
}
fn ach(name: &str, desc: &str, req: AchievementRequirement) -> Achievement {
    Achievement {
        name: name.into(),
        description: desc.into(),
        emoji: "".into(),
        requirement: req,
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
                    state.cookies_per_click += powerup.multiplier * powerup.count;
                }

                println!("💾 Sauvegarde chargée : {} cookies, {} CPS, {} CPC", state.cookies, state.cookies_per_second, state.cookies_per_click);
                return state;
            }
        }
    }

    let powerups = vec![
        pu("Clic renforcé", 100, 1),
        pu("Double clic", 500, 2),
        pu("Triple frappe", 2_000, 5),
        pu("Clic furieux", 10_000, 10),
        pu("Doigt d'acier", 50_000, 25),
        pu("Bras cybernétique", 250_000, 50),
        pu("Main divine", 1_000_000, 100),
        pu("Clic cosmique", 5_000_000, 250),
        pu("Doigt quantique", 20_000_000, 500),
        pu("Main fractale", 100_000_000, 1_000),
        pu("Hyper clic", 500_000_000, 2_500),
        pu("Clic critique", 1_000_000_000, 5_000),
        pu("Clic instable", 2_500_000_000, 10_000),
        pu("Résonance du combo", 5_000_000_000, 20_000),
        pu("Clic abyssal", 10_000_000_000, 50_000),
        pu("Clic divin absolu", 50_000_000_000, 100_000),
        pu("Main temporelle", 100_000_000_000, 250_000),
        pu("Doigt infini", 500_000_000_000, 500_000),
        pu("Clic paradoxal", 1_000_000_000_000, 1_000_000),
        pu("Clic dimensionnel", 5_000_000_000_000, 2_500_000),
        pu("Clic cosmologique", 10_000_000_000_000, 5_000_000),
        pu("Clic primordial", 50_000_000_000_000, 10_000_000),
        pu("Clic universel", 100_000_000_000_000, 25_000_000),
        pu("Clic omnipotent", 500_000_000_000_000, 50_000_000),
        pu("Clic absolu", 1_000_000_000_000_000, 100_000_000),
    ];

    let upgrades = vec![
        up("Curseur", 15, 0.1, 0),
        up("Grand-mère", 100, 1.0, 0),
        up("Ferme", 1_100, 8.0, 0),
        up("Mine", 12_000, 47.0, 0),
        up("Usine", 130_000, 260.0, 0),
        up("Banque", 1_400_000, 1_400.0, 0),
        up("Temple", 20_000_000, 7_800.0, 0),
        up("Tour de magie", 330_000_000, 44_000.0, 0),
        up("Portail", 5_100_000_000, 260_000.0, 0),
        up("Machine temporelle", 75_000_000_000, 1_600_000.0, 0),
        up("Condensateur", 1_000_000_000_000, 10_000_000.0, 1),
        up("Prisme", 14_000_000_000_000, 65_000_000.0, 1),
        up("Chancemaker", 170_000_000_000_000, 430_000_000.0, 1),
        up("Fractale", 2_100_000_000_000_000, 2_900_000_000.0, 1),
        up("Console JS", 26_000_000_000_000_000, 21_000_000_000.0, 1),
        up("Simulateur de réalité", 80_000_000_000_000_000, 120_000_000_000.0, 2),
        up("Forge cosmique", 250_000_000_000_000_000, 900_000_000_000.0, 2),
        up("Singularité", 900_000_000_000_000_000, 7_000_000_000_000.0, 2),
        up("Source primordiale", 3_000_000_000_000_000_000, 55_000_000_000_000.0, 3),
        up("Moteur de l'existence", 10_000_000_000_000_000_000, 400_000_000_000_000.0, 3),
        up("Cœur du multivers", 50_000_000_000_000_000_000, 3_500_000_000_000_000.0, 3),
        up("Dieu mathématique", 200_000_000_000_000_000_000, 30_000_000_000_000_000.0, 4),
        up("Entité absolue", 1_000_000_000_000_000_000_000, 250_000_000_000_000_000.0, 4),
        up("Origine", 5_000_000_000_000_000_000_000, 2_000_000_000_000_000_000.0, 5),
    ];

    let achievements = vec![
        // === CLICKS ===
        ach("Premiers pas", "Cliquez 10 fois", AchievementRequirement::Clicks(10)),
        ach("Addict", "Cliquez 1 000 fois", AchievementRequirement::Clicks(1_000)),
        ach("Machine à cliquer", "Cliquez 100 000 fois", AchievementRequirement::Clicks(100_000)),
        ach("Doigt divin", "Cliquez 1 million de fois", AchievementRequirement::Clicks(1_000_000)),
        ach("Doigt qui chauffe", "Cliquez 100 fois", AchievementRequirement::Clicks(100)),
        ach("Tendinite imminente", "Cliquez 10 000 fois", AchievementRequirement::Clicks(10_000)),
        ach("Os broyés", "Cliquez 1 million de fois", AchievementRequirement::Clicks(1_000_000)),
        ach("Plus de doigts", "Cliquez 10 millions de fois", AchievementRequirement::Clicks(10_000_000)),
        ach("Pure volonté", "Cliquez 100 millions de fois", AchievementRequirement::Clicks(100_000_000)),


        // === TOTAL COOKIES ===
        ach("Accumulation", "Gagnez 1 million de cookies", AchievementRequirement::TotalCookies(1_000_000)),
        ach("Capitaliste", "Gagnez 1 milliard de cookies", AchievementRequirement::TotalCookies(1_000_000_000)),
        ach("Tycoon", "Gagnez 1 trillion de cookies", AchievementRequirement::TotalCookies(1_000_000_000_000)),
        ach("Entité cosmique", "Gagnez 1 quintillion de cookies", AchievementRequirement::TotalCookies(1_000_000_000_000_000_000)),
        ach("Boulanger amateur", "Gagnez 100 000 cookies", AchievementRequirement::TotalCookies(100_000)),
        ach("Boulanger industriel", "Gagnez 10 millions de cookies", AchievementRequirement::TotalCookies(10_000_000)),
        ach("Dieu du gluten", "Gagnez 10 billions de cookies", AchievementRequirement::TotalCookies(10_000_000_000_000)),
        ach("Violation des lois physiques", "Gagnez 1 sextillion de cookies", AchievementRequirement::TotalCookies(1_000_000_000_000_000_000_000)),


        // === CPS ===
        ach("Production stable", "1 000 cookies/sec", AchievementRequirement::CookiesPerSecond(1_000)),
        ach("Usine infernale", "1 million cookies/sec", AchievementRequirement::CookiesPerSecond(1_000_000)),
        ach("Réalité industrielle", "1 milliard cookies/sec", AchievementRequirement::CookiesPerSecond(1_000_000_000)),
        ach("Ça tourne", "100 cookies/sec", AchievementRequirement::CookiesPerSecond(100)),
        ach("Rythme soutenu", "10 000 cookies/sec", AchievementRequirement::CookiesPerSecond(10_000)),
        ach("Effondrement énergétique", "10 millions cookies/sec", AchievementRequirement::CookiesPerSecond(10_000_000)),
        ach("Singularité sucrée", "1 trillion cookies/sec", AchievementRequirement::CookiesPerSecond(1_000_000_000_000)),


        // === COMBOS ===
        ach("Encore un", "10 combos", AchievementRequirement::Combos(10)),
        ach("Tu t'arrêtes jamais ?", "100 combos", AchievementRequirement::Combos(100)),
        ach("C'est obsessionnel", "1 000 combos", AchievementRequirement::Combos(1_000)),
        ach("Va toucher de l'herbe", "5 000 combos", AchievementRequirement::Combos(5_000)),
        ach("Carnage", "10 000 combos", AchievementRequirement::Combos(10_000)),
        ach("Boucher en série", "15 000 combos", AchievementRequirement::Combos(15_000)),
        ach("Extinction totale", "20 000 combos", AchievementRequirement::Combos(20_000)),
        ach("Combo naturel", "25 combos", AchievementRequirement::Combos(25)),
        ach("Enchaînement malsain", "250 combos", AchievementRequirement::Combos(250)),
        ach("Ce n’est plus un jeu", "2 500 combos", AchievementRequirement::Combos(2_500)),
        ach("Déni de réalité", "50 000 combos", AchievementRequirement::Combos(50_000)),
        ach("Au-delà du combo", "100 000 combos", AchievementRequirement::Combos(100_000)),



        // === BUILDINGS ===
        ach("Collectionneur", "50 curseurs", AchievementRequirement::BuildingCount(0, 50)),
        ach("Maison de retraite", "50 grand-mères", AchievementRequirement::BuildingCount(1, 50)),
        ach("Ferme intensive", "50 fermes", AchievementRequirement::BuildingCount(2, 50)),
        ach("Empire industriel", "100 usines", AchievementRequirement::BuildingCount(4, 100)),
        ach("Colonisation", "100 curseurs", AchievementRequirement::BuildingCount(0, 100)),
        ach("Exploitation familiale", "100 grand-mères", AchievementRequirement::BuildingCount(1, 100)),
        ach("Agro-capitalisme", "200 fermes", AchievementRequirement::BuildingCount(2, 200)),
        ach("Complexe militaro-boulanger", "300 usines", AchievementRequirement::BuildingCount(4, 300)),


        // === GOLDEN ===
        ach("Chance insolente", "10 golden cookies", AchievementRequirement::GoldenCookies(10)),
        ach("Béni des dieux", "100 golden cookies", AchievementRequirement::GoldenCookies(100)),
        ach("Coup de bol", "1 golden cookie", AchievementRequirement::GoldenCookies(1)),
        ach("Chercheur d’or", "25 golden cookies", AchievementRequirement::GoldenCookies(25)),
        ach("Favori du RNG", "250 golden cookies", AchievementRequirement::GoldenCookies(250)),
        ach("Manipulateur de probas", "1 000 golden cookies", AchievementRequirement::GoldenCookies(1_000)),


        // === PRESTIGE ===
        ach("Renaissance", "Prestige niveau 1", AchievementRequirement::PrestigeLevel(1)),
        ach("Ascension", "Prestige niveau 5", AchievementRequirement::PrestigeLevel(5)),
        ach("Immortel", "Prestige niveau 25", AchievementRequirement::PrestigeLevel(25)),
        ach("Transcendance", "Prestige niveau 100", AchievementRequirement::PrestigeLevel(100)),
        ach("Encore une fois", "Prestige niveau 2", AchievementRequirement::PrestigeLevel(2)),
        ach("Refus de finir", "Prestige niveau 10", AchievementRequirement::PrestigeLevel(10)),
        ach("Éternel recommencement", "Prestige niveau 50", AchievementRequirement::PrestigeLevel(50)),
        ach("Dieu ancien", "Prestige niveau 250", AchievementRequirement::PrestigeLevel(250)),
        ach("Le jeu te joue", "Prestige niveau 1 000", AchievementRequirement::PrestigeLevel(1_000)),

    ];

    let unlocked = vec![false; achievements.len()];
    let achievements = AchievementList { achievements, unlocked };


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

fn pu(name: &str, base: u128, mult: u128) -> PowerUp {
    PowerUp {
        name: name.into(),
        emoji: "".into(),
        base_cost: base,
        cost: base,
        multiplier: mult,
        count: 0,
        description: format!("+{} cookies par clic", mult),
    }
}


fn up(name: &str, base: u128, cps: f64, tier: u128) -> Upgrade {
    Upgrade {
        name: name.into(),
        emoji: "".into(),
        base_cost: base,
        cost: base,
        cps,
        count: 0,
        tier,
        description: format!("Produit {} cookies/sec", cps),
    }
}


pub fn save_game_state(game_state: &GameState) {
    if let Ok(data) = serde_json::to_string_pretty(game_state) {
        if let Err(e) = fs::write(SAVE_FILE, data) {
            eprintln!("❌ Erreur de sauvegarde : {}", e);
        }
    }
}