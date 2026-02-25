use crate::spin_lock::SpinLock;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

static KEYBOARD: SpinLock<Keyboard<layouts::Us104Key, ScancodeSet1>> = SpinLock::new(
    Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore
    )
);

pub fn process_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();
    println!("pressed !");
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                }
                DecodedKey::RawKey(key) => {
                    print!("{:?}", key);
                }
            }
        }
    }
}