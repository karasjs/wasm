use std::ptr;
use wasm_bindgen::prelude::*;
use crate::node::Node;

#[wasm_bindgen]
pub struct Root {
  list: Vec<*mut Node>,
}

#[wasm_bindgen]
impl Root {
  pub fn new() -> Root {
    Root {
      list: Vec::new(),
    }
  }

  pub fn add(&mut self, node: *mut Node) -> () {
    self.list.push(node);
  }

  pub fn size(&self) -> usize {
    self.list.len()
  }
}
