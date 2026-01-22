use serde::{Deserialize, Serialize};

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

impl Upgrade {
    pub fn calculate_cost(&self) -> u128 {
        let base = self.base_cost as f64 * 1.15_f64.powi(self.count as i32);
        (base * (1.0 - (self.tier as f64 * 0.01))) as u128
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

pub fn get_upgrade() -> Vec<Upgrade> {
    vec![
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
        up("Générateur d'entropie", 50_000_000_000_000_000, 60_000_000_000.0, 2),
        up("Simulateur de réalité", 80_000_000_000_000_000, 120_000_000_000.0, 2),
        up("Distorseur quantique", 150_000_000_000_000_000, 400_000_000_000.0, 2),
        up("Forge cosmique", 250_000_000_000_000_000, 900_000_000_000.0, 2),
        up("Matrice probabiliste", 500_000_000_000_000_000, 3_000_000_000_000.0, 2),
        up("Singularité", 900_000_000_000_000_000, 7_000_000_000_000.0, 2),
        up("Collapseur stellaire", 1_500_000_000_000_000_000, 20_000_000_000_000.0, 3),
        up("Source primordiale", 3_000_000_000_000_000_000, 55_000_000_000_000.0, 3),
        up("Nexus dimensionnel", 6_000_000_000_000_000_000, 150_000_000_000_000.0, 3),
        up("Moteur de l'existence", 10_000_000_000_000_000_000, 400_000_000_000_000.0, 3),
        up("Catalyseur d'univers", 25_000_000_000_000_000_000, 1_200_000_000_000_000.0, 3),
        up("Cœur du multivers", 50_000_000_000_000_000_000, 3_500_000_000_000_000.0, 3),
        up("Tisseur de réalités", 120_000_000_000_000_000_000, 10_000_000_000_000_000.0, 4),
        up("Dieu mathématique", 200_000_000_000_000_000_000, 30_000_000_000_000_000.0, 4),
        up("Oracle temporel", 500_000_000_000_000_000_000, 90_000_000_000_000_000.0, 4),
        up("Entité absolue", 1_000_000_000_000_000_000_000, 250_000_000_000_000_000.0, 4),
        up("Architecte des vides", 2_500_000_000_000_000_000_000, 700_000_000_000_000_000.0, 5),
        up("Origine", 5_000_000_000_000_000_000_000, 2_000_000_000_000_000_000.0, 5),
        up("Essence primale", 12_000_000_000_000_000_000_000, 6_000_000_000_000_000_000.0, 5),
        up("Architecte dimensionnel", 25_000_000_000_000_000_000_000, 15_000_000_000_000_000_000.0, 5),
        up("Volonté cosmique", 60_000_000_000_000_000_000_000, 45_000_000_000_000_000_000.0, 5),
        up("Gardien éternel", 125_000_000_000_000_000_000_000, 120_000_000_000_000_000_000.0, 6),
        up("Maître du chaos", 300_000_000_000_000_000_000_000, 350_000_000_000_000_000_000.0, 6),
        up("Tisseur de probabilités", 600_000_000_000_000_000_000_000, 900_000_000_000_000_000_000.0, 6),
        up("Forgeron d'étoiles", 1_500_000_000_000_000_000_000_000, 2_700_000_000_000_000_000_000.0, 7),
        up("Démiurge", 3_000_000_000_000_000_000_000_000, 7_500_000_000_000_000_000_000.0, 7),
    ]
}