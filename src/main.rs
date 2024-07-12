use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let device_state = DeviceState::new();
    let mut pressed_keys = device_state.get_keys();

    loop {
        let current_keys = device_state.get_keys();
        
        for key in current_keys.iter() {
            if !pressed_keys.contains(key) {
                println!("Key pressed: {:?}", key);
            }
        }

        for key in pressed_keys.iter() {
            if !current_keys.contains(key) {
                println!("Key released: {:?}", key);
            }
        }

        pressed_keys = current_keys;
    }
}
