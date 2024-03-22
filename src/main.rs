use sdl2::{event::Event};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::{path::Path, time::Duration};
use ace::rope::{*};
fn main() -> Result<(), String>{
    // pkg_config::probe_library
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
                .window("Ace", 800, 600)
                .position_centered()
                .opengl()
                .build()
                .map_err(|e| e.to_string())?;
 
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();
    canvas.present();
    let mut user_input_text = String::from(" ");
    let mut event_pump = sdl_context.event_pump()?;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::TextInput { text, ..} => {
                    user_input_text += &text;
                },
                Event::KeyDown{keycode: Some(Keycode::Backspace ), ..} => {
                    user_input_text.pop();
                },
                _ => {}
            }
        }

        canvas.clear();
        let font_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font_path = Path::new("asset/fonts/Raleway-VariableFont_wght.ttf");
        let mut raleway_font = font_context.load_font(font_path, 24).unwrap();
        let surface = raleway_font.render(&user_input_text).blended(Color::RGB(255,160,122)).unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let texture_query = texture.query();
        let texture_rec = Rect::new(0, 0, texture_query.width, texture_query.height);
        canvas.copy(&texture, None,  texture_rec).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...

    }
    return Ok(());
}
