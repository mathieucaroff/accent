use inputbot::{KeybdKey::*, *};
use std::env::args;
use std::process::exit;

mod altcode_from;
mod hold;
mod is_pressed;
mod send;
mod structure;

use is_pressed::*;
use send::*;
use structure::*;

// Windows {
const COMMA: KeybdKey = OtherKey(0xBC);
const PERIOD: KeybdKey = OtherKey(0xBE);
// const QUOTE: KeybdKey = OtherKey(0xDE);
const SEMI: KeybdKey = OtherKey(0xBA);

const ALT: KeybdKey = OtherKey(0x12);
// }

const SHIFT: EitherKey = EitherKey(&LShiftKey, &RShiftKey);
const CONTROL: EitherKey = EitherKey(&LControlKey, &RControlKey);

// Accents:
// ä œ ë ü ï ö
// à ñ è ù ì ò
// á ß é ú í ó
// â ç ê   î ô

const ACCENT_ARRAY: [(KeybdKey, char, char); 27] = [
    (Numrow1Key, 'ä', 'Ä'), // 1
    (QKey, 'à', 'À'),
    (AKey, 'á', 'Á'),
    (ZKey, 'â', 'Â'),
    (WKey, 'ñ', 'Ñ'), // 2
    (SKey, 'ß', 'ß'),
    (XKey, 'ç', 'Ç'),
    (Numrow3Key, 'ë', 'Ë'), // 3
    (DKey, 'è', 'È'),
    (EKey, 'é', 'É'),
    (CKey, 'ê', 'Ê'),
    (TKey, '€', '€'), // 4
    (HKey, '―', '―'), // 6
    (KKey, '–', '–'),
    (YKey, 'ý', 'Ý'),
    (Numrow7Key, 'ü', 'Ü'), // 7
    (PKey, 'ù', 'Ù'),
    (NKey, 'ú', 'Ú'),
    (MKey, 'û', 'Û'),
    (Numrow8Key, 'ï', 'Ï'), // 8
    (UKey, 'ì', 'Ì'),
    (IKey, 'í', 'Í'),
    (COMMA, 'î', 'Î'),
    (Numrow9Key, 'ö', 'Ö'), // 9
    (LKey, 'ò', 'Ò'),
    (OKey, 'ó', 'Ó'),
    (PERIOD, 'ô', 'Ô'),
];

fn numlock_set(state: bool) {
    if NumLockKey.is_toggled() != state {
        NumLockKey.send();
    }
    assert_eq!(NumLockKey.is_toggled(), state);
}

fn main() {
    let mut dev = false;
    let qquit = true;

    for arg in args() {
        if arg == "--dev" {
            dev = true;
        }
    }

    for &(key, low_char, up_char) in ACCENT_ARRAY.iter() {
        let report = |case: &'static str| move || panic!("{:?} {:?} {:?}", case, low_char, up_char);

        let alt_code_low = AltCode::from_accent1(low_char)
            .unwrap_or_else(|| AltCode::from_accent0(low_char).unwrap_or_else(report("low")));
        let alt_code_up = AltCode::from_accent1(up_char)
            .unwrap_or_else(|| AltCode::from_accent0(up_char).unwrap_or_else(report("up")));

        // let alt_code_low = AltCode::Char(low_char);
        // let alt_code_up = AltCode::Char(up_char);

        let handle_quote = move || {
            if SEMI.is_pressed() {
                key.release();
                BackspaceKey.send();
                BackspaceKey.send();
                if SHIFT.is_pressed() {
                    numlock_set(true);
                    alt_code_up.send();
                } else {
                    numlock_set(true);
                    alt_code_low.send();
                }
            }
        };

        if dev && key == SKey {
            key.bind(move || {
                handle_quote();
                if CONTROL.is_pressed() {
                    exit(0);
                }
            });
        } else if qquit && key == QKey {
            key.bind(move || {
                handle_quote();
                if CONTROL.is_pressed() && SHIFT.is_pressed() && ALT.is_pressed() {
                    exit(0);
                }
            });
        } else {
            key.bind(handle_quote)
        }
    }
    handle_input_events();
}
