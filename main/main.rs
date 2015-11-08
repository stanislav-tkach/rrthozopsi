extern crate sdl2;

fn main()
{
    let context = sdl2::init().unwrap();
    let video = context.video().unwrap();

    let window = video.window("rrthozopsi", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {..} => {
                    renderer.set_draw_color(sdl2::pixels::Color::RGB(255, 0, 0));
                    renderer.clear();
                    renderer.present();
                },
                _ => {}
            }
        }
    }
}
