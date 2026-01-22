use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};


#[derive(Resource, Serialize, Deserialize, Clone, Default)]
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
pub fn get_achievements() -> AchievementList {
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
    AchievementList { achievements, unlocked }
}