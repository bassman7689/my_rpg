extern crate sdl2;
extern crate specs;

mod resources;

use sdl2::event::Event;
use sdl2::image::InitFlag;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};

use resources::TextureManager;

struct Player {
    position: Point,
    sprite: Rect,
    current_frame: u32,
}

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    player: &Player,
    texture: &Texture,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let sprite_rect = Rect::new(
        player.sprite.x + (player.sprite.width() * player.current_frame) as i32,
        player.sprite.y,
        player.sprite.width(),
        player.sprite.height(),
    );

    let world_rect = Rect::new(
        player.position.x,
        player.position.y,
        (player.sprite.width() as f32 * 0.3) as u32,
        (player.sprite.height() as f32 * 0.3) as u32,
    );

    canvas.copy(&texture, sprite_rect, world_rect)?;
    canvas.present();
    Ok(())
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);
    let texture = texture_manager
        .load("assets/hero.png")
        .map_err(|e| e.to_string())?;

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 150, 182),
        current_frame: 0,
    };

    let mut i = 0;
    'running: loop {
        i += 1;
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if i % 600 == 0 {
            player.current_frame = (player.current_frame + 1) % 3;
        }

        render(
            &mut canvas,
            Color::RGBA(255, 255, 255, 255),
            &player,
            &texture,
        )?;
    }

    Ok(())
}
