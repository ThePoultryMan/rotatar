use std::{thread, time::Duration};

use mouce::{Mouse, MouseActions};

struct State {
    screen_size: (i32, i32),
    height_sections: i32,
    width_sections: i32,
}

fn main() {
    let mouse = Mouse::new();

    let state = State {
        screen_size: (1920, 1080),
        height_sections: 3,
        width_sections: 3,
    };

    let x_sections_size = state.screen_size.0 / state.width_sections;
    let y_sections_size = state.screen_size.1 / state.height_sections;

    loop {
        thread::sleep(Duration::from_millis(100));
        if let Ok(position) = mouse.get_position() {
            let image_x = position.0 / x_sections_size;
            let image_y = position.1 / y_sections_size;

            println!("({image_x}, {image_y})");
        }
    }
}
