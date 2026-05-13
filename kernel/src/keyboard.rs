use core::any::TypeId;
use core::fmt::Write;
use crate::spin_lock::SpinLock;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use crate::dbg_println;
use crate::gui::terminal::Terminal;
use crate::gui::window::WINDOW_REGISTRY;

static KEYBOARD: SpinLock<Keyboard<layouts::Azerty, ScancodeSet1>> = SpinLock::new(
    Keyboard::new(
        ScancodeSet1::new(),
        layouts::Azerty,
        HandleControl::MapLettersToUnicode
    )
);

pub fn process_scancode(scancode: u8) {
    let key = {
        let mut keyboard = KEYBOARD.lock();
        keyboard.add_byte(scancode)
            .ok()
            .flatten()
            .and_then(|event| keyboard.process_keyevent(event))
    };

    if let Some(key) = key {
        let mut registry = WINDOW_REGISTRY.lock();

        let terminal = registry
            .get_mut(&TypeId::of::<Terminal>(), 0)
            .and_then(|w| w.as_any_mut().downcast_mut::<Terminal>());

        if let Some(terminal) = terminal {
            match key {
                DecodedKey::Unicode('\x08') => {
                    terminal.remove_char();
                }, // Backspace en ASCII (lié a la comptabilité de traitement des ces raw code)
                DecodedKey::Unicode('\x1B') => todo! (),   // Escape en ASCII
                DecodedKey::Unicode('\x7F') => todo! (),   // Delete en ASCII (parfois)
                DecodedKey::Unicode(character) => {
                    terminal.write_char(character).expect("Aie coup dur pour guillaume");

                }
                DecodedKey::RawKey(keycode) => {
                    match keycode {
                        KeyCode::Backspace => {
                            terminal.remove_char();
                        }
                        _ => {
                            terminal.write_str("c'est quand meme mieux print").expect("aled");
                        }
                    }
                }
            }
            dbg_println!("writing: {:?}", key);
        }
    }
}