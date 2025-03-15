use std::{thread, time::Duration};

use app::App;
use iced::{
    task::Handle,
    widget::{self, Image},
    Task,
};
use mouce::{Mouse, MouseActions};
use state::to_2d_index;
use tokio::time;

mod app;
mod message;
mod state;

#[tokio::main]
async fn main() -> Result<(), iced::Error> {
    let app = App::new();

    let state = app.state();
    tokio::spawn(async move {
        let mouse = Mouse::new();
        let mut interval = time::interval(Duration::from_millis(100));
        loop {
            interval.tick().await;
            if let Ok(mouse_position) = mouse.get_position() {
                match state.lock() {
                    Ok(mut state) => {
                        let section_size = state.section_size();
                        state.set_current_image(to_2d_index(
                            mouse_position.0 / section_size.0,
                            mouse_position.1 / section_size.1,
                            section_size.0,
                        ));
                    }
                    Err(error) => {
                        todo!("{error}")
                    }
                }
            }
        }
    });

    iced::application("rotatar", App::update, App::view).run_with(|| (app, Task::none()))?;
    Ok(())
}
