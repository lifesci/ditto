use std::collections::HashMap;

pub struct Cactus {
    active: Node,
}

struct Scope {
    parent: Node,
    vars: HashMap<String, i32>,
}

impl Scope {
    fn lookup(&self, name: String) -> Option<i32> {
        match self.vars.get(&name) {
            None => match self.parent.as_ref() {
                None => None,
                Some(n) => n.lookup(name),
            },
            Some(x) => Some(*x),
        }
    }

    fn add(&mut self, name: String, val: i32) {
        self.vars.insert(name, val);
    }
}

type Node = Option<Box<Scope>>;

impl Cactus {
    pub fn new() -> Self {
        Self { active: None }
    }

    pub fn push(&mut self) {
        let new_scope = Scope {
            parent: self.active.take(),
            vars: HashMap::new(),
        };
        self.active = Some(Box::new(new_scope));
    }

    pub fn add(&mut self, name: String, val: i32) {
        match self.active.as_mut() {
            None => (),
            Some(n) => n.add(name, val),
        }
    }

    pub fn pop(&mut self) {
        self.active = match self.active.take() {
            None => None,
            Some(x) => x.parent,
        }
    }

    pub fn lookup(&self, name: String) -> Option<i32> { 
        match self.active.as_ref() {
            None => None,
            Some(n) => n.lookup(name),
        }
    }
}
