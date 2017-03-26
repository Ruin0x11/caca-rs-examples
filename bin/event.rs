extern crate caca;

use std::collections::VecDeque;
use std::time::Duration;

use caca::*;

fn main() {
    let mut canvas = CacaCanvas::new(80, 24)
        .expect("Failed to create canvas");

    let mut display = CacaDisplay::new(InitOptions { canvas: Some(&canvas),
                                                     ..InitOptions::default()})
        .expect("Failed to create display");

    let height = canvas.height() - 1;
    let width = canvas.width() - 1;
    let mut events = VecDeque::new();

    canvas.set_color_ansi(AnsiColor::White, AnsiColor::Blue);
    canvas.draw_line(0, 0, width, 0, ' ');
    canvas.draw_line(0, height, width, height, ' ');
    canvas.put_str(0, height, "type \"quit\" to exit");

    display.refresh();

    let mut quit = 0;

    while quit != 4 {
        let mut event_opt = display.poll_event(EVENT_ANY.bits());
        while event_opt.is_some() {
            let event = event_opt.unwrap();
            match event {
                Event::KeyPress(key) => match key {
                    Key::Char(c) => match c {
                        'q' if quit == 0 => quit = 1,
                        'u' if quit == 1 => quit = 2,
                        'i' if quit == 2 => quit = 3,
                        't' if quit == 3 => quit = 4,
                        'q' => quit = 1,
                        _   => quit = 0,
                    },
                    _ => (),
                },
                _ => (),
            }
            events.push_front(event);
            event_opt = display.peek_event(EVENT_ANY.bits(), Duration::new(0, 0));
        }
        canvas.set_color_ansi(AnsiColor::LightGray, AnsiColor::Black);
        canvas.clear();

        canvas.set_color_ansi(AnsiColor::White, AnsiColor::Blue);
        canvas.draw_line(0, 0, width, 0, ' ');
        if let Some(first) = events.iter().next() {
            canvas.put_str(0, 0, format!("{:?}", first).as_str());
        }

        canvas.draw_line(0, height, width, height, ' ');
        canvas.put_str(0, height, format!("type \"quit\" to exit: {}", quit_string(quit)).as_str());

        canvas.set_color_ansi(AnsiColor::White, AnsiColor::Black);
        for (i, event) in events.iter().skip(1).enumerate() {
            let idx = i as i32 + 1;
            if idx < height {
                canvas.put_str(0, idx, format!("{:?}", event).as_str());
            }
        }
        display.refresh();
    }
}

fn quit_string(quit: i32) -> &'static str {
    match quit {
        0 => "",
        1 => "q",
        2 => "qu",
        3 => "qui",
        4 => "quit",
        _ => "",
    }
}
