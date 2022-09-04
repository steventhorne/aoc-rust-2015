use std::{collections::HashMap, rc::Rc};

pub struct WireConfiguration {
    pub label: String,
    pub op: WireOp,
    pub value: Option<u16>,
}

pub enum Input {
    Value(u16),
    Wire(String),
}

impl Input {
    pub fn from(value: &str) -> Input {
        if let Ok(v) = value.parse::<u16>() {
            Input::Value(v)
        }
        else {
            Input::Wire(value.to_string())
        }
    }
}

pub enum WireOp {
    Value(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
    Not(Input),
}

#[derive(Default)]
pub struct Wires {
    pub wires: HashMap<String, Rc<WireConfiguration>>,
    pub wire_cache: HashMap<String, u16>,
}

impl Wires {
    pub fn add_wire(&mut self, config: WireConfiguration) {
        self.wires.insert(config.label.clone(), Rc::new(config));
    }

    pub fn get_and_cache_value(&mut self, label: &str) -> u16 {
        if let Some(v) = self.wire_cache.get(label) {
            return *v;
        }
        let wire = Rc::clone(self.wires.get(label).unwrap());
        let value = match &wire.op {
            WireOp::Value(i) => match i {
                Input::Wire(v) => self.get_and_cache_value(v),
                Input::Value(v) => *v,
            },
            WireOp::And(li, ri) => {
                let l = match li {
                    Input::Wire(l) => self.get_and_cache_value(l),
                    Input::Value(l) => *l,
                };
                let r = match ri {
                    Input::Wire(r) => self.get_and_cache_value(r),
                    Input::Value(r) => *r,
                };
                l & r
            }
            WireOp::Or(li, ri) => {
                let l = match li {
                    Input::Wire(l) => self.get_and_cache_value(l),
                    Input::Value(l) => *l,
                };
                let r = match ri {
                    Input::Wire(r) => self.get_and_cache_value(r),
                    Input::Value(r) => *r,
                };
                l | r
            }
            WireOp::LShift(li, n) => {
                let l = match li {
                    Input::Wire(l) => self.get_and_cache_value(l),
                    Input::Value(l) => *l,
                };
                l << n
            }
            WireOp::RShift(li, n) => {
                let l = match li {
                    Input::Wire(l) => self.get_and_cache_value(l),
                    Input::Value(l) => *l,
                };
                l >> n
            }
            WireOp::Not(i) => match i {
                Input::Wire(v) => !self.get_and_cache_value(v),
                Input::Value(v) => !(*v),
            }
        };
        self.cache_value(label, value);
        value
    }

    pub fn cache_value(&mut self, label: &str, value: u16) {
        self.wire_cache.insert(label.to_string(), value);
    }

    pub fn get_sorted(&self) -> Vec<(&String, &Rc<WireConfiguration>)> {
        let mut vec = self.wires.iter()
            .collect::<Vec<(&String, &Rc<WireConfiguration>)>>();
        vec.sort_by(|a, b| a.0.cmp(b.0));
        vec
    }
}
