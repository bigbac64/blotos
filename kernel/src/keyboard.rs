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
                /* Ctrl+A - début de ligne */
                DecodedKey::Unicode('\u{1}')  => {  }
                /* Ctrl+C - interruption   */
                DecodedKey::Unicode('\u{3}')  => {  }
                /* Ctrl+D - fin de fichier */
                DecodedKey::Unicode('\u{4}')  => {  }
                /* Ctrl+L - clear screen   */
                DecodedKey::Unicode('\u{C}')  => { 
                }
                /* Ctrl+Z - suspend        */
                DecodedKey::Unicode('\u{1A}') => {  }
                /* Enter                   */
                DecodedKey::Unicode('\r')     => {  }
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
                        KeyCode::LShift => {
                            dbg_println!("shift : {:?}", keycode);
                        }
                        _ => {
                            dbg_println!("no treatment : {:?}", keycode);
                        }
                    }
                }
            }
            dbg_println!("writing: {:?}", key);
        }
    }
}