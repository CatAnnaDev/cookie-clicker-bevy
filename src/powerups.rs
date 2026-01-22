use serde::{Deserialize, Serialize};

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

impl PowerUp {
    pub fn calculate_cost(&self) -> u128 {
        (self.base_cost as f64 * 1.2_f64.powi(self.count as i32)) as u128
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

pub fn get_powerups() -> Vec<PowerUp> {
    vec![
        pu("Clic renforcé", 100, 1),
        pu("Double clic", 500, 2),
        pu("Triple frappe", 2_000, 5),
        pu("Clic furieux", 10_000, 10),
        pu("Doigt d'acier", 50_000, 25),
        pu("Bras cybernétique", 250_000, 50),
        pu("Main divine", 1_000_000, 100),
        pu("Frappe amplifiée", 2_500_000, 150),
        pu("Clic cosmique", 5_000_000, 250),
        pu("Poing stellaire", 12_000_000, 375),
        pu("Doigt quantique", 20_000_000, 500),
        pu("Coup atomique", 50_000_000, 750),
        pu("Main fractale", 100_000_000, 1_000),
        pu("Frappe moléculaire", 250_000_000, 1_500),
        pu("Hyper clic", 500_000_000, 2_500),
        pu("Clic critique", 1_000_000_000, 5_000),
        pu("Frappe explosive", 1_750_000_000, 7_500),
        pu("Clic instable", 2_500_000_000, 10_000),
        pu("Poing du titan", 3_750_000_000, 15_000),
        pu("Résonance du combo", 5_000_000_000, 20_000),
        pu("Frappe sismique", 7_500_000_000, 35_000),
        pu("Clic abyssal", 10_000_000_000, 50_000),
        pu("Main des profondeurs", 25_000_000_000, 75_000),
        pu("Clic divin absolu", 50_000_000_000, 100_000),
        pu("Frappe céleste", 75_000_000_000, 175_000),
        pu("Main temporelle", 100_000_000_000, 250_000),
        pu("Clic distordu", 250_000_000_000, 375_000),
        pu("Doigt infini", 500_000_000_000, 500_000),
        pu("Frappe éternelle", 750_000_000_000, 750_000),
        pu("Clic paradoxal", 1_000_000_000_000, 1_000_000),
        pu("Poing galactique", 2_500_000_000_000, 1_500_000),
        pu("Clic dimensionnel", 5_000_000_000_000, 2_500_000),
        pu("Frappe du vide", 7_500_000_000_000, 3_750_000),
        pu("Clic cosmologique", 10_000_000_000_000, 5_000_000),
        pu("Main de l'oubli", 25_000_000_000_000, 7_500_000),
        pu("Clic primordial", 50_000_000_000_000, 10_000_000),
        pu("Frappe ancestrale", 75_000_000_000_000, 17_500_000),
        pu("Clic universel", 100_000_000_000_000, 25_000_000),
        pu("Poing de la création", 250_000_000_000_000, 37_500_000),
        pu("Clic omnipotent", 500_000_000_000_000, 50_000_000),
        pu("Frappe du destin", 750_000_000_000_000, 75_000_000),
        pu("Clic absolu", 1_000_000_000_000_000, 100_000_000),
        pu("Main de l'alpha", 2_500_000_000_000_000, 175_000_000),
        pu("Frappe éthérée", 5_000_000_000_000_000, 250_000_000),
        pu("Clic transcendant", 10_000_000_000_000_000, 400_000_000),
        pu("Poing originel", 17_500_000_000_000_000, 650_000_000),
        pu("Main du vide", 25_000_000_000_000_000, 1_000_000_000),
        pu("Frappe omnisciente", 50_000_000_000_000_000, 1_750_000_000),
        pu("Doigt du destin", 100_000_000_000_000_000, 3_000_000_000),
        pu("Clic du néant", 200_000_000_000_000_000, 5_000_000_000),
    ]
}