use std::ptr;
use std::cmp;
use wasm_bindgen::prelude::*;
use crate::node::Root;
use crate::style::style_unit;
use crate::animation::Animation;

fn cal_unit(v: f32, u: u8, percent: f32, root: &Root) -> f32 {
  if u == style_unit::PERCENT {
    return v * 100.0 * percent
  } else if u == style_unit::REM {
    return v * root.font_size
  } else if u == style_unit::VW {
    return v * 0.01 * root.width as f32
  } else if u == style_unit::VH {
    return v * 0.01 * root.height as f32
  } else if u == style_unit::VMAX {
    return v * 0.01 * cmp::max(root.width, root.height) as f32
  } else if u == style_unit::VMIN {
    return v * 0.01 * cmp::min(root.width, root.height) as f32
  } else {
    return v
  }
}

#[wasm_bindgen]
pub struct Node {
  root: *mut Root,
  x: f32,
  y: f32,
  offset_width: f32,
  offset_height: f32,
  lv: usize,
  refresh_level: usize,
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
  pub fn new(root: *mut Root, x: f32, y: f32, offset_width: f32, offset_height: f32, lv: usize,
             cs0: f32, cs1: f32, cs2: f32, cs3: f32, cs4: f32, cs5: f32, cs6: f32,
             cs7: f32, cs8: f32, cs9: f32, cs10: f32, cs11: f32, cs12: f32,
             cu0: u8, cu1: u8, cu2: u8, cu11: u8, cu12: u8, opacity: f32) -> Node {
    let root = unsafe { &mut *root };
    let tx = cal_unit(cs1, cu0, offset_width, root);
    let ty = cal_unit(cs2, cu1, offset_height, root);
    let tz = cal_unit(cs3, cu2, offset_width, root);
    let tfo_x = cal_unit(cs11, cu11, offset_width, root);
    let tfo_y = cal_unit(cs12, cu12, offset_height, root);
    Node {
      root,
      x,
      y,
      offset_width,
      offset_height,
      lv,
      refresh_level: 0,
      current_style: [
        cs0,
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
        cu0,
        cu1,
        cu2,
        style_unit::DEG,
        style_unit::DEG,
        style_unit::DEG,
        style_unit::NUMBER,
        style_unit::NUMBER,
        style_unit::NUMBER,
        style_unit::DEG,
        style_unit::DEG,
        cu11,
        cu12,
        style_unit::NUMBER,
      ],
      computed_style: [
        tx,
        ty,
        tz,
        cs3,
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

  pub fn on_frame(&mut self, delta: usize) -> () {
    let mut count = 0;
    let len = self.animations.len();
    while count < len {
      let mut animation = unsafe { &mut *self.animations[count] };
      animation.on_frame(delta);
      count += 1;
    }
  }
}
