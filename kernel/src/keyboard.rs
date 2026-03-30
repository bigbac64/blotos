use crate::spin_lock::SpinLock;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};

static KEYBOARD: SpinLock<Keyboard<layouts::Azerty, ScancodeSet1>> = SpinLock::new(
    Keyboard::new(
        ScancodeSet1::new(),
        layouts::Azerty,
        HandleControl::MapLettersToUnicode
    )
);

pub fn process_scancode(scancode: u8) {
    let mut keyboard = KEYBOARD.lock();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode('\x08') => {
                    let mut guard = crate::DISPLAY.lock();
                    if let Some(terminal) = guard.as_mut() {
                        terminal.remove_char();
                    }
                }, // Backspace en ASCII (lié a la comptabilité de traitement des ces raw code)
                DecodedKey::Unicode('\x1B') => print!("<Escape>"),   // Escape en ASCII
                DecodedKey::Unicode('\x7F') => print!("<Delete>"),   // Delete en ASCII (parfois)
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                }
                DecodedKey::RawKey(keycode) => {
                    match keycode {
                        KeyCode::Backspace => {
                            let mut guard = crate::DISPLAY.lock();
                            if let Some(terminal) = guard.as_mut() {
                                terminal.remove_char();
                            }
                        }
                        _ => {
                            print!("{:?}", keycode);
                        }
                    }
                }
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                }
            }
        }
    }
}