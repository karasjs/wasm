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
  list: Vec<*mut Node>, // 对应js的Root下structs先序遍历的节点列表
}

#[wasm_bindgen]
impl Root {
  pub fn new(mode: u8, width: f32, height: f32, font_size: f32) -> Root {
    Root {
      mode,
      width,
      height,
      font_size,
      list: Vec::new(),
    }
  }

  pub fn add(&mut self, node: *mut Node) -> () {
    self.list.push(node);
  }

  pub fn size(&self) -> usize {
    self.list.len()
  }

  // 每帧raf优先存调用，传入运行时间，后续节点动画来计算transition
  pub fn on_frame(&mut self, delta: usize) -> () {
    let mut count = 0;
    let len = self.list.len();
    while count < len {
      let mut node = unsafe { &mut *self.list[count] };
      node.on_frame(delta);
      count += 1;
    }
  }
}
