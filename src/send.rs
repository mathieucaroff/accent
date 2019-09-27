use inputbot::{KeybdKey, KeybdKey::OtherKey};
use std::{thread::sleep, time::Duration};

use crate::hold::Hold;
use crate::structure::AltCode;

pub trait Send {
    fn send(&self);
}

// Windows {
const NUMPAD0_CODE: u64 = 0x60;
const ALT: KeybdKey = OtherKey(0x12);
// }

impl Send for KeybdKey {
    fn send(&self) {
        self.press();
        self.release();
    }
}

impl Send for AltCode<'_> {
    fn send(&self) {
        let persist;
        let digits = match self {
            AltCode::Digits(s) => *s,
            AltCode::Char(c) => {
                persist = (*c as u32).to_string();
                &persist
            }
        };
        ALT.hold(&mut || {
            for chr in digits.chars() {
                let digit = (chr as u64) - ('0' as u64);
                let key = OtherKey(NUMPAD0_CODE + digit);
                key.send();
                sleep(Duration::from_nanos(100));
            }
        });
    }
}
