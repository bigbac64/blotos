use core::any::TypeId;
use core::fmt::Write;
use crate::spin_lock::SpinLock;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use crate::dbg_println;
use crate::terminal::Terminal;

static KEYBOARD: SpinLock<Keyboard<layouts::Azerty, ScancodeSet1>> = SpinLock::new(
    Keyboard::new(
        ScancodeSet1::new(),
        layouts::Azerty,
        HandleControl::MapLettersToUnicode
    )
);

pub fn process_scancode(scancode: u8) {
    let mut key = {
        let mut keyboard = KEYBOARD.lock();
        keyboard.add_byte(scancode)
            .ok()
            .flatten()
            .and_then(|event| keyboard.process_keyevent(event))
    };
    dbg_println!("KEYBOARD locked {}", KEYBOARD.is_locked());

    if let Some(key) = key {
        dbg_println!("WINDOW_REGISTRY locked {}", crate::window::WINDOW_REGISTRY.is_locked());
        let mut registry = crate::window::WINDOW_REGISTRY.lock();
        dbg_println!("WINDOW_REGISTRY locked {}", crate::window::WINDOW_REGISTRY.is_locked());

        dbg_println!("registry size: {}", registry.register.len());
        dbg_println!("terminal exists: {}",
            registry.register.contains_key(&TypeId::of::<Terminal>()));

        let terminal = registry
            .get_mut(&TypeId::of::<Terminal>(), 0)
            .and_then(|w| w.as_any_mut().downcast_mut::<Terminal>());

        dbg_println!("terminal set");

        if let Some(terminal) = terminal {
            dbg_println!("pass");
            match key {
                DecodedKey::Unicode('\x08') => {
                    terminal.remove_char();
                }, // Backspace en ASCII (lié a la comptabilité de traitement des ces raw code)
                DecodedKey::Unicode('\x1B') => todo! (),   // Escape en ASCII
                DecodedKey::Unicode('\x7F') => todo! (),   // Delete en ASCII (parfois)
                DecodedKey::Unicode(character) => {
                    dbg_println!("writing: {}", character);
                    terminal.write_char(character).expect("Aie coup dur pour guillaume");

                    dbg_println!("action added");
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
        }

        dbg_println!("end ok");
    }
}