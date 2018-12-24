extern crate amethyst;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

use std::time::Duration;

mod dogfight;
mod level;
mod plane;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir();

    let path = format!("{}/resources/display_config.ron", app_root);
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new()),
    );

    use crate::dogfight::Dogfight;

    let assets_dir = app_root;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::build(assets_dir, Dogfight)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();

    Ok(())
}
