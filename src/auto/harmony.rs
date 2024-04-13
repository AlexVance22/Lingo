

pub fn is_vowel(val: char) -> bool {
    (val == 'a') || (val == 'e') || (val == 'i') || (val == 'o') || (val == 'u') || (val == 'ı') || (val == 'ö') || (val == 'ü')
}

pub fn get_last_vowel(val: &str) -> Option<char> {
    for c in val.chars().rev() {
        if is_vowel(c) {
            return Some(c)
        }
    }
    None
}


pub enum Harmony { A, E, O, Oe }

impl Harmony {
    pub fn from_root(val: &str) -> Self {
        match get_last_vowel(val).unwrap() {
            'a' => Harmony::A,
            'e' => Harmony::E,
            'o' => Harmony::O,
            'ö' => Harmony::Oe,
            'ı' => Harmony::A,
            'i' => Harmony::E,
            'u' => Harmony::O,
            'ü' => Harmony::Oe,
            _ => unreachable!()
        }
    }
}

impl Harmony {
    pub fn way2(&self)    -> char { match self { Self::A => 'ı', Self::E => 'i', Self::O => 'ı', Self::Oe => 'i' } }
    pub fn way2inv(&self) -> char { match self { Self::A => 'a', Self::E => 'e', Self::O => 'a', Self::Oe => 'e' } }
    pub fn way4(&self)    -> char { match self { Self::A => 'ı', Self::E => 'i', Self::O => 'u', Self::Oe => 'ü' } }
    pub fn way4inv(&self) -> char { match self { Self::A => 'a', Self::E => 'e', Self::O => 'o', Self::Oe => 'ö' } }
}

