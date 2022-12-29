use std::ptr;
use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use crate::node::Root;
use crate::style::style_unit;
use crate::style::style_key::*;
use crate::refresh::refresh_level;
use crate::animation::Animation;
use crate::math::*;

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
  current_style: [f32; 18],
  current_unit: [usize; 18],
  computed_style: [f32; 18],
  transform: [f32; 16],
  matrix: [f32; 16],
  matrix_event: [f32; 16],
  pub opacity: f32,
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
      lv: refresh_level::NONE,
      refresh_level: 0,
      current_style: [0.0; 18],
      current_unit: [0; 18],
      computed_style: [0.0; 18],
      transform: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      matrix_event: [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0],
      opacity: 1.0,
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
                   cs13: f32, cs14: f32, cs15: f32, cs16: f32, cs17: f32,
                   cu0: usize, cu1: usize, cu2: usize, cu16: usize, cu17: usize) -> () {
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
    self.current_style[14] = cs14;
    self.current_style[15] = cs15;
    self.current_style[16] = cs16;
    self.current_style[17] = cs17;
    self.current_unit[0] = cu0;
    self.current_unit[1] = cu1;
    self.current_unit[2] = cu2;
    self.current_unit[16] = cu16;
    self.current_unit[17] = cu17;
    self.computed_style[0] = self.cal_size(cs0, cu0, offset_width);
    self.computed_style[1] = self.cal_size(cs1, cu1, offset_height);
    self.computed_style[2] = self.cal_size(cs2, cu2, offset_width);
    self.computed_style[3] = cs3;
    self.computed_style[4] = cs4;
    self.computed_style[5] = cs5;
    self.computed_style[6] = cs6;
    self.computed_style[7] = cs7;
    self.computed_style[8] = cs8;
    self.computed_style[9] = cs9;
    self.computed_style[10] = cs10;
    self.computed_style[11] = cs11;
    self.computed_style[12] = cs12;
    self.computed_style[13] = cs13;
    self.computed_style[14] = cs14;
    self.computed_style[15] = cs15;
    self.computed_style[16] = self.cal_size(cs16, cu16, offset_width);
    self.computed_style[17] = self.cal_size(cs17, cu17, offset_height);
    self.cal_matrix(refresh_level::TRANSFORM);
  }

  pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32,
                       e: f32, f: f32, g: f32, h: f32,
                       i: f32, j: f32, k: f32, l: f32,
                       m: f32, n: f32, o: f32, p: f32) -> () {
    self.transform[0] = a;
    self.transform[1] = b;
    self.transform[2] = c;
    self.transform[3] = d;
    self.transform[4] = e;
    self.transform[5] = f;
    self.transform[6] = g;
    self.transform[7] = h;
    self.transform[8] = i;
    self.transform[9] = j;
    self.transform[10] = k;
    self.transform[11] = l;
    self.transform[12] = m;
    self.transform[13] = n;
    self.transform[14] = o;
    self.transform[15] = p;
  }

  pub fn set_matrix(&mut self, a: f32, b: f32, c: f32, d: f32,
                       e: f32, f: f32, g: f32, h: f32,
                       i: f32, j: f32, k: f32, l: f32,
                       m: f32, n: f32, o: f32, p: f32) -> () {
    self.matrix[0] = a;
    self.matrix[1] = b;
    self.matrix[2] = c;
    self.matrix[3] = d;
    self.matrix[4] = e;
    self.matrix[5] = f;
    self.matrix[6] = g;
    self.matrix[7] = h;
    self.matrix[8] = i;
    self.matrix[9] = j;
    self.matrix[10] = k;
    self.matrix[11] = l;
    self.matrix[12] = m;
    self.matrix[13] = n;
    self.matrix[14] = o;
    self.matrix[15] = p;
  }

  pub fn m_ptr(&self) -> *const f32 {
    self.matrix.as_ptr()
  }

  pub fn me_ptr(&self) -> *const f32 {
    self.matrix_event.as_ptr()
  }

  pub fn get_op(&self) -> f32 {
    self.computed_style[OPACITY]
  }

  pub fn get_rl(&self) -> usize {
    self.refresh_level
  }

  pub fn on_frame(&mut self, diff: f32) -> () {
    let mut count = 0;
    let len = self.animations.len();
    while count < len {
      let mut animation = unsafe { &mut *self.animations[count] };
      let len = animation.on_frame(diff);
      if len > 0 {
        let ts = animation.get_transition();
        let mut lv = 0_usize;
        for item in ts.iter() {
          self.current_style[item.k] = item.v;
          self.current_unit[item.k] = item.u;
          if item.k == TRANSLATE_X {
            lv |= refresh_level::TRANSLATE_X;
          } else if item.k == TRANSLATE_Y {
            lv |= refresh_level::TRANSLATE_Y;
          } else if item.k == TRANSLATE_Z {
            lv |= refresh_level::TRANSLATE_Z;
          } else if item.k == ROTATE_X {
            lv |= refresh_level::TRANSFORM;
          } else if item.k == ROTATE_Y {
            lv |= refresh_level::TRANSFORM;
          } else if item.k == ROTATE_Z {
            lv |= refresh_level::ROTATE_Z;
          } else if item.k == SCALE_X {
            lv |= refresh_level::SCALE_X;
          } else if item.k == SCALE_Y {
            lv |= refresh_level::SCALE_Y;
          } else if item.k == SCALE_Z {
            lv |= refresh_level::SCALE_Z;
          } else if item.k == SKEW_X {
            lv |= refresh_level::TRANSFORM;
          } else if item.k == SKEW_Y {
            lv |= refresh_level::TRANSFORM;
          } else if item.k == OPACITY {
            lv |= refresh_level::OPACITY;
          } else if item.k == TFO_X {
            lv |= refresh_level::TRANSFORM;
          } else if item.k == TFO_Y {
            lv |= refresh_level::TRANSFORM;
          }
        }
        if lv & refresh_level::TRANSFORM_ALL > 0 {
          self.cal_matrix(lv);
        }
        if lv & refresh_level::OPACITY > 0 {
          self.computed_style[OPACITY]
            = self.current_style[OPACITY];
        }
        self.refresh_level |= lv;
      }
      count += 1;
    }
  }

  fn cal_matrix(&mut self, rl: usize) -> () {
    let mut optimize = true;
    if rl & refresh_level::TRANSFORM > 0 {
      optimize = false;
    } else if rl & refresh_level::SCALE_X > 0 && self.computed_style[SCALE_X] == 0.0 {
      optimize = false;
    } else if rl & refresh_level::SCALE_Y > 0 && self.computed_style[SCALE_Y] == 0.0 {
      optimize = false;
    } else if rl & refresh_level::SCALE_Z > 0 && self.computed_style[SCALE_Z] == 0.0 {
      optimize = false;
    } else if rl & refresh_level::ROTATE_Z > 0
      && (self.computed_style[ROTATE_X] != 0.0
      || self.computed_style[ROTATE_Y] != 0.0
      || self.computed_style[SKEW_X] != 0.0
      || self.computed_style[SKEW_Y] != 0.0) {
      optimize = false;
    }
    if optimize {
      if rl & refresh_level::TRANSLATE_X > 0 {
        let v = self.cal_size(self.current_style[TRANSLATE_X], self.current_unit[TRANSLATE_X], self.offset_width);
        let x = v - self.computed_style[TRANSLATE_X];
        self.computed_style[TRANSLATE_X] = v;
        self.transform[12] += x;
        self.matrix[12] += x;
      }
      if rl & refresh_level::TRANSLATE_Y > 0 {
        let v = self.cal_size(self.current_style[TRANSLATE_Y], self.current_unit[TRANSLATE_Y], self.offset_height);
        let y = v - self.computed_style[TRANSLATE_Y];
        self.computed_style[TRANSLATE_Y] = v;
        self.transform[13] += y;
        self.matrix[13] += y;
      }
      if rl & refresh_level::TRANSLATE_Z > 0 {
        let v = self.cal_size(self.current_style[TRANSLATE_Z], self.current_unit[TRANSLATE_Z], self.offset_width);
        let z = v - self.computed_style[TRANSLATE_Z];
        self.computed_style[TRANSLATE_Z] = v;
        self.transform[14] += z;
        self.matrix[14] += z;
      }
      if rl & refresh_level::ROTATE_Z > 0 {
        let v = self.current_style[ROTATE_Z];
        self.computed_style[ROTATE_Z] = v;
        let r = v * PI / 180.0;
        let sin = r.sin();
        let cos = r.cos();
        let x = self.computed_style[SCALE_X];
        let y = self.computed_style[SCALE_Y];
        let cx = cos * x;
        self.transform[0] = cx;
        self.matrix[0] = cx;
        let sx = sin * x;
        self.transform[1] = cx;
        self.matrix[1] = cx;
        let sy = -sin * y;
        self.transform[4] = cx;
        self.matrix[4] = cx;
        let cy = cos * y;
        self.transform[5] = cx;
        self.matrix[5] = cx;
        let ox = self.computed_style[TFO_X] + self.x;
        let oy = self.computed_style[TFO_Y] + self.y;
        self.matrix[12] = self.transform[12] + ox - cx * ox - oy * sy;
        self.matrix[13] = self.transform[13] + oy - sx * ox - oy * cy;
      }
      if rl & refresh_level::SCALE > 0 {
        if rl & refresh_level::SCALE_X > 0 {
          if self.computed_style[SCALE_X] == 0.0 {
            return self.cal_matrix(refresh_level::TRANSFORM)
          }
          let v = self.current_style[SCALE_X];
          let x = v / self.computed_style[SCALE_X];
          self.computed_style[SCALE_X] = v;
          self.transform[0] *= x;
          self.transform[1] *= x;
          self.transform[2] *= x;
          self.matrix[0] *= x;
          self.matrix[1] *= x;
          self.matrix[2] *= x;
        }
        if rl & refresh_level::SCALE_Y > 0 {
          if self.computed_style[SCALE_Y] == 0.0 {
            return self.cal_matrix(refresh_level::TRANSFORM)
          }
          let v = self.current_style[SCALE_Y];
          let y = v / self.computed_style[SCALE_Y];
          self.computed_style[SCALE_Y] = v;
          self.transform[4] *= y;
          self.transform[5] *= y;
          self.transform[6] *= y;
          self.matrix[4] *= y;
          self.matrix[5] *= y;
          self.matrix[6] *= y;
        }
        if rl & refresh_level::SCALE_Z > 0 {
          if self.computed_style[SCALE_Z] == 0.0 {
            return self.cal_matrix(refresh_level::TRANSFORM)
          }
          let v = self.current_style[SCALE_Z];
          let z = v / self.computed_style[SCALE_Z];
          self.computed_style[SCALE_Z] = v;
          self.transform[8] *= z;
          self.transform[9] *= z;
          self.transform[10] *= z;
          self.matrix[8] *= z;
          self.matrix[9] *= z;
          self.matrix[10] *= z;
        }
        let ox = self.computed_style[TFO_X] + self.x;
        let oy = self.computed_style[TFO_Y] + self.y;
        self.matrix[12] = self.transform[12] + ox - self.transform[0] * ox - self.transform[4] * oy;
        self.matrix[13] = self.transform[13] + oy - self.transform[1] * ox - self.transform[5] * oy;
        self.matrix[14] = self.transform[14] - self.transform[2] * ox - self.transform[6] * oy;
      }
    } else {
      self.set_transform(1.0, 0.0, 0.0, 0.0,
                      0.0, 1.0, 0.0, 0.0,
                      0.0, 0.0, 1.0, 0.0,
                      0.0, 0.0, 0.0, 1.0);
      let v = self.cal_size(self.current_style[TRANSLATE_X], self.current_unit[TRANSLATE_X], self.offset_width);
      self.computed_style[TRANSLATE_X] = v;
      self.transform[12] = v;
      let v = self.cal_size(self.current_style[TRANSLATE_Y], self.current_unit[TRANSLATE_Y], self.offset_height);
      self.computed_style[TRANSLATE_Y] = v;
      self.transform[13] = v;
      let v = self.cal_size(self.current_style[TRANSLATE_Z], self.current_unit[TRANSLATE_Z], self.offset_width);
      self.computed_style[TRANSLATE_Z] = v;
      self.transform[14] = v;
      let v = self.current_style[ROTATE_X];
      self.computed_style[ROTATE_X] = v;
      multiply_rotate_x(&mut self.transform, v);
      let v = self.current_style[ROTATE_Y];
      self.computed_style[ROTATE_Y] = v;
      multiply_rotate_y(&mut self.transform, v);
      let v = self.current_style[ROTATE_Z];
      self.computed_style[ROTATE_Z] = v;
      multiply_rotate_z(&mut self.transform, v);
      if (self.current_style[ROTATE_3D_X] != 0.0
        || self.current_style[ROTATE_3D_Y] != 0.0
        || self.current_style[ROTATE_3D_Z] != 0.0) && self.current_style[ROTATE_3D_A] != 0.0 {
        let mut t = identity();
        cal_rotate_3d(&mut t, self.current_style[ROTATE_3D_X],
                      self.current_style[ROTATE_3D_Y],
                      self.current_style[ROTATE_3D_Z],
                      self.current_style[ROTATE_3D_A]);
        multiply(&mut self.transform, &t);
      } else {
        self.computed_style[ROTATE_3D_X] = 0.0;
        self.computed_style[ROTATE_3D_Y] = 0.0;
        self.computed_style[ROTATE_3D_Z] = 0.0;
        self.computed_style[ROTATE_3D_A] = 0.0;
      }
      let v = self.current_style[SKEW_X];
      self.computed_style[SKEW_X] = v;
      multiply_skew_x(&mut self.transform, v);
      let v = self.current_style[SKEW_Y];
      self.computed_style[SKEW_Y] = v;
      multiply_skew_y(&mut self.transform, v);
      let v = self.current_style[SCALE_X];
      self.computed_style[SCALE_X] = v;
      multiply_scale_x(&mut self.transform, v);
      let v = self.current_style[SCALE_Y];
      self.computed_style[SCALE_Y] = v;
      multiply_scale_y(&mut self.transform, v);
      let v = self.current_style[SCALE_Z];
      self.computed_style[SCALE_Z] = v;
      multiply_scale_z(&mut self.transform, v);
      assign_m(&mut self.matrix, &self.transform);
      let ox = self.computed_style[TFO_X] + self.x;
      let oy = self.computed_style[TFO_Y] + self.y;
      if ox == 0.0 && oy == 0.0 || is_e(&self.matrix) {
        return
      }
      tfo_multiply(&mut self.matrix, ox, oy);
      multiply_tfo(&mut self.matrix, -ox, -oy);
    }
  }

  fn cal_size(&self, v: f32, u: usize, parent: f32) -> f32 {
    if u == style_unit::PERCENT {
      return v * parent * 0.01
    } else if u == style_unit::VW {
      let root = unsafe { &*self.root };
      return v * root.width * 0.01
    } else if u == style_unit::VH {
      let root = unsafe { &*self.root };
      return v * root.height * 0.01
    } else if u == style_unit::VMAX {
      let root = unsafe { &*self.root };
      return v * f32::max(root.width, root.height) * 0.01
    } else if u == style_unit::VMIN {
      let root = unsafe { &*self.root };
      return v * f32::min(root.width, root.height) * 0.01
    } else if u == style_unit::REM {
      let root = unsafe { &*self.root };
      return v * root.font_size
    }
    v
  }
}
