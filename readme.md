# 💂‍♂️ Beefeater

Generic guard around any type that implements Copy.

## Usage

Add a dependency to your `Cargo.toml`:

```toml
[dependencies]
beefeater = "0.1"
```

Add a use declaration to import the crate:

```rust
use beefeater::Beefeater;
```

Finally, use the 💂‍♂️ Beefeater to guard access:

```rust
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
```
