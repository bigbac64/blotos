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
    let mut registry = crate::window::WINDOW_REGISTRY.lock();
    let terminal = registry.get_mut(&core::any::TypeId::of::<crate::terminal::Terminal>(), 0)
        .unwrap().as_any_mut().downcast_mut::<crate::terminal::Terminal>().unwrap();


    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode('\x08') => {
                    terminal.remove_char();
                }, // Backspace en ASCII (lié a la comptabilité de traitement des ces raw code)
                DecodedKey::Unicode('\x1B') => print!("<Escape>"),   // Escape en ASCII
                DecodedKey::Unicode('\x7F') => print!("<Delete>"),   // Delete en ASCII (parfois)
                DecodedKey::Unicode(character) => {
                    print!("{}", character);
                }
                DecodedKey::RawKey(keycode) => {
                    match keycode {
                        KeyCode::Backspace => {
                            terminal.remove_char();
                        }
                        _ => {
                            print!("{:?}", keycode);
                        }
                    }
                }
            }
        }
    }
}