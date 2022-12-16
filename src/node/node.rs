use std::ptr;
use wasm_bindgen::prelude::*;
use crate::node::Root;
use crate::style::style_unit;
use crate::refresh::refresh_level;
use crate::animation::Animation;

#[wasm_bindgen]
pub struct Node {
  root: *mut Root,
  pub is_text: bool,
  pub x: f32,
  pub y: f32,
  pub offset_width: f32,
  pub offset_height: f32,
  pub lv: usize,
  pub refresh_level: usize,
  current_style: [f32; 14],
  current_unit: [u8; 14],
  computed_style: [f32; 14],
  transform: [f32; 16],
  matrix: [f32; 16],
  matrix_event: [f32; 16],
  animations: Vec<*mut Animation>,
}

#[wasm_bindgen]
impl Node {
  pub fn new(is_text: bool) -> Node {
    Node {
      root: ptr::null_mut(),
      is_text,
      x: 0.0,
      y: 0.0,
      offset_width: 0.0,
      offset_height: 0.0,
      lv: refresh_level::REFLOW,
      refresh_level: 0,
      current_style: [0.0; 14],
      current_unit: [0; 14],
      computed_style: [0.0; 14],
      transform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix_event: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      animations: Vec::new(),
    }
  }

  pub fn set_root(&mut self, root: *mut Root) -> () {
    self.root = root;
  }

  pub fn add(&mut self, animation: *mut Animation) -> () {
    self.animations.push(animation);
  }

  pub fn remove(&mut self, animation: *mut Animation) -> () {
    self.animations.retain(|&x| x != animation);
  }

  pub fn clear(&mut self) -> () {
    self.animations.clear();
  }

  pub fn set_style(&mut self, x: f32, y: f32, offset_width: f32, offset_height: f32,
                   cs0: f32, cs1: f32, cs2: f32, cs3: f32, cs4: f32, cs5: f32,
                   cs6: f32, cs7: f32, cs8: f32, cs9: f32, cs10: f32, cs11: f32, cs12: f32,
                   cs13: f32, cu0: u8, cu1: u8, cu2: u8, cu12: u8, cu13: u8) -> () {
    self.x = x;
    self.y = y;
    self.offset_width = offset_width;
    self.offset_height = offset_height;
    self.current_style[0] = cs0;
    self.current_style[1] = cs1;
    self.current_style[2] = cs2;
    self.current_style[3] = cs3;
    self.current_style[4] = cs4;
    self.current_style[5] = cs5;
    self.current_style[6] = cs6;
    self.current_style[7] = cs7;
    self.current_style[8] = cs8;
    self.current_style[9] = cs9;
    self.current_style[10] = cs10;
    self.current_style[11] = cs11;
    self.current_style[12] = cs12;
    self.current_style[13] = cs13;
    self.current_unit[0] = cu0;
    self.current_unit[1] = cu1;
    self.current_unit[2] = cu2;
    self.current_unit[12] = cu12;
    self.current_unit[13] = cu13;
  }

  pub fn on_frame(&mut self, diff: f32) -> () {
    let mut count = 0;
    let len = self.animations.len();
    while count < len {
      let mut animation = unsafe { &mut *self.animations[count] };
      animation.on_frame(diff);
      count += 1;
    }
  }
}
