use bevy::prelude::*;

#[derive(Component)]
pub struct Cookie;

#[derive(Component)]
pub struct CookieCounter;

#[derive(Component)]
pub struct CpsCounter;

#[derive(Component)]
pub struct StatsText;

#[derive(Component)]
pub struct UpgradeButton {
    pub upgrade_index: usize,
}

#[derive(Component)]
pub struct UpgradeText {
    pub upgrade_index: usize,
}

#[derive(Component)]
pub struct PowerUpButton {
    pub powerup_index: usize,
}

#[derive(Component)]
pub struct PowerUpText {
    pub powerup_index: usize,
}

#[derive(Component)]
pub struct PopupText {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct CookieScale {
    pub base_scale: f32,
    pub pulse: f32,
}

#[derive(Component)]
pub struct GoldenCookie {
    pub lifetime: Timer,
    pub multiplier: u64,
}

#[derive(Component)]
pub struct ComboText;

#[derive(Component)]
pub struct AchievementText;

#[derive(Component)]
pub struct PrestigeButton;

#[derive(Component)]
pub struct PrestigeText;

#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct MilestoneText;

#[derive(Component)]
pub struct ScrollingList {
    pub(crate) position: f32,
}

#[derive(Component)]
pub struct PowerUpScrollArea;

#[derive(Component)]
pub struct BuildingScrollArea;

#[derive(Component)]
pub struct AchievementPopup {
    pub timer: Timer,
    pub index: usize,
}
