use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::ui;
use crate::ui::spawn_achievement_popup;
use crate::utils::*;

const CLICKS_PER_COMBO: u128 = 10;

pub fn cookie_click_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Node, &GlobalTransform, &mut CookieScale),
        (Changed<Interaction>, With<Cookie>),
    >,
    mut game_state: ResMut<GameState>,
    click_power: Res<ClickPower>,
    mut combo: ResMut<ComboSystem>,
    window_query: Query<&Window>,
) {
    for (interaction, mut color, node, transform, mut scale) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let base_earned = click_power.0;
                let combo_mult = if combo.active { 1 + combo.combo } else { 1 };
                let earned = base_earned * combo_mult;

                game_state.cookies += earned;
                game_state.total_cookies_earned += earned;
                game_state.lifetime_cookies += earned;
                game_state.click_count += 1;

                combo.clicks += 1;
                combo.timer.reset();
                combo.active = true;

                let previous_combo = combo.combo;
                combo.combo = combo.clicks / CLICKS_PER_COMBO;

                let combo_up = combo.combo > previous_combo;

                let earned = base_earned * combo_mult;

                *color = Color::srgb(1.0, 0.7, 0.3).into();
                scale.pulse = 1.0;

                let window = window_query.single();
                let window_height = window.height();
                let cookie_pos = transform.translation();

                spawn_popup(&mut commands, earned, cookie_pos, node, window_height, combo_up);
                spawn_particles(&mut commands, cookie_pos, node, window_height);
            }
            Interaction::Hovered => {
                *color = Color::srgb(0.95, 0.65, 0.25).into();
            }
            Interaction::None => {
                *color = Color::srgb(0.9, 0.6, 0.2).into();
            }
        }
    }
}

pub fn popup_movement(
    time: Res<Time>,
    mut query: Query<(&mut Style, &mut PopupText)>,
) {
    for (mut style, mut popup) in &mut query {
        let dt = time.delta_seconds();

        popup.velocity.y += 420.0 * dt; // gravity inversée douce
        popup.velocity *= 0.92;         // damping

        if let Val::Px(x) = style.left {
            style.left = Val::Px(x + popup.velocity.x * dt);
        }
        if let Val::Px(y) = style.top {
            style.top = Val::Px(y + popup.velocity.y * dt);
        }
    }
}


fn random_ring(min: f32, max: f32) -> Vec2 {
    let angle = pseudo_random() * std::f32::consts::TAU;
    let r = min + pseudo_random() * (max - min);
    Vec2::new(angle.cos() * r, angle.sin() * r)
}


fn spawn_popup(
    commands: &mut Commands,
    earned: u128,
    cookie_pos: Vec3,
    node: &Node,
    window_height: f32,
    is_combo: bool,
) {
    let base_color = if is_combo {
        Color::srgb(1.0, 0.35, 0.35)
    } else {
        Color::srgb(1.0, 0.9, 0.35)
    };

    let radius_min = if is_combo { 20.0 } else { 12.0 };
    let radius_max = if is_combo { 45.0 } else { 28.0 };
    let offset = random_ring(radius_min, radius_max);

    let spawn_x = cookie_pos.x + node.size().x / 2.0 + offset.x;
    let spawn_y = cookie_pos.y - 70.0 + offset.y;

    let dir = offset.normalize_or_zero();

    let impulse = if is_combo { 260.0 } else { 200.0 };
    let velocity = Vec2::new(
        dir.x * impulse + (pseudo_random() - 0.5) * 40.0,
        dir.y * impulse - 120.0,
    );

    let scale = 0.85 + pseudo_random() * 0.25;

    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(spawn_x),
                top: Val::Px(spawn_y),
                ..default()
            },
            text: Text::from_section(
                format!("+{}", format_number(earned as u128)),
                TextStyle {
                    font_size: 40.0 , // * scale
                    color: base_color,
                    ..default()
                },
            ),
            z_index: ZIndex::Global(100),
            transform: Transform::from_scale(Vec3::splat(scale)),
            ..default()
        },
        PopupText {
            lifetime: Timer::from_seconds(if is_combo { 1.8 } else { 1.4 }, TimerMode::Once),
            velocity,
        },
    ));
}


