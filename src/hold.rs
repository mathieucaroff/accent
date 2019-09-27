use inputbot::KeybdKey;

pub trait Hold {
    fn hold<F>(&self, f: &mut F)
    where
        F: FnMut();
}

impl Hold for KeybdKey {
    fn hold<F>(&self, f: &mut F)
    where
        F: FnMut(),
    {
        self.press();
        f();
        self.release();
    }
}
