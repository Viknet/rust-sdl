extern crate sdl;
extern crate sdl_image;
extern crate sdl_ttf;

use std::path::Path;

use sdl::event::{Event, Key};
use sdl::video::{Color, SurfaceFlag, VideoFlag};

fn main() {
    sdl::init(&[sdl::InitFlag::Video]);
    sdl::wm::set_caption("rust-sdl demo - ttf", "rust-sdl");

    let screen = match sdl::video::set_video_mode(
        640,
        480,
        32,
        &[SurfaceFlag::HWSurface],
        &[VideoFlag::DoubleBuf],
    ) {
        Ok(screen) => screen,
        Err(err) => panic!("failed to set video mode: {}", err),
    };

    let _ = sdl_image::init(&[sdl_image::InitFlag::PNG]);
    let path = Path::new("background.png");
    let image_surface = sdl_image::load(&path).unwrap();

    let _ = sdl_ttf::init();
    let path = Path::new("lazy.ttf");
    //    let path = Path::new("examples/DroidSansFallbackFull.ttf");
    let font = sdl_ttf::open_font(&path, 28).unwrap();
    let color = Color::RGB(255, 255, 255);
    let text_surface =
        sdl_ttf::render_utf8_solid(&font, "Wonderful! 世界你好！".to_string(), &color).unwrap();

    let _ = screen.blit(&image_surface);
    let _ = screen.blit(&text_surface);

    screen.flip();

    'main: loop {
        'event: loop {
            match sdl::event::poll_event() {
                Event::Quit => break 'main,
                Event::None => break 'event,
                Event::Key(k, _, _, _) if k == Key::Escape => break 'main,
                _ => {}
            }
        }
    }

    sdl_ttf::close_font(font);
    sdl_ttf::quit();
    sdl_image::quit();
    sdl::quit();
}