fn spawn_particles(commands: &mut Commands, cookie_pos: Vec3, node: &Node, window_height: f32) {
    for _ in 0..5 {
        let angle = pseudo_random() * std::f32::consts::TAU;
        let speed = 100.0 + pseudo_random() * 100.0;

        commands.spawn((
            TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(cookie_pos.x + node.size().x / 2.0),
                    top: Val::Px(window_height / 2.0 - cookie_pos.y),
                    ..default()
                },
                text: Text::from_section(
                    "✨",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgba(1.0, 0.9, 0.5, 1.0),
                        ..default()
                    },
                ),
                z_index: ZIndex::Global(99),
                ..default()
            },
            Particle {
                lifetime: Timer::from_seconds(0.8, TimerMode::Once),
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            },
        ));
    }
}

pub fn upgrade_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &UpgradeButton),
        Changed<Interaction>,
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color, mut border, upgrade_button) in &mut interaction_query {
        if upgrade_button.upgrade_index >= game_state.upgrades.len() {
            continue;
        }

        let upgrade_cost = game_state.upgrades[upgrade_button.upgrade_index].cost;
        let upgrade_cps = game_state.upgrades[upgrade_button.upgrade_index].cps;
        let can_afford = game_state.cookies >= upgrade_cost;

        match *interaction {
            Interaction::Pressed => {
                if can_afford {
                    game_state.cookies -= upgrade_cost;
                    let upgrade = &mut game_state.upgrades[upgrade_button.upgrade_index];
                    upgrade.count += 1;
                    upgrade.cost = upgrade.calculate_cost();
                    let multiplier = 1.0 + (game_state.prestige_level as f64 * 0.01);
                    game_state.cookies_per_second += upgrade_cps * multiplier;
                    *color = Color::srgb(0.3, 0.7, 0.4).into();
                    *border = Color::srgb(0.4, 1.0, 0.5).into();
                }
            }
            Interaction::Hovered => {
                if can_afford {
                    *color = Color::srgb(0.25, 0.4, 0.5).into();
                    *border = Color::srgb(0.4, 0.7, 0.9).into();
                } else {
                    *color = Color::srgb(0.35, 0.25, 0.25).into();
                    *border = Color::srgb(0.6, 0.3, 0.3).into();
                }
            }
            Interaction::None => {
                if can_afford {
                    *color = Color::srgb(0.2, 0.3, 0.4).into();
                    *border = Color::srgb(0.3, 0.4, 0.5).into();
                } else {
                    *color = Color::srgb(0.25, 0.15, 0.15).into();
                    *border = Color::srgb(0.4, 0.2, 0.2).into();
                }
            }
        }
    }
}

pub fn powerup_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor, &PowerUpButton),
        Changed<Interaction>,
    >,
    mut game_state: ResMut<GameState>,
    mut click_power: ResMut<ClickPower>,
) {
    for (interaction, mut color, mut border, powerup_button) in &mut interaction_query {
        if powerup_button.powerup_index >= game_state.powerups.len() {
            continue;
        }

        let powerup = &game_state.powerups[powerup_button.powerup_index];
        let can_afford = game_state.cookies >= powerup.cost;

        match *interaction {
            Interaction::Pressed => {
                if can_afford {
                    game_state.cookies -= powerup.cost;
                    let powerup = &mut game_state.powerups[powerup_button.powerup_index];
                    powerup.count += 1;
                    powerup.cost = powerup.calculate_cost();
                    click_power.0 += powerup.multiplier;
                    game_state.cookies_per_click = click_power.0;
                    *color = Color::srgb(0.5, 0.3, 0.7).into();
                    *border = Color::srgb(0.7, 0.4, 1.0).into();
                }
            }
            Interaction::Hovered => {
                if can_afford {
                    *color = Color::srgb(0.3, 0.25, 0.4).into();
                    *border = Color::srgb(0.5, 0.4, 0.7).into();
                } else {
                    *color = Color::srgb(0.3, 0.2, 0.3).into();
                    *border = Color::srgb(0.5, 0.3, 0.4).into();
                }
            }
            Interaction::None => {
                if can_afford {
                    *color = Color::srgb(0.25, 0.2, 0.35).into();
                    *border = Color::srgb(0.4, 0.3, 0.5).into();
                } else {
                    *color = Color::srgb(0.2, 0.15, 0.2).into();
                    *border = Color::srgb(0.3, 0.2, 0.3).into();
                }
            }
        }
    }
}

