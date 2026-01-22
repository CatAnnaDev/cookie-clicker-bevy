use crate::components::*;
use crate::ui_fonts::UiFonts;
use crate::ui_icons::icon;
use bevy::asset::AssetPath;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::resources::GameState;
pub fn text(
    value: &str,
    font: Handle<Font>,
    size: f32,
    color: Color,
) -> TextBundle {
    TextBundle::from_section(
        value,
        TextStyle {
            font,
            font_size: size,
            color,
        },
    )
}

pub fn emoji(
    value: &str,
    fonts: &UiFonts,
    size: f32,
    color: Color,
) -> TextBundle {
    TextBundle::from_section(
        value,
        TextStyle {
            font: fonts.emojis.clone(),
            font_size: size,
            color,
        },
    )
}


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
            background_color: Color::srgb(0.08, 0.06, 0.12).into(),
            ..default()
        })
        .with_children(|root| {
            // =========================
            // LEFT COLUMN
            // =========================
            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(65.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(24.0)),
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                ..default()
            })
                .with_children(|left| {
                    // Container principal
                    left.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    })
                        .with_children(|main_content| {
                            // Contenu principal (cookie + stats)
                            main_content.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(90.0),
                                    flex_direction: FlexDirection::Column,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(32.0)),
                                    row_gap: Val::Px(12.0),
                                    ..default()
                                },
                                background_color: Color::srgba(0.15, 0.12, 0.22, 0.6).into(),
                                border_radius: BorderRadius::all(Val::Px(24.0)),
                                ..default()
                            })
                                .with_children(|header| {
                                    // TITLE avec icône
                                    header
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                column_gap: Val::Px(12.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|row| {
                                            icon(row, &assets, AssetPath::from("ui/icons/cookie.png"), 48.0);
                                            row.spawn(text("COOKIE EMPIRE", fonts.bold.clone(), 56.0, Color::srgb(1.0, 0.85, 0.4))
                                            );
                                        });

                                    // Milestone
                                    header.spawn((
                                        TextBundle::from_section(
                                            "",
                                            TextStyle {
                                                font: fonts.semibold.clone(),

                                                font_size: 24.0,
                                                color: Color::srgb(0.7, 0.9, 1.0),
                                            },
                                        ),
                                        MilestoneText,
                                    ));

                                    // COUNTER PRINCIPAL
                                    header.spawn((
                                        TextBundle::from_section(
                                            "0",
                                            TextStyle {
                                                font: fonts.bold.clone(),
                                                font_size: 72.0,
                                                color: Color::srgb(1.0, 1.0, 1.0),
                                            },
                                        ),
                                        CookieCounter,
                                    ));

                                    // Label "cookies"
                                    // header.spawn(TextBundle::from_section(
                                    //     "cookies",
                                    //     TextStyle {
                                    //         font: fonts.regular.clone(),
                                    //         font_size: 28.0,
                                    //         color: Color::srgb(0.6, 0.6, 0.7),
                                    //     },
                                    // ));

                                    // CPS
                                    header
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                column_gap: Val::Px(8.0),
                                                padding: UiRect::all(Val::Px(12.0)),
                                                margin: UiRect::top(Val::Px(8.0)),
                                                ..default()
                                            },
                                            background_color: Color::srgba(0.2, 0.8, 0.5, 0.15).into(),
                                            border_radius: BorderRadius::all(Val::Px(16.0)),
                                            ..default()
                                        })
                                        .with_children(|row| {
                                            //row.spawn(emoji("⚡", &fonts, 20.0, Color::srgb(0.3, 0.9, 0.6)));
                                            row.spawn((
                                                text("0 per second", fonts.semibold.clone(), 20.0, Color::srgb(0.3, 0.9, 0.6), ),
                                                CpsCounter,
                                            ));
                                        });

                                    // COMBO
                                    header.spawn((
                                        TextBundle::from_section(
                                            "",
                                            TextStyle {
                                                font: fonts.bold.clone(),
                                                font_size: 28.0,
                                                color: Color::srgb(1.0, 0.3, 0.4),
                                            },
                                        ),
                                        ComboText,
                                    ));

                                    // COOKIE BUTTON
                                    header
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Px(220.0),
                                                    height: Val::Px(220.0),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    margin: UiRect::top(Val::Px(24.0)),
                                                    border: UiRect::all(Val::Px(6.0)),
                                                    ..default()
                                                },
                                                background_color: Color::srgb(0.95, 0.65, 0.3).into(),
                                                border_color: Color::srgb(1.0, 0.85, 0.5).into(),
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
                                                    width: Val::Px(140.0),
                                                    height: Val::Px(140.0),
                                                    ..default()
                                                },
                                                image: UiImage::new(assets.load("ui/icons/cookie.png")),
                                                ..default()
                                            });
                                        });
                                });

                            // FOOTER CARDS (Stats, Achievements, Prestige)
                            main_content.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Val::Px(16.0),
                                    ..default()
                                },
                                ..default()
                            })
                                .with_children(|footer| {
                                    // 1. STATS CARD
                                    footer
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(33.33),
                                                flex_direction: FlexDirection::Column,
                                                padding: UiRect::all(Val::Px(18.0)),
                                                row_gap: Val::Px(8.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::srgba(0.15, 0.12, 0.22, 0.6).into(),
                                            border_radius: BorderRadius::all(Val::Px(20.0)),
                                            ..default()
                                        })
                                        .with_children(|card| {
                                            card.spawn(emoji("📊", &fonts, 18.0, Color::srgb(0.7, 0.85, 1.0)));
                                            card.spawn(TextBundle::from_section(
                                                "Stats",
                                                TextStyle {
                                                    font: fonts.bold.clone(),
                                                    font_size: 18.0,
                                                    color: Color::srgb(0.7, 0.85, 1.0),
                                                },
                                            ));
                                            card.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: fonts.regular.clone(),
                                                        font_size: 14.0,
                                                        color: Color::srgb(0.7, 0.7, 0.8),
                                                    },
                                                ),
                                                StatsText,
                                            ));
                                        });

                                    // 2. ACHIEVEMENTS CARD
                                    footer
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(33.33),
                                                flex_direction: FlexDirection::Column,
                                                padding: UiRect::all(Val::Px(18.0)),
                                                row_gap: Val::Px(8.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::srgba(0.15, 0.12, 0.22, 0.6).into(),
                                            border_radius: BorderRadius::all(Val::Px(20.0)),
                                            ..default()
                                        })
                                        .with_children(|card| {
                                            card.spawn(emoji("🏆", &fonts, 18.0, Color::srgb(1.0, 0.8, 0.3)));
                                            card.spawn(TextBundle::from_section(
                                                "Achievements",
                                                TextStyle {
                                                    font: fonts.bold.clone(),
                                                    font_size: 18.0,
                                                    color: Color::srgb(1.0, 0.8, 0.3),
                                                },
                                            ));
                                            card.spawn((
                                                TextBundle::from_section(
                                                    "0/0",
                                                    TextStyle {
                                                        font: fonts.semibold.clone(),
                                                        font_size: 20.0,
                                                        color: Color::srgb(1.0, 0.8, 0.3),
                                                    },
                                                ),
                                                AchievementText,
                                            ));
                                        });

                                    // 3. PRESTIGE CARD
                                    footer
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(33.33),
                                                flex_direction: FlexDirection::Column,
                                                padding: UiRect::all(Val::Px(18.0)),
                                                row_gap: Val::Px(10.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::srgba(0.15, 0.12, 0.22, 0.6).into(),
                                            border_radius: BorderRadius::all(Val::Px(20.0)),
                                            ..default()
                                        })
                                        .with_children(|card| {
                                            card.spawn((
                                                TextBundle::from_section(
                                                    "",
                                                    TextStyle {
                                                        font: fonts.semibold.clone(),
                                                        font_size: 16.0,
                                                        color: Color::srgb(0.9, 0.7, 1.0),
                                                    },
                                                ),
                                                PrestigeText,
                                            ));

                                            card.spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                                                        column_gap: Val::Px(8.0),
                                                        ..default()
                                                    },
                                                    background_color: Color::srgb(0.6, 0.3, 0.8).into(),
                                                    border_radius: BorderRadius::all(Val::Px(12.0)),
                                                    ..default()
                                                },
                                                PrestigeButton,
                                            ))
                                                .with_children(|btn| {
                                                    icon(btn, &assets, AssetPath::from("ui/icons/star.png"), 18.0);
                                                    btn.spawn(TextBundle::from_section(
                                                        "PRESTIGE",
                                                        TextStyle {
                                                            font: fonts.bold.clone(),
                                                            font_size: 16.0,
                                                            color: Color::WHITE,
                                                        },
                                                    ));
                                                });
                                        });
                                });
                        });
                });

                    // =========================
                    // RIGHT COLUMN (UPGRADES)
                    // =========================
                    root.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(35.0),
                            height: Val::Percent(100.0),
                            padding: UiRect::all(Val::Px(24.0)),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(16.0),
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.08, 0.15, 0.4).into(),
                        ..default()
                    })
                        .with_children(|right| {
                            section(
                                right,
                                &assets,
                                &fonts,
                                "⚡",
                                &format!("POWER UPS [{}]", game_state.powerups.len()),
                                AssetPath::from("ui/icons/power.png"),
                                game_state.powerups.len(),
                                true,
                            );
                            section(
                                right,
                                &assets,
                                &fonts,
                                "🏭",
                                &format!("BUILDINGS [{}]", game_state.upgrades.len()),
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
                    scrolling_list.position = scrolling_list.position.clamp(-5000.0, 0.0);
                    style.top = Val::Px(scrolling_list.position);
                }
            }
        }
    }

    fn section(
        parent: &mut ChildBuilder,
        _assets: &AssetServer,
        fonts: &UiFonts,
        emoji_title: &str,
        title: &str,
        _icon_path: AssetPath,
        count: usize,
        is_powerup: bool,
    ) {
        parent
            .spawn((NodeBundle {
                style: Style {
                    height: Val::Percent(48.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(12.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.12, 0.22, 0.5).into(),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                ..default()
            }, ScrollingList { position: 0.0 },))
            .with_children(|section| {
                // HEADER
                //section.spawn(emoji(emoji_title, fonts, 24.0, Color::srgb(0.9, 0.9, 1.0)));
                section.spawn(TextBundle::from_sections([
                    TextSection::new(
                        emoji_title,
                        TextStyle {
                            font: fonts.emojis.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.7, 0.85, 1.0),
                        },
                    ),
                    TextSection::new(
                        title,
                        TextStyle {
                            font: fonts.bold.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.7, 0.85, 1.0),
                        },
                    ),
                ]));

                // LIST SCROLLABLE
                section
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(8.0),
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
                                            padding: UiRect::all(Val::Px(14.0)),
                                            border: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.2, 0.18, 0.28).into(),
                                        border_color: Color::srgba(0.5, 0.4, 0.7, 0.3).into(),
                                        border_radius: BorderRadius::all(Val::Px(12.0)),
                                        ..default()
                                    },
                                    PowerUpButton { powerup_index: i },
                                )).insert(Interaction::default())
                                    .with_children(|btn| {
                                        btn.spawn((
                                            TextBundle::from_section(
                                                "Power Up",
                                                TextStyle {
                                                    font: fonts.semibold.clone(),
                                                    font_size: 15.0,
                                                    color: Color::srgb(0.9, 0.9, 1.0),
                                                },
                                            ),
                                            PowerUpText { powerup_index: i },
                                        ));
                                    });
                            } else {
                                list.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            padding: UiRect::all(Val::Px(14.0)),
                                            border: UiRect::all(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.2, 0.18, 0.28).into(),
                                        border_color: Color::srgba(0.4, 0.6, 0.5, 0.3).into(),
                                        border_radius: BorderRadius::all(Val::Px(12.0)),
                                        ..default()
                                    },
                                    UpgradeButton { upgrade_index: i },
                                ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            TextBundle::from_section(
                                                "Building",
                                                TextStyle {
                                                    font: fonts.semibold.clone(),
                                                    font_size: 15.0,
                                                    color: Color::srgb(0.9, 0.9, 1.0),
                                                },
                                            ),
                                            UpgradeText { upgrade_index: i },
                                        ));
                                    });
                            }
                        }
                    });
            });
    }

    pub fn spawn_achievement_popup(
        commands: &mut Commands,
        _emoji: &str,
        name: &str,
        desc: &str,
        offset_y: f32,
        title: &str,
    ) {
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(-350.0),
                    top: Val::Px(40.0 + offset_y),
                    width: Val::Px(330.0),
                    height: Val::Px(90.0),
                    padding: UiRect::all(Val::Px(16.0)),
                    column_gap: Val::Px(16.0),
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.1, 0.2, 0.98).into(),
                border_color: Color::srgb(1.0, 0.7, 0.2).into(),
                border_radius: BorderRadius::all(Val::Px(16.0)),
                z_index: ZIndex::Global(200),
                ..default()
            },
            AchievementPopup {
                timer: Timer::from_seconds(4.0, TimerMode::Once),
                index: 0,
            },
        ))
            .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(50.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(1.0, 0.7, 0.2).into(),
                    border_radius: BorderRadius::all(Val::Percent(50.0)),
                    ..default()
                })
                    .with_children(|icon| {
                        icon.spawn(TextBundle::from_section(
                            "🏆",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });

                // Texte
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    ..default()
                })
                    .with_children(|text_col| {
                        text_col.spawn(TextBundle::from_section(
                            title,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::srgb(1.0, 0.7, 0.2),
                                ..default()
                            },
                        ));
                        text_col.spawn(TextBundle::from_section(
                            name,
                            TextStyle {
                                font_size: 18.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                        text_col.spawn(TextBundle::from_section(
                            desc,
                            TextStyle {
                                font_size: 15.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            });
    }

    pub fn achievement_popup_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(Entity, &mut Style, &mut AchievementPopup)>,
    ) {
        for (entity, mut style, mut popup) in &mut query {
            popup.timer.tick(time.delta());
            let t = popup.timer.elapsed_secs();

            if t < 0.5 {
                let progress = t / 0.5;
                let bounce = progress * progress * (3.0 - 2.0 * progress);
                style.right = Val::Px(-350.0 + bounce * 370.0);
            } else if popup.timer.remaining_secs() < 0.5 {
                let out_t = 0.5 - popup.timer.remaining_secs();
                style.right = Val::Px(20.0 + out_t * 370.0);
            } else {
                style.right = Val::Px(20.0);
            }

            if popup.timer.finished() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }