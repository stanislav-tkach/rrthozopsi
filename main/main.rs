extern crate sdl2;

mod events;

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
    let mut events = events::Events::new(context.event_pump().unwrap());

    loop {
        events.pump();

        if events.quit || events.key_escape {
            break;
        }

        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();
    }
}
