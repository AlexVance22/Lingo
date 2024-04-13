#![allow(dead_code)]
// İI iı ş ğ
mod harmony;
use harmony::*;


pub struct VerbRoot {
    root: String,
    harmony: Harmony,
    ending_vowel: bool,
    ending_soft: bool,
    dt_mutate: Option<String>,
}


impl VerbRoot {
    pub fn from_inf(val: &str) -> Self {
        let mut root = val.to_string();
        root.pop();
        root.pop();
        root.pop();
        let last = root.chars().last().unwrap();
        let harmony = Harmony::from_root(&root);
        let ending_vowel = is_vowel(last);
        let ending_soft = last == 'l' || last == 'r';
        let dt_mutate = if last == 't' {
            let mut temp = root.clone();
            temp.pop();
            temp.push('d');
            Some(temp)
        } else {
            None
        };
        Self{ root, harmony, ending_vowel, ending_soft, dt_mutate }
    }
}

impl VerbRoot {
    fn soft_mutate(&self) -> String {
        if let Some(root) = &self.dt_mutate { root.clone() } else { self.root.clone() }
    }

    fn aorist_root(&self) -> String {
        if self.root.len() < 3 && self.ending_vowel {
            format!("{}r", self.root)
        } else {
            let mut temp = self.soft_mutate();
            if self.ending_vowel { temp.pop(); }
            format!("{}{}r", temp, self.harmony.way4inv())
        }
    }
    fn continuous_root(&self) -> String {
        let mut temp = self.soft_mutate();
        if self.ending_vowel { temp.pop(); }
        format!("{}{}yor", temp, self.harmony.way4())
    }
    fn definite_past_root(&self) -> String {
        format!("{}{}", self.root, if self.ending_vowel || self.ending_soft { 'd' } else { 't' })
    }
    fn reported_past_root(&self) -> String {
        format!("{}m{}şt", self.root, self.harmony.way4())
    }
    fn future_root(&self) -> String {
        format!("{}{}{v}c{v}", self.soft_mutate(), if self.ending_vowel { "y" } else { "" }, v = self.harmony.way2inv())
    }

    fn neg_aorist_root(&self) -> String {
        format!("{}m{}", self.root, self.harmony.way2inv())
    }
    fn neg_continuous_root(&self) -> String {
        format!("{}m{}yor", self.root, self.harmony.way4())
    }
    fn neg_definite_past_root(&self) -> String {
        format!("{}m{}d", self.root, self.harmony.way2inv())
    }
    fn neg_reported_past_root(&self) -> String {
        format!("{}m{}m{}şt", self.root, self.harmony.way2inv(), self.harmony.way2())
    }
    fn neg_future_root(&self) -> String {
        format!("{}m{v}y{v}c{v}", self.root, v = self.harmony.way2inv())
    }
}

impl VerbRoot {
    pub fn imperative(&self) -> String {
        self.root.clone()
    }
    pub fn neg_imperative(&self) -> String {
        format!("{}me", self.root.clone())
    }

    pub fn aorist(&self) -> Vec<String> {
        let root = self.aorist_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}{v}m"),
            format!("{root}s{v}n"),
            format!("{root}"),
            format!("{root}{v}z"),
            format!("{root}s{v}n{v}z"),
            format!("{root}l{}r", h.way2inv()),
        ]
    }
    pub fn continuous(&self) -> Vec<String> {
        let root = self.continuous_root();
        vec![
            format!("{root}um"),
            format!("{root}sun"),
            format!("{root}"),
            format!("{root}uz"),
            format!("{root}sunuz"),
            format!("{root}lar"),
        ]
    }
    pub fn definite_past(&self) -> Vec<String> {
        let root = self.definite_past_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}{v}m"),
            format!("{root}{v}n"),
            format!("{root}{v}"),
            format!("{root}{v}k"),
            format!("{root}{v}n{v}z"),
            format!("{root}{v}l{}r", h.way2inv())
        ]
    }
    pub fn reported_past(&self) -> Vec<String> {
        let root = self.reported_past_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}{v}m"),
            format!("{root}{v}n"),
            format!("{root}{v}"),
            format!("{root}{v}z"),
            format!("{root}{v}n{v}z"),
            format!("{root}{v}l{}r", h.way2inv())
        ]
    }
    pub fn future(&self) -> Vec<String> {
        let root = self.future_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}ğ{v}m"),
            format!("{root}ks{v}n"),
            format!("{root}k"),
            format!("{root}ğ{v}z"),
            format!("{root}ks{v}n{v}z"),
            format!("{root}kl{}r", h.way2inv())
        ]
    }

    pub fn neg_aorist(&self) -> Vec<String> {
        let root = self.neg_aorist_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}m"),
            format!("{root}zs{v}n"),
            format!("{root}z"),
            format!("{root}y{v}z"),
            format!("{root}zs{v}n{v}z"),
            format!("{root}zl{}r", h.way2inv()),
        ]
    }
    pub fn neg_continuous(&self) -> Vec<String> {
        let root = self.neg_continuous_root();
        vec![
            format!("{root}um"),
            format!("{root}sun"),
            format!("{root}"),
            format!("{root}uz"),
            format!("{root}sunuz"),
            format!("{root}lar"),
        ]
    }
    pub fn neg_definite_past(&self) -> Vec<String> {
        let root = self.neg_definite_past_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}{v}m"),
            format!("{root}{v}n"),
            format!("{root}{v}"),
            format!("{root}{v}k"),
            format!("{root}{v}n{v}z"),
            format!("{root}{v}l{}r", h.way2inv())
        ]
    }
    pub fn neg_reported_past(&self) -> Vec<String> {
        let root = self.neg_reported_past_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}{v}m"),
            format!("{root}{v}n"),
            format!("{root}{v}"),
            format!("{root}{v}z"),
            format!("{root}{v}n{v}z"),
            format!("{root}{v}l{}r", h.way2inv())
        ]
    }
    pub fn neg_future(&self) -> Vec<String> {
        let root = self.neg_future_root();
        let h = Harmony::from_root(&root);
        let v = h.way4();
        vec![
            format!("{root}ğ{v}m"),
            format!("{root}ks{v}n"),
            format!("{root}k"),
            format!("{root}ğ{v}z"),
            format!("{root}ks{v}n{v}z"),
            format!("{root}kl{}r", h.way2inv())
        ]
    }
}

