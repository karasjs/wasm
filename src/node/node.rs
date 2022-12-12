use std::ptr;
use wasm_bindgen::prelude::*;
use crate::node::Root;
use crate::style::style_unit;
use crate::animation::Animation;

#[wasm_bindgen]
pub struct Node {
  root: *mut Root,
  x: f32,
  y: f32,
  offset_width: f32,
  offset_height: f32,
  lv: usize,
  refresh_level: usize,
  current_style: [f32; 13],
  current_unit: [u8; 5],
  computed_style: [f32; 13],
  transform: [f32; 16],
  matrix: [f32; 16],
  matrix_event: [f32; 16],
  animations: Vec<Animation>,
}

#[wasm_bindgen]
impl Node {
  pub fn new(x: f32, y: f32, offset_width: f32, offset_height: f32, lv: usize,
             cs1: f32, cs2: f32, cs3: f32, cs4: f32, cs5: f32, cs6: f32,
             cs7: f32, cs8: f32, cs9: f32, cs10: f32, cs11: f32, cs12: f32,
             cu1:u8, cu2:u8, cu3:u8, cu11: u8, cu12:u8, opacity: f32) -> Node {
    let tx = if cu1 == style_unit::PERCENT {
      cs1 * 100.0 * offset_width
    }
    else {
      cs1
    };
    let ty = if cu2 == style_unit::PERCENT {
      cs2 * 100.0 * offset_height
    }
    else {
      cs2
    };
    let tz = if cu3 == style_unit::PERCENT {
      cs3 * 100.0 * offset_width
    }
    else {
      cs3
    };
    let tfo_x = if cu11 == style_unit::PERCENT {
      cs11 * 100.0 * offset_width
    }
    else {
      cs11
    };
    let tfo_y = if cu12 == style_unit::PERCENT {
      cs12 * 100.0 * offset_height
    }
    else {
      cs12
    };
    Node {
      root: ptr::null_mut(),
      x,
      y,
      offset_width,
      offset_height,
      lv,
      refresh_level: 0,
      current_style: [
        cs1,
        cs2,
        cs3,
        cs4,
        cs5,
        cs6,
        cs7,
        cs8,
        cs9,
        cs10,
        cs11,
        cs12,
        opacity,
      ],
      current_unit: [
        cu1,
        cu2,
        cu3,
        cu11,
        cu12,
      ],
      computed_style: [
        tx,
        ty,
        tz,
        cs4,
        cs5,
        cs6,
        cs7,
        cs8,
        cs9,
        cs10,
        tfo_x,
        tfo_y,
        opacity,
      ],
      transform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix_event: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      animations: Vec::new(),
    }
  }

  pub fn set_root(&mut self, root: *mut Root) -> () {
    self.root = root;
  }
}
