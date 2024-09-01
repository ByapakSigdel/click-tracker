use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use log::{info};
use std::collections::HashMap;

pub fn start_tracking() {
    let device_state = DeviceState::new();
    let mut key_pressed = Vec::new();
    let mut key_count: HashMap<String, u32> = HashMap::new();
    let mut mouse_clicks = 0;

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let mouse: MouseState = device_state.get_mouse();

        // Log mouse clicks
        if mouse.button_pressed[0] || mouse.button_pressed[1] || mouse.button_pressed[2] {
            mouse_clicks += 1;
            info!("Mouse Clicks: {}", mouse_clicks);
        }

        // Log key presses
        if keys != key_pressed {
            for key in &keys {
                let key_str = format!("{:?}", key);
                let counter = key_count.entry(key_str.clone()).or_insert(0);
                *counter += 1;
                info!("Key Pressed: {:?}, Total: {}", key, counter);
            }
            key_pressed = keys;
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
