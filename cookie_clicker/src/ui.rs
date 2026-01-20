use crate::components::*;
use crate::ui_fonts::UiFonts;
use crate::ui_icons::icon;
use crate::ui_theme::UiTheme;
use bevy::asset::AssetPath;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::resources::GameState;

pub fn setup_ui(mut commands: Commands, assets: Res<AssetServer>, fonts: Res<UiFonts>, game_state: Res<GameState>) {
    commands.spawn(Camera2dBundle::default());

    // ROOT
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            background_color: UiTheme::BG_MAIN.into(),
            ..default()
        })
        .with_children(|root| {
            // =========================
            // LEFT COLUMN
            // =========================
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|left| {
                // HEADER + COOKIE
                left.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(70.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: UiTheme::BG_PANEL.into(),
                    ..default()
                })
                .with_children(|header| {
                    // TITLE
                    header
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|row| {
                            icon(row, &assets, AssetPath::from("ui/icons/cookie.png"), 36.0);
                            row.spawn(TextBundle::from_section(
                                "COOKIE EMPIRE",
                                TextStyle {
                                    font: fonts.bold.clone(),
                                    font_size: 40.0,
                                    color: UiTheme::ACCENT,
                                },
                            ));
                        });

                    header.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: fonts.bold.clone(),
                                font_size: 32.0,
                                color: UiTheme::TEXT,
                            },
                        ),
                        MilestoneText,
                    ));

                    // COUNTER
                    header.spawn((
                        TextBundle::from_section(
                            "0 cookies",
                            TextStyle {
                                font: fonts.bold.clone(),
                                font_size: 52.0,
                                color: UiTheme::TEXT,
                            },
                        ),
                        CookieCounter,
                    ));

                    header.spawn((
                        TextBundle::from_section(
                            "per second: 0",
                            TextStyle {
                                font: fonts.regular.clone(),
                                font_size: 18.0,
                                color: UiTheme::MUTED,
                            },
                        ),
                        CpsCounter,
                    ));

                    header.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: fonts.semibold.clone(),
                                font_size: 22.0,
                                color: Color::srgb(1.0, 0.4, 0.3),
                            },
                        ),
                        ComboText,
                    ));

                    // COOKIE BUTTON
                    header
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(180.0),
                                    height: Val::Px(180.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect::top(Val::Px(20.0)),
                                    ..default()
                                },
                                background_color: UiTheme::ACCENT.into(),
                                border_radius: BorderRadius::all(Val::Percent(50.0)),
                                ..default()
                            },
                            Cookie,
                            CookieScale {
                                base_scale: 1.0,
                                pulse: 0.0,
                            },
                        ))
                        .with_children(|btn| {
                            btn.spawn(ImageBundle {
                                style: Style {
                                    width: Val::Px(110.0),
                                    height: Val::Px(110.0),
                                    ..default()
                                },
                                image: UiImage::new(assets.load("ui/icons/cookie.png")),
                                ..default()
                            });
                        });
                });

                // FOOTER (STATS / PRESTIGE)
                left.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(30.0),
                        flex_direction: FlexDirection::Row,
                        padding: UiRect::all(Val::Px(16.0)),
                        column_gap: Val::Px(16.0),
                        ..default()
                    },
                    background_color: UiTheme::BG_PANEL.into(),
                    ..default()
                })
                .with_children(|footer| {
                    // STATS CARD
                    footer
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(6.0),
                                ..default()
                            },
                            background_color: UiTheme::BG_CARD.into(),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|card| {
                            card.spawn(
                                (TextBundle::from_section(
                                "Stats",
                                TextStyle {
                                    font: fonts.semibold.clone(),
                                    font_size: 18.0,
                                    color: UiTheme::TEXT,
                                },
                            ), StatsText
                                ));
                            card.spawn((
                                TextBundle::from_section(
                                    "Achievements: 0/0",
                                    TextStyle {
                                        font: fonts.regular.clone(),
                                        font_size: 14.0,
                                        color: UiTheme::MUTED,
                                    },
                                ),
                                AchievementText,
                            ));
                        });

                    // PRESTIGE CARD
                    footer
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(10.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: UiTheme::BG_CARD.into(),
                            border_radius: BorderRadius::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_children(|card| {
                            card.spawn((
                                TextBundle::from_section(
                                    "Prestige",
                                    TextStyle {
                                        font: fonts.semibold.clone(),
                                        font_size: 16.0,
                                        color: UiTheme::TEXT,
                                    },
                                ),
                                PrestigeText,
                            ));

                            card.spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(12.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.18, 0.18, 0.22).into(),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                PrestigeButton,
                            ))
                            .with_children(|btn| {
                                icon(btn, &assets, AssetPath::from("ui/icons/star.png"), 16.0);
                                btn.spawn(TextBundle::from_section(
                                    "PRESTIGE",
                                    TextStyle {
                                        font: fonts.semibold.clone(),
                                        font_size: 16.0,
                                        color: UiTheme::ACCENT,
                                    },
                                ));
                            });
                        });
                });
            });

            // =========================
            // RIGHT COLUMN (UPGRADES)
            // =========================
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(40.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(14.0)),
                    ..default()
                },
                background_color: UiTheme::BG_PANEL.into(),
                ..default()
            })
            .with_children(|right| {
                section(
                    right,
                    &assets,
                    &fonts,
                    "POWER UPS",
                    AssetPath::from("ui/icons/power.png"),
                    game_state.powerups.len(),
                    true,
                );
                section(
                    right,
                    &assets,
                    &fonts,
                    "BUILDINGS",
                    AssetPath::from("ui/icons/building.png"),
                    game_state.upgrades.len(),
                    false,
                );
            });
        });
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_scroll: Query<(&mut ScrollingList, &mut Style, &Children, &Interaction), With<Node>>,
    query_children: Query<&Interaction>,
) {
    for event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, children, interaction) in &mut query_scroll {

            // Vérifie hover parent OU hover sur un enfant
            let mut is_hovered = *interaction == Interaction::Hovered;
            if !is_hovered {
                for &child in children.iter() {
                    if let Ok(child_interaction) = query_children.get(child) {
                        if *child_interaction == Interaction::Hovered {
                            is_hovered = true;
                            break;
                        }
                    }
                }
            }

            if is_hovered {
                scrolling_list.position += event.y * 25.0;
                scrolling_list.position = scrolling_list.position.clamp(-2000.0, 0.0);
                style.top = Val::Px(scrolling_list.position);
            }
        }
    }
}


