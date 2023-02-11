use std::ptr;
use wasm_bindgen::prelude::*;
use crate::{wasm_ptr};
use crate::node::Root;
use crate::style::style_unit;
use crate::style::style_key::*;
use crate::refresh::refresh_level;
use crate::animation::{Animation, RUNNING};
use crate::math::*;

#[wasm_bindgen]
pub struct Node {
  pub root: *mut Root,
  pub is_text: bool,
  pub x: f64,
  pub y: f64,
  pub offset_width: f64,
  pub offset_height: f64,
  pub xa: f64,
  pub ya: f64,
  pub xb: f64,
  pub yb: f64,
  pub lv: usize,
  pub refresh_level: usize,
  current_style: [f64; 18],
  current_unit: [usize; 18],
  computed_style: [f64; 18],
  transform: [f64; 16],
  matrix: [f64; 16],
  matrix_event: [f64; 16],
  pub opacity: f64, // 存储包含父继承的最终世界opacity
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
      xa: 0.0,
      ya: 0.0,
      xb: 0.0,
      yb: 0.0,
      lv: 0,
      refresh_level: refresh_level::NONE,
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

  pub fn add_ani(&mut self, animation: *mut Animation) -> () {
    let animation = wasm_ptr::transform_mut(animation);
    self.animations.push(animation);
  }

  pub fn remove_ani(&mut self, animation: *mut Animation) -> () {
    self.animations.retain(|&x| x != animation);
  }

  pub fn clear(&mut self) -> () {
    let mut count = 0;
    let len = self.animations.len();
    while count < len {
      let ani = unsafe { &mut *self.animations[count] };
      ani.clear();
      count += 1;
    }
    self.animations.clear();
  }

  pub fn set_style(&mut self, x: f64, y: f64, offset_width: f64, offset_height: f64,
                   cs0: f64, cs1: f64, cs2: f64, cs3: f64, cs4: f64, cs5: f64,
                   cs6: f64, cs7: f64, cs8: f64, cs9: f64, cs10: f64, cs11: f64, cs12: f64,
                   cs13: f64, cs14: f64, cs15: f64, cs16: f64, cs17: f64,
                   cu0: usize, cu1: usize, cu2: usize, cu16: usize, cu17: usize) -> () {
    self.x = x;
    self.y = y;
    self.offset_width = offset_width;
    self.offset_height = offset_height;
    self.xa = x;
    self.ya = y;
    // cache存的尺寸都是ceil整数，为了在Page中满足尺寸，以及边缘透明需求
    self.xb = x + offset_width.ceil();
    self.yb = y + offset_height.ceil();
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
    self.cal_matrix(refresh_level::REFLOW);
  }

  // 文本style复用parent，所以只需要设置尺寸位置
  pub fn set_txt(&mut self, x: f64, y: f64, offset_width: f64, offset_height: f64) -> () {
    self.x = x;
    self.y = y;
    self.offset_width = offset_width;
    self.offset_height = offset_height;
    self.xa = x;
    self.ya = y;
    self.xb = x + offset_width.ceil();
    self.yb = y + offset_height.ceil();
  }

  pub fn set_transform(&mut self, a: f64, b: f64, c: f64, d: f64,
                       e: f64, f: f64, g: f64, h: f64,
                       i: f64, j: f64, k: f64, l: f64,
                       m: f64, n: f64, o: f64, p: f64) -> () {
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

  pub fn computed_style_ptr(&self) -> *const f64 {
    self.computed_style.as_ptr()
  }

  pub fn transform_ptr(&self) -> *const f64 {
    self.transform.as_ptr()
  }

  pub fn m_ptr(&self) -> *const f64 {
    self.matrix.as_ptr()
  }

  pub fn me_ptr(&self) -> *const f64 {
    self.matrix_event.as_ptr()
  }

  pub fn get_op(&self) -> f64 {
    self.opacity
  }

  pub fn get_rl(&self) -> usize {
    self.refresh_level
  }

  pub fn before(&mut self, mut diff: f64) -> usize {
    let mut count = 0;
    let len = self.animations.len();
    let mut res = 0;
    self.refresh_level = refresh_level::NONE;
    while count < len {
      let ani = unsafe { &mut *self.animations[count] };
      if ani.play_state == RUNNING {
        // 需要刷新的动画返回计数+1
        if ani.before(diff) {
          res += 1;
          self.cal_trans(ani);
        }
      }
      count += 1;
    }
    res
  }

  pub fn after(&mut self) -> usize {
    let mut count = 0;
    let len = self.animations.len();
    let mut res = 0;
    while count < len {
      let ani = unsafe { &mut *self.animations[count] };
      if ani.play_state == RUNNING {
        ani.after();
        res += 1;
      }
      count += 1;
    }
    res
  }

  pub fn cal_trans(&mut self, ani: &mut Animation) {
    let ts = ani.get_transition();
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

  pub fn cal_matrix(&mut self, rl: usize) -> () {
    let mut optimize = true;
    if rl & refresh_level::TRANSFORM > 0 || rl >= refresh_level::REPAINT {
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
        let r = d2r(v);
        let sin = r.sin();
        let cos = r.cos();
        let x = self.computed_style[SCALE_X];
        let y = self.computed_style[SCALE_Y];
        let cx = cos * x;
        self.transform[0] = cx;
        self.matrix[0] = cx;
        let sx = sin * x;
        self.transform[1] = sx;
        self.matrix[1] = sx;
        let sy = -sin * y;
        self.transform[4] = sy;
        self.matrix[4] = sy;
        let cy = cos * y;
        self.transform[5] = cy;
        self.matrix[5] = cy;
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

  pub fn cal_size(&self, v: f64, u: usize, parent: f64) -> f64 {
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
      return v * f64::max(root.width, root.height) * 0.01
    } else if u == style_unit::VMIN {
      let root = unsafe { &*self.root };
      return v * f64::min(root.width, root.height) * 0.01
    } else if u == style_unit::REM {
      let root = unsafe { &*self.root };
      return v * root.font_size
    }
    v
  }

  pub fn equal_style(&self, k: usize, v: f64, u: usize) -> bool {
    if k == TRANSLATE_X || k == TRANSLATE_Z {
      let v = self.cal_size(v, u, self.offset_width);
      return v == self.computed_style[TRANSLATE_X]
    } else if k == TRANSLATE_Y {
      let v = self.cal_size(v, u, self.offset_height);
      return v == self.computed_style[TRANSLATE_Y]
    } else {
      return v == self.computed_style[k]
    }
  }

  pub fn update_style(&mut self, k: usize, v: f64, u: usize) -> () {
    self.current_style[k] = v;
    self.current_unit[k] = u;
    if k == TRANSLATE_X || k == TRANSLATE_Z || k == TFO_X {
      self.computed_style[k] = self.cal_size(v, u, self.offset_width);
    } else if k == TRANSLATE_Y {
      self.computed_style[k] = self.cal_size(v, u, self.offset_height);
    } else {
      self.computed_style[k] = v;
    }
  }
}
