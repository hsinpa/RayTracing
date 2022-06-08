use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::material::material_interface::MaterialTrait;

pub struct Materials {
    mat_list: HashMap<u32, Rc<RefCell<dyn MaterialTrait>>>
}

impl Materials {
    pub fn new() -> Self {
        Self {
            mat_list: HashMap::new()
        }
    }

    pub fn push(&mut self, id: u32, mat: Rc<RefCell<dyn MaterialTrait>>) {
        self.mat_list.insert(id, mat);
    }

    pub fn get(&self, id: u32) -> Option<&Rc<RefCell<dyn MaterialTrait>>> {
        return   self.mat_list.get(&id);
    }
}