fn section(
    parent: &mut ChildBuilder,
    assets: &AssetServer,
    fonts: &UiFonts,
    title: &str,
    icon_path: AssetPath,
    count: usize,
    is_powerup: bool,
) {
    parent
        .spawn((NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: UiTheme::BG_CARD.into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        }, ScrollingList { position: 0.0 },))
        .with_children(|section| {
            // HEADER
            section
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|row| {
                    icon(row, assets, icon_path, 18.0);
                    row.spawn(TextBundle::from_section(
                        title,
                        TextStyle {
                            font: fonts.semibold.clone(),
                            font_size: 18.0,
                            color: UiTheme::TEXT,
                        },
                    ));
                });

            // LIST
            section
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(6.0),
                            ..default()
                        },
                        ..default()
                    },
                    ScrollingList { position: 0.0 },
                )).insert(Interaction::default())
                .with_children(|list| {
                    for i in 0..count {
                        if is_powerup {
                            list.spawn((
                                ButtonBundle {
                                    style: Style {
                                        //padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.18, 0.18, 0.22).into(),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                PowerUpButton { powerup_index: i },
                            )).insert(Interaction::default())
                                .with_children(|btn| {
                                    btn.spawn((
                                        TextBundle::from_section(
                                            "Item",
                                            TextStyle {
                                                font: fonts.regular.clone(),
                                                font_size: 14.0,
                                                color: UiTheme::TEXT,
                                            },
                                        ),
                                        PowerUpText { powerup_index: i,},
                                    ));
                                });
                        }else{
                            list.spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    background_color: Color::srgb(0.18, 0.18, 0.22).into(),
                                    border_radius: BorderRadius::all(Val::Px(8.0)),
                                    ..default()
                                },
                                UpgradeButton{ upgrade_index: i},
                            ))
                                .with_children(|btn| {
                                    btn.spawn((
                                        TextBundle::from_section(
                                            "Item",
                                            TextStyle {
                                                font: fonts.regular.clone(),
                                                font_size: 14.0,
                                                color: UiTheme::TEXT,
                                            },
                                        ),
                                        UpgradeText {upgrade_index: i},
                                    ));
                                });
                        }
                    }
                });
        });
}


pub fn spawn_achievement_popup(
    commands: &mut Commands,
    emoji: &str,
    name: &str,
    offset_y: f32,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(-320.0), // start off-screen
                top: Val::Px(40.0 + offset_y),
                width: Val::Px(300.0),
                height: Val::Px(80.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.08, 0.08, 0.08, 0.95).into(),
            z_index: ZIndex::Global(200),
            ..default()
        },
        AchievementPopup {
            timer: Timer::from_seconds(4.0, TimerMode::Once),
            index: 0,
        },
    ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                emoji,
                TextStyle {
                    font_size: 36.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent.spawn(
                TextBundle::from_section(
                    format!("{}", name),
                    TextStyle {
                        font_size: 18.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                    .with_style(Style {
                        margin: UiRect::left(Val::Px(10.0)),
                        ..default()
                    }),
            );
        });
}

pub fn achievement_popup_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Style, &mut AchievementPopup)>,
) {
    for (entity, mut style, mut popup) in &mut query {
        popup.timer.tick(time.delta());

        // Slide in
        let t = popup.timer.elapsed_secs();

        if t < 0.4 {
            style.right = Val::Px(-320.0 + t * 800.0);
        }
        // Slide out
        else if popup.timer.remaining_secs() < 0.4 {
            let out_t = 0.4 - popup.timer.remaining_secs();
            style.right = Val::Px(out_t * 800.0);
        } else {
            style.right = Val::Px(20.0);
        }

        if popup.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
