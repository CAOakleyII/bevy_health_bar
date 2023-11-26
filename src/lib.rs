use bevy::{
    app::{App, Plugin, Update},
    asset::{Assets, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Query, Res},
    },
    math::{Rect, Vec2},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

#[derive(Component, Default)]
pub struct ProgressBar {
    pub value: f32,
    pub max_value: f32,
    pub step: f32,
}

#[derive(Bundle, Default)]
pub struct ProgressBarBundle {
    pub progresss_bar: ProgressBar,

    pub sprite_bundle: SpriteBundle,
}

impl ProgressBarBundle {
    pub fn new(progress: f32, texture: Handle<Image>) -> Self {
        Self {
            progresss_bar: ProgressBar {
                value: progress,
                max_value: progress,
                ..Default::default()
            },
            sprite_bundle: SpriteBundle {
                texture,
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::CenterLeft,
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.sprite_bundle.transform = transform;
        self
    }
}
pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        // Probably Want Event Stream for steps
        app.add_systems(Update, Self::update);
    }
}

impl ProgressBarPlugin {
    fn update(
        mut query: Query<(&ProgressBar, &Handle<Image>, &mut Sprite)>,
        assets: Res<Assets<Image>>,
    ) {
        for (progress_bar, image_hanadle, mut sprite) in query.iter_mut() {
            let image = assets
                .get(image_hanadle)
                .expect(format!("Image {:?} not found", image_hanadle).as_str());
            let progress_percent =
                f32::clamp(progress_bar.value / progress_bar.max_value, 0.0, 1.0);
            let width = image.width() as f32;

            sprite.rect = Some(Rect {
                min: Vec2::ZERO,
                max: Vec2::new(width * progress_percent, image.height() as f32),
            });
            sprite.custom_size = Some(Vec2::new(width * progress_percent, image.height() as f32));
        }
    }
}