pub fn prestige_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PrestigeButton>),
    >,
    mut game_state: ResMut<GameState>,
) {
    for (interaction, mut color) in &mut interaction_query {
        let prestige_cookies_needed = prestige_requirement((game_state.prestige_level + 100) as u64);
        let can_prestige = game_state.cookies >= prestige_cookies_needed;

        match *interaction {
            Interaction::Pressed => {
                if can_prestige {
                    game_state.prestige_level += 1;
                    game_state.prestige_points = game_state.lifetime_cookies / 1_000_000;
                    game_state.cookies = 0;
                    game_state.total_cookies_earned = 0;
                    game_state.cookies_per_second = 0.0;

                    for upgrade in &mut game_state.upgrades {
                        upgrade.count = 0;
                        upgrade.cost = upgrade.base_cost;
                    }

                    println!("⭐ PRESTIGE! Niveau {}", game_state.prestige_level);
                }
            }
            Interaction::Hovered => {
                *color = if can_prestige {
                    Color::srgb(1.0, 0.8, 0.2).into()
                } else {
                    Color::srgb(0.3, 0.3, 0.3).into()
                };
            }
            Interaction::None => {
                *color = if can_prestige {
                    Color::srgb(0.9, 0.7, 0.1).into()
                } else {
                    Color::srgb(0.2, 0.2, 0.2).into()
                };
            }
        }
    }
}

pub fn passive_income_system(mut game_state: ResMut<GameState>, time: Res<Time>) {
    if game_state.cookies_per_second > 0.0 {
        let earned = (game_state.cookies_per_second * time.delta_seconds_f64()) as u128;
        if earned > 0 {
            game_state.cookies += earned;
            game_state.total_cookies_earned += earned;
            game_state.lifetime_cookies += earned;
        }
    }
}

pub fn golden_cookie_spawn_system(
    mut commands: Commands,
    mut timer: ResMut<GoldenCookieTimer>,
    time: Res<Time>,
    golden_cookies: Query<&GoldenCookie>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() && golden_cookies.iter().count() < 2 {
        let x = -400.0 + pseudo_random() * 800.0;
        let y = -300.0 + pseudo_random() * 600.0;
        let multiplier = 7 + (pseudo_random() * 14.0) as u64;

        commands.spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(x + 600.0),
                    top: Val::Px(y + 400.0),
                    width: Val::Px(60.0),
                    height: Val::Px(60.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(1.0, 0.9, 0.2, 0.9).into(),
                border_radius: BorderRadius::all(Val::Percent(50.0)),
                z_index: ZIndex::Global(50),
                ..default()
            },
            GoldenCookie {
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                multiplier,
            },
        ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "🌟",
                    TextStyle {
                        font_size: 40.0,
                        ..default()
                    },
                ));
            });
    }
}

pub fn golden_cookie_click_system(
    mut commands: Commands,
    mut interaction_query: Query<(Entity, &Interaction, &GoldenCookie), Changed<Interaction>>,
    mut game_state: ResMut<GameState>,
) {
    for (entity, interaction, golden) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let mut bonus = game_state.cookies_per_second as u128 * golden.multiplier as u128 * 60;
            if bonus == 0{
                bonus = 2000;
            }
            game_state.cookies += bonus;
            game_state.total_cookies_earned += bonus;
            game_state.lifetime_cookies += bonus;
            game_state.golden_cookies_clicked += 1;

            commands.entity(entity).despawn_recursive();
            spawn_achievement_popup(&mut commands, "🍪", &format!("✨ Golden Cookie! +{} cookies", format_number(bonus)), 0.0);
        }
    }
}

