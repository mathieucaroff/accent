use inputbot::KeybdKey;

pub enum AltCode<'a> {
    Digits(&'a str),
    #[allow(dead_code)]
    Char(char),
}

pub struct EitherKey<'a, 'b>(pub &'a KeybdKey, pub &'b KeybdKey);
