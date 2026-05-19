/*
By: <Your Name Here>
Date: 2026-05-19
Program Details: <Program Description Here>
*/

mod modules;
use crate::modules::still_image::StillImage;
use crate::modules::grid::draw_grid;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "trophyspin".to_string(),
        window_width: 1280,
        window_height: 960,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let center_x = 640.0;
    let center_y = 480.0;
    let radius = 400.0;
    let mut angle = 0.0;
    let img_dot = StillImage::new(
        "assets/black.png",
        5.0,
        5.0,
        640.0,
        480.0,
        true,
        1.0
    ).await;
    let img_trophy = StillImage::new(
        "assets/trophy.png",
        100.0,
        150.0,
        150.0,
        50.0,
        true,
        1.0
    ).await;
    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();
        img_dot.x = x;
        img_dot.y = y;
        img_trophy.draw();
        img_dot.draw();
        

        next_frame().await;
    }
}
