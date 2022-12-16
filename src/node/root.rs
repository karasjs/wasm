use std::ptr;
use wasm_bindgen::prelude::*;
use crate::node::Node;

pub const CANVAS: u8 = 0;
pub const WEBGL: u8 = 2;

#[wasm_bindgen]
pub struct Root {
  pub mode: u8,
  pub width: f32,
  pub height: f32,
  pub font_size: f32,
  nodes: Vec<*mut Node>, // 对应js的Root下structs先序遍历的节点列表
}

#[wasm_bindgen]
impl Root {
  pub fn new() -> Root {
    Root {
      mode: 0,
      width: 0.0,
      height: 0.0,
      font_size: 0.0,
      nodes: Vec::new(),
    }
  }

  pub fn add(&mut self, node: *mut Node) -> () {
    self.nodes.push(node);
  }

  pub fn remove(&mut self, i: usize) -> () {
    self.nodes.remove(i);
  }

  pub fn clear(&mut self) -> () {
    self.nodes.clear();
  }

  pub fn size(&self) -> usize {
    self.nodes.len()
  }

  pub fn resize(&mut self, width: f32, height: f32) -> () {
    self.width = width;
    self.height = height;
  }

  pub fn set_font_size(&mut self, font_size: f32) -> () {
    self.font_size = font_size;
  }

  // 每帧raf优先存调用，传入运行时间，后续节点动画来计算transition
  pub fn on_frame(&mut self, diff: f32) -> () {
    let mut count = 0;
    let len = self.nodes.len();
    while count < len {
      let mut node = unsafe { &mut *self.nodes[count] };
      node.on_frame(diff);
      count += 1;
    }
  }
}
