# Installation
```sh
cargo add bevy_health_bar
```

```toml
bevy_health_bar = "0.1.0"
```

# Example
```sh
# Left Click | Right Click
cargo run --example basic
```
# Usage
```rs
use bevy::prelude::*;
use bevy_health_bar::{ProgressBar, ProgressBarBundle, ProgressBarPlugin};

fn main() {
    let mut app = App::new();
    // Add Plugin
    app.add_plugins((DefaultPlugins, ProgressBarPlugin));

    // Spawn A Progress Bar
    app.add_systems(Startup, spawn_health_bar);

    // Control Progress Bar
    app.add_systems(Update, update_health_bar);
    app.run();
}

fn spawn_health_bar(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Need a camera to see scene
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle);

    // Sets value (and max value) of health bar
    commands.spawn(ProgressBarBundle::new(
        100.0,
        asset_server.load("health_bar.png"),
    ));
}

fn update_health_bar(
    mut query: Query<&mut ProgressBar>,
    mouse_input: Res<Input<MouseButton>>,
    dt: Res<Time>,
) {
    if mouse_input.pressed(MouseButton::Left) {
        for mut health_bar in query.iter_mut() {
            health_bar.value -= 10.0 * dt.delta_seconds();
        }
    }
}

```

# Features
- [x] 0.1 Horizontal Progress Bar
- [ ] 0.2 Event Stream For Steps
- [ ] 0.3 Vertical Progress Bar
- [ ] 0.4 Background Image
- [ ] x.x Animated Progress Bars

# Credits
* mz_eth for example health bar art 