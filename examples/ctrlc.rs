use std::sync::Arc;
use beefeater::Beefeater;
use ctrlc;

#[derive(Clone, Copy)]
enum State {
    Playing,
    Stopped,
}

fn main() {
    let state = Arc::new(Beefeater::new(State::Playing));

    {
        let state = state.clone();

        ctrlc::set_handler(move || {
            println!("Recevied SIGINT, stopping playback");
            state.store(State::Stopped);
        }).unwrap();
    }

    loop {
        match state.load() {
            State::Playing => {
                // Feed next chunk
            }
            State::Stopped => {
                break;
            }
        }
    }
}