pub fn golden_cookie_cleanup_system(
    mut commands: Commands,
    mut golden_cookies: Query<(Entity, &mut GoldenCookie)>,
    time: Res<Time>,
) {
    for (entity, mut golden) in &mut golden_cookies {
        golden.lifetime.tick(time.delta());
        if golden.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn combo_system(
    mut combo: ResMut<ComboSystem>,
    time: Res<Time>,
) {
    if combo.active {
        combo.timer.tick(time.delta());
        if combo.timer.finished() {
            combo.combo = 0;
            combo.active = false;
            combo.clicks = 0;
        }
    }
}

pub fn check_achievements_system(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
) {
    let mut to_unlock = Vec::new();

    for (i, achievement) in game_state.achievements.achievements.iter().enumerate() {
        if !game_state.achievements.unlocked[i] {
            let unlocked = match &achievement.requirement {
                AchievementRequirement::TotalCookies(amount) => {
                    game_state.total_cookies_earned >= *amount
                }
                AchievementRequirement::CookiesPerSecond(amount) => {
                    game_state.cookies_per_second as u128 >= *amount
                }
                AchievementRequirement::Clicks(amount) => {
                    game_state.click_count >= *amount
                }
                AchievementRequirement::GoldenCookies(amount) => {
                    game_state.golden_cookies_clicked >= *amount
                }
                AchievementRequirement::BuildingCount(index, count) => {
                    if *index < game_state.upgrades.len() {
                        game_state.upgrades[*index].count >= *count
                    } else {
                        false
                    }
                }
                AchievementRequirement::PrestigeLevel(level) => {
                    game_state.prestige_level >= *level
                }
            };

            if unlocked {
                to_unlock.push((i, achievement.emoji.clone(), achievement.name.clone()));
            }
        }
    }

    let mut popup_index = 0;

    for (i, emoji, name) in to_unlock {
        game_state.achievements.unlocked[i] = true;

        spawn_achievement_popup(
            &mut commands,
            &*emoji,
            &*name,
            popup_index as f32 * 90.0, // stack vertically
        );

        popup_index += 1;
    }

}

pub fn milestone_system(
    game_state: Res<GameState>,
    mut milestone_query: Query<&mut Text, With<MilestoneText>>,
) {
    for mut text in &mut milestone_query {
        let milestone = if game_state.total_cookies_earned >= 1_000_000_000_000_000_000_000 {
            "🌌 Déesse des Cookies"
        } else if game_state.total_cookies_earned >= 100_000_000_000_000_000_000 {
            "✨ Entité Suprême du Four"
        } else if game_state.total_cookies_earned >= 10_000_000_000_000_000_000 {
            "👁️ Architecte du Multivers Sucré"
        } else if game_state.total_cookies_earned >= 1_000_000_000_000_000_000 {
            "🔥 Impératrice des Cookies"
        } else if game_state.total_cookies_earned >= 100_000_000_000_000_000 {
            "🌠 Reine Cosmique du Sucre"
        } else if game_state.total_cookies_earned >= 10_000_000_000_000_000 {
            "💫 Souveraine des Dimensions Sucrées"
        } else if game_state.total_cookies_earned >= 1_000_000_000_000_000 {
            "🏆 Légende des Cookies"
        } else if game_state.total_cookies_earned >= 100_000_000_000_000 {
            "💎 Maîtresse Absolue du Four"
        } else if game_state.total_cookies_earned >= 10_000_000_000_000 {
            "👑 Grande Reine des Cookies"
        } else if game_state.total_cookies_earned >= 1_000_000_000_000 {
            "👸 Reine des Cookies"
        } else if game_state.total_cookies_earned >= 100_000_000_000 {
            "🌟 Duchesse du Sucre"
        } else if game_state.total_cookies_earned >= 10_000_000_000 {
            "🎖️ Marquise des Cookies"
        } else if game_state.total_cookies_earned >= 1_000_000_000 {
            "⭐ Baronne des Cookies"
        } else if game_state.total_cookies_earned >= 100_000_000 {
            "🍰 Dame du Four"
        } else if game_state.total_cookies_earned >= 10_000_000 {
            "🥐 Experte Pâtissière"
        } else if game_state.total_cookies_earned >= 1_000_000 {
            "🍪 Maîtresse Boulangère"
        } else if game_state.total_cookies_earned >= 100_000 {
            "🧁 Pâtissière Confirmée"
        } else if game_state.total_cookies_earned >= 10_000 {
            "🥖 Apprentie Boulangère"
        } else if game_state.total_cookies_earned >= 1_000 {
            "🎀 Novice du Sucre"
        } else {
            "🌱 Débutante"
        };


        text.sections[0].value = milestone.to_string();
    }
}


pub fn animate_cookie_system(
    mut cookie_query: Query<(&mut Style, &mut CookieScale), With<Cookie>>,
    time: Res<Time>,
) {
    for (mut style, mut cookie_scale) in &mut cookie_query {
        if cookie_scale.pulse > 0.0 {
            cookie_scale.pulse -= time.delta_seconds() * 4.0;
            cookie_scale.pulse = cookie_scale.pulse.max(0.0);
        }

        let base_pulse = (time.elapsed_seconds() * 2.0).sin() * 0.02;
        let scale = 1.0 + base_pulse + cookie_scale.pulse * 0.2;
        let size = 180.0 * scale;

        style.width = Val::Px(size);
        style.height = Val::Px(size);
    }
}

pub fn animate_popup_system(
    mut popup_query: Query<(&mut Style, &mut Text, &mut PopupText)>,
    time: Res<Time>,
) {
    for (mut style, mut text, mut popup) in &mut popup_query {
        popup.lifetime.tick(time.delta());

        let progress = popup.lifetime.fraction();

        if let Val::Px(top) = style.top {
            style.top = Val::Px(top + popup.velocity.y * time.delta_seconds());
        }
        if let Val::Px(left) = style.left {
            style.left = Val::Px(left + popup.velocity.x * time.delta_seconds());
        }

        let alpha = 1.0 - progress;
        let scale = 1.0 + progress * 0.4;
        text.sections[0].style.color.set_alpha(alpha);
        text.sections[0].style.font_size = 40.0; // * scale
    }
}

pub fn cleanup_popup_system(
    mut commands: Commands,
    popup_query: Query<(Entity, &PopupText)>,
) {
    for (entity, popup) in &popup_query {
        if popup.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn particle_system(
    mut particle_query: Query<(&mut Style, &mut Text, &mut Particle)>,
    time: Res<Time>,
) {
    for (mut style, mut text, mut particle) in &mut particle_query {
        particle.lifetime.tick(time.delta());

        let progress = particle.lifetime.fraction();

        if let Val::Px(top) = style.top {
            style.top = Val::Px(top + particle.velocity.y * time.delta_seconds());
        }
        if let Val::Px(left) = style.left {
            style.left = Val::Px(left + particle.velocity.x * time.delta_seconds());
        }

        let alpha = 1.0 - progress;
        text.sections[0].style.color.set_alpha(alpha);
    }
}

pub fn particle_cleanup_system(
    mut commands: Commands,
    particle_query: Query<(Entity, &Particle)>,
) {
    for (entity, particle) in &particle_query {
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_ui_system(
    game_state: Res<GameState>,
    mut cookie_query: Query<&mut Text, With<CookieCounter>>,
    mut cps_query: Query<&mut Text, (With<CpsCounter>, Without<CookieCounter>)>,
    mut stats_query: Query<&mut Text, (With<StatsText>, Without<CookieCounter>, Without<CpsCounter>)>,
    mut upgrade_query: Query<(&mut Text, &UpgradeText), (Without<CookieCounter>, Without<CpsCounter>, Without<StatsText>)>,
) {
    for mut text in &mut cookie_query {
        text.sections[0].value = format!("{} cookies", format_number(game_state.cookies as u128));
    }

    for mut text in &mut cps_query {
        text.sections[0].value = format!("par seconde: {}", format_number(game_state.cookies_per_second as u128));
    }

    for mut text in &mut stats_query {
        text.sections[0].value = format!(
            "📊 Statistiques:\n\
            Total gagné: {}\n\
            Puissance de clic: {}\n\
            Clics: {}\n\
            Golden cookies: {}\n\
            Prestige: ⭐ Niveau {}",
            format_number(game_state.total_cookies_earned as u128),
            format_number(game_state.cookies_per_click as u128),
            format_number(game_state.click_count as u128),
            game_state.golden_cookies_clicked,
            game_state.prestige_level
        );
    }

    for (mut text, upgrade_text) in &mut upgrade_query {
        if upgrade_text.upgrade_index < game_state.upgrades.len() {
            let upgrade = &game_state.upgrades[upgrade_text.upgrade_index];
            text.sections[0].value = format!(
                "{} {} [{}]\n{}\n💰 {} | ⚡ {}/s",
                upgrade.emoji,
                upgrade.name,
                upgrade.count,
                upgrade.description,
                format_number(upgrade.cost as u128),
                format_number(upgrade.cps as u128)
            );
        }
    }
}

pub fn update_stats_system(
    game_state: Res<GameState>,
    combo: Res<ComboSystem>,
    mut powerup_query: Query<(&mut Text, &PowerUpText), Without<ComboText>>,
    mut combo_query: Query<&mut Text, (With<ComboText>, Without<PowerUpText>, Without<AchievementText>)>,
    mut achievement_query: Query<&mut Text, (With<AchievementText>, Without<ComboText>, Without<PowerUpText>)>,
    mut prestige_query: Query<&mut Text, (With<PrestigeText>, Without<ComboText>, Without<PowerUpText>, Without<AchievementText>)>,
) {
    for (mut text, powerup_text) in &mut powerup_query {
        if powerup_text.powerup_index < game_state.powerups.len() {
            let powerup = &game_state.powerups[powerup_text.powerup_index];
            text.sections[0].value = format!(
                "{} {} [{}]\n{}\n💰 {}",
                powerup.emoji,
                powerup.name,
                powerup.count,
                powerup.description,
                format_number(powerup.cost)
            );
        }
    }

    for mut text in &mut combo_query {
        if combo.active && combo.combo > 1 {
            text.sections[0].value = format!("🔥 COMBO x{} 🔥", combo.combo);
        } else {
            text.sections[0].value = "".to_string();
        }
    }

    for mut text in &mut achievement_query {
        let unlocked = game_state.achievements.unlocked.iter().filter(|&&x| x).count();
        let total = game_state.achievements.achievements.len();
        text.sections[0].value = format!("🏆 Succès: {}/{}", unlocked, total);
    }

    for mut text in &mut prestige_query {
        let prestige_cookies_needed = prestige_requirement((game_state.prestige_level + 100) as u64);
        if game_state.cookies >= prestige_cookies_needed {
            let bonus = (game_state.prestige_level + 1) as f64 * 0.01 * 100.0;
            text.sections[0].value = format!(
                "⭐ PRESTIGE DISPONIBLE!\nBonus: +{:.0}% production\nNiveau suivant: {}",
                bonus,
                game_state.prestige_level + 1
            );
        } else {
            let progress = (game_state.cookies as f64 / prestige_cookies_needed as f64 * 100.0) as u32;
            text.sections[0].value = format!(
                "Prestige: {}%\nRequis: {}",
                progress,
                format_number(prestige_cookies_needed)
            );
        }
    }
}

pub fn prestige_requirement(level: u64) -> u128 {
    let base = 1e11;
    let exponent = 1.15 + (level as f64 * 0.005);

    (base * (level as f64 + 1.0).powf(exponent)) as u128
}


pub fn auto_save_system(
    mut save_timer: ResMut<SaveTimer>,
    game_state: Res<GameState>,
    time: Res<Time>,
) {
    save_timer.0.tick(time.delta());

    if save_timer.0.just_finished() {
        save_game_state(&*game_state);
        println!("💾 Sauvegarde automatique");
    }
}
