use super::{DisplayQuality, GameState, TEXT_COLOR, Volume, despawn_screen};
use bevy::{
    color::palettes::basic::{BLUE, LIME},
    prelude::*,
};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands, display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnGameScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            children![
                (
                    Text::new("Will be back to the menu shortly..."),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                ),
                (
                    Text::default(),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                    children![
                        (
                            TextSpan(format!("quality: {:?}", *display_quality)),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(BLUE.into()),
                        ),
                        (
                            TextSpan::new(" - "),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ),
                        (
                            TextSpan(format!("volume: {:?}", *volume)),
                            TextFont {
                                font_size: 50.0,
                                ..default()
                            },
                            TextColor(LIME.into()),
                        ),
                    ]
                ),
            ]
        )],
    ));

    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
