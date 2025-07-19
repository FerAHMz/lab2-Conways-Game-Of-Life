mod framebuffer;
mod game_of_life;
mod patterns;

use framebuffer::Framebuffer;
use game_of_life::GameOfLife;
use patterns::load_all_patterns;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let (screen_width, screen_height) = (600, 600);
    let cell_size = 1;
    let grid_width = screen_width / cell_size;
    let grid_height = screen_height / cell_size;

    let mut fb = Framebuffer::new(grid_width as u32, grid_height as u32, Color::BLACK);
    let mut game = GameOfLife::new(grid_width, grid_height);

    load_all_patterns(&mut game);

    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(10);

    while !rl.window_should_close() {
        game.update();

        fb.clear();
        fb.set_current_color(Color::RED);
        game.render(&mut fb);

        let texture = rl.load_texture_from_image(&thread, &fb.color_buffer).unwrap();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        thread::sleep(Duration::from_millis(100));
    }

}
