use std::ffi::CString;

use raylib::prelude::*;

const FONT_SIZE: f32 = 20.0;
const MIN_WIDTH: i32 = 800;
const MIN_HEIGHT: i32 = 600;

const BG_COLOR: Color = Color::new(46, 52, 64, 1);
const FG_COLOR: Color = Color::WHITE;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("THe best text editor")
        .resizable()
        .size(MIN_WIDTH, MIN_HEIGHT)
        .build();

    rl.set_window_min_size(MIN_WIDTH, MIN_HEIGHT);

    let font = rl
        .load_font_ex(&thread, "BlexMonoNerdFont-Text.ttf", FONT_SIZE as i32, None)
        .expect("Couldn't load font");

    let mut text = "Hello, world!".to_owned();
    rl.set_target_fps(60);
    let mut last_back_space = 0.0;
    let mut text_changed;
    let mut cur_tup = locate_cursor(&text, &rl, &font);
    rl.set_text_line_spacing(FONT_SIZE as i32);
    while !rl.window_should_close() {
        text_changed = false;
        let curr_time = rl.get_time();
        if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
            if (curr_time - last_back_space) >= 0.1 {
                text.pop();
                text_changed = true;
                last_back_space = curr_time;
            }
        } else {
            last_back_space = 0.0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            text.push('\n');
            text_changed = true;
        }
        match rl.get_char_pressed() {
            Some(key) => {
                text.push(key);
                text_changed = true;
            }
            _ => {}
        };

        if text_changed {
            cur_tup = locate_cursor(&text, &rl, &font);
        }
        let mut d = rl.begin_drawing(&thread);
        d.draw_text_ex(
            &font,
            "|",
            Vector2::new(cur_tup.x + 12.0, cur_tup.y + 12.0),
            FONT_SIZE,
            1.0,
            Color::WHITE,
        );

        d.clear_background(BG_COLOR);
        d.draw_text_ex(
            &font,
            text.as_str(),
            Vector2::new(12.0, 12.0),
            FONT_SIZE,
            1.0,
            FG_COLOR,
        );
    }
}

fn locate_cursor(text: &String, _: &RaylibHandle, font: &Font) -> Vector2 {
    let mut last_new_line_idx = 0;
    let mut line_count = 0;

    let text_chars: Vec<char> = text.chars().collect();

    for i in 0..text_chars.len() {
        if text_chars[i] == '\n' {
            last_new_line_idx = i;
            line_count += 1;
        }
    }

    let last_line: String = text_chars[last_new_line_idx..].iter().collect();

    let text_size = unsafe {
        let c_text = CString::new(last_line.as_str()).unwrap();
        ffi::MeasureTextEx(**font, c_text.as_ptr(), FONT_SIZE, 1.0)
    };

    Vector2::new(
        text_size.x as f32,
        (text_size.y - FONT_SIZE) * (line_count as f32),
    )
}
