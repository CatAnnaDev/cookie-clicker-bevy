use crate::components::*;
use crate::ui_fonts::UiFonts;
use crate::ui_icons::icon;
use crate::ui_theme::UiTheme;
use bevy::asset::AssetPath;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, assets: Res<AssetServer>, fonts: Res<UiFonts>) {
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
                    6,
                    true,
                );
                section(
                    right,
                    &assets,
                    &fonts,
                    "BUILDINGS",
                    AssetPath::from("ui/icons/building.png"),
                    15,
                    false,
                );
            });
        });
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent)>,
    query_node: Query<&Interaction>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (mut scrolling_list, mut style, parent) in &mut query_list {
            if let Ok(interaction) = query_node.get(parent.get()) {
                if *interaction == Interaction::Hovered {
                    scrolling_list.position += mouse_wheel_event.y * 25.0;
                    scrolling_list.position = scrolling_list.position.clamp(-2000.0, 0.0);
                    style.top = Val::Px(scrolling_list.position);
                }
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
                ))
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
