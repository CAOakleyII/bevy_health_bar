use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_health_bar::{ProgressBar, ProgressBarBundle, ProgressBarPlugin};

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

fn main() {
    let mut app = App::new();
    // Add Plugin
    app.add_plugins((DefaultPlugins, ProgressBarPlugin));

    // Game State, Used in example to load
    // asset and set texture to nearest
    app.add_state::<GameState>();

    // Spawn A Progress Bar
    app.add_systems(Startup, spawn_progress_bar);
    app.add_systems(Update, load_assets.run_if(in_state(GameState::Loading)));

    // Systems to Control Progress Bar
    app.add_systems(
        Update,
        update_progress_bar.run_if(in_state(GameState::Playing)),
    );
    app.run();
}

// Example of spawning a progress bar
fn spawn_progress_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    commands.spawn(camera_bundle);

    let health_bar_bundle = ProgressBarBundle::new(100.0, asset_server.load("health_bar.png"));
    commands.spawn(health_bar_bundle);
}

// Example of loading assets and setting texture to nearest
fn load_assets(
    mut image_events: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for event in image_events.read() {
        match event {
            AssetEvent::Added { id } => {
                let image = assets
                    .get_mut(*id)
                    .expect("Failed to retrieve added image.");
                image.sampler = ImageSampler::nearest();
                state.set(GameState::Playing);
            }
            _ => {}
        }
    }
}

// Example of updating progress bar
fn update_progress_bar(
    mut query: Query<&mut ProgressBar>,
    mouse_input: Res<Input<MouseButton>>,
    dt: Res<Time>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        for mut progress_bar in query.iter_mut() {
            progress_bar.value -= 50.0 * dt.delta_seconds();
        }
    } else if mouse_input.pressed(MouseButton::Right) {
        for mut progress_bar in query.iter_mut() {
            progress_bar.value += 50.0 * dt.delta_seconds();
        }
    }
}
