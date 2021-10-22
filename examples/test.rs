use inputbot::{BlockInput::*, KeybdKey::*, MouseButton::*, *};
use std::{thread::sleep, time::Duration};

fn main() {
    SpaceKey.bind(|| {
        LeftKey.press();
        sleep(Duration::from_millis(100));
        LeftKey.release();
    });
    // Call this to start listening for bound inputs.
    handle_input_events();
}
