use std::f32::consts::PI;
use wasm_bindgen::prelude::*;

pub fn identity() -> [f32; 16] {
  [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
}

pub fn is_e(m: &[f32; 16]) -> bool {
  if m[0] == 1.0
    && m[1] == 0.0
    && m[2] == 0.0
    && m[3] == 0.0
    && m[4] == 0.0
    && m[5] == 1.0
    && m[6] == 0.0
    && m[7] == 0.0
    && m[8] == 0.0
    && m[9] == 0.0
    && m[10] == 1.0
    && m[11] == 0.0
    && m[12] == 0.0
    && m[13] == 0.0
    && m[14] == 0.0
    && m[15] == 1.0 {
    return true
  }
  false
}


pub fn assign_m(m1: &mut [f32; 16], m2: &[f32; 16]) -> () {
  m1[0] = m2[0];
  m1[1] = m2[1];
  m1[2] = m2[2];
  m1[3] = m2[3];
  m1[4] = m2[4];
  m1[5] = m2[5];
  m1[6] = m2[6];
  m1[7] = m2[7];
  m1[8] = m2[8];
  m1[9] = m2[9];
  m1[10] = m2[10];
  m1[11] = m2[11];
  m1[12] = m2[12];
  m1[13] = m2[13];
  m1[14] = m2[14];
  m1[15] = m2[15];
}

pub fn multiply(m1: &mut [f32; 16], m2: &[f32; 16]) -> () {
  if is_e(m1) {
    assign_m(m1, m2);
  } else if is_e(m2) {
    //
  } else {
    for i in 0..4 {
      let a0 = m1[i];
      let a1 = m1[i + 4];
      let a2 = m1[i + 8];
      let a3 = m1[i + 12];
      m1[i] = a0 * m2[0] + a1 * m2[1] + a2 * m2[2] + a3 * m2[3];
      m1[i + 4] = a0 * m2[4] + a1 * m2[5] + a2 * m2[6] + a3 * m2[7];
      m1[i + 8] = a0 * m2[8] + a1 * m2[9] + a2 * m2[10] + a3 * m2[11];
      m1[i + 12] = a0 * m2[12] + a1 * m2[13] + a2 * m2[14] + a3 * m2[15];
    }
  }
}

pub fn multiply_rotate_x(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let sin = v.sin();
  let cos = v.cos();
  let e = m[4];
  let f = m[5];
  let g = m[6];
  let h = m[7];
  let i = m[8];
  let k = m[10];
  let l = m[11];
  m[4] = e * cos + i * sin;
  m[5] = f * cos + g * sin;
  m[6] = g * cos + k * sin;
  m[7] = h * cos + l * sin;
  m[8] = e * -sin + i * cos;
  m[9] = f * -sin + g * cos;
  m[10] = g * -sin + k * cos;
  m[11] = h * -sin + l * cos;
}

pub fn multiply_rotate_y(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let sin = v.sin();
  let cos = v.cos();
  let a = m[0];
  let b = m[1];
  let c = m[2];
  let d = m[3];
  let i = m[8];
  let j = m[9];
  let k = m[10];
  let l = m[11];
  m[0] = a * cos + i * -sin;
  m[1] = b * cos + j * -sin;
  m[2] = c * cos + k * -sin;
  m[3] = d * cos + l * -sin;
  m[8] = a * sin + i * cos;
  m[9] = b * sin + j * cos;
  m[10] = c * sin + k * cos;
  m[11] = d * sin + l * cos;
}

pub fn multiply_rotate_z(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let sin = v.sin();
  let cos = v.cos();
  let a = m[0];
  let b = m[1];
  let c = m[2];
  let d = m[3];
  let e = m[4];
  let f = m[5];
  let g = m[6];
  let h = m[7];
  m[0] = a * cos + e * sin;
  m[1] = b * cos + f * sin;
  m[2] = c * cos + g * sin;
  m[3] = d * cos + h * sin;
  m[4] = a * -sin + e * cos;
  m[5] = b * -sin + f * cos;
  m[6] = c * -sin + g * cos;
  m[7] = d * -sin + h * cos;
}

pub fn multiply_skew_x(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let tan = v.tan();
  m[4] += m[0] * tan;
  m[5] += m[1] * tan;
  m[6] += m[2] * tan;
  m[7] += m[3] * tan;
}

pub fn multiply_skew_y(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let tan = v.tan();
  m[0] += m[4] * tan;
  m[1] += m[5] * tan;
  m[2] += m[6] * tan;
  m[3] += m[7] * tan;
}

pub fn multiply_scale_x(m: &mut [f32; 16], v: f32) -> () {
  if v == 1.0 {
    return
  }
  m[0] *= v;
  m[1] *= v;
  m[2] *= v;
  m[3] *= v;
}

pub fn multiply_scale_y(m: &mut [f32; 16], v: f32) -> () {
  if v == 1.0 {
    return
  }
  m[4] *= v;
  m[5] *= v;
  m[6] *= v;
  m[7] *= v;
}

pub fn multiply_scale_z(m: &mut [f32; 16], v: f32) -> () {
  if v == 1.0 {
    return
  }
  m[8] *= v;
  m[9] *= v;
  m[10] *= v;
  m[11] *= v;
}

pub fn tfo_multiply(m: &mut [f32; 16], x: f32, y: f32) -> () {
  if x == 0.0 && y == 0.0 {
    return
  }
  let d = m[3];
  let h = m[7];
  let l = m[11];
  let p = m[15];
  m[0] += d * x;
  m[1] += d * y;
  m[4] += h * x;
  m[5] += h * y;
  m[8] += l * x;
  m[9] += l * y;
  m[12] += p * x;
  m[13] += p * y;
}

pub fn multiply_tfo(m: &mut [f32; 16], x: f32, y: f32) -> () {
  if x == 0.0 && y == 0.0 {
    return
  }
  m[12] += m[0] * x + m[4] * y;
  m[13] += m[1] * x + m[5] * y;
  m[14] += m[2] * x + m[6] * y;
  m[15] += m[3] * x + m[7] * y;
}

pub fn cal_rotate_3d(t: &mut[f32; 16], mut x: f32, mut y: f32, mut z: f32, a: f32) -> () {
  let r = a * PI / 180.0;
  let mut s = r.sin();
  let mut c = r.cos();
  if x != 0.0 && y == 0.0 && z == 0.0 {
    if x < 0.0 {
      s = -s;
    }
    t[5] = c;
    t[9] = -s;
    t[6] = s;
    t[10] = c;
  } else if y != 0.0 && x == 0.0 && z == 0.0 {
    if y < 0.0 {
      s = -s;
    }
    t[0] = c;
    t[8] = s;
    t[2] = -s;
    t[10] = c;
  } else if z != 0.0 && x == 0.0 && y == 0.0 {
    if z < 0.0 {
      s = -s;
    }
    t[0] = c;
    t[4] = -s;
    t[1] = s;
    t[5] = c;
  } else {
    let len = (x * x + y * y + z * z).sqrt();
    if len != 1.0 {
      let r = 1.0 / len;
      x *= r;
      y *= r;
      z *= r;
    }
    let nc = 1.0 - c;
    let xy = x * y;
    let yz = y * z;
    let zx = z * x;
    let xs = x * s;
    let ys = y * s;
    let zs = z * s;

    t[0] = x * x * nc + c;
    t[1] = xy * nc + zs;
    t[2] = zx * nc - ys;

    t[4] = xy * nc - zs;
    t[5] = y * y * nc + c;
    t[6] = yz * nc + xs;

    t[8] = zx * nc + ys;
    t[9] = yz * nc - xs;
    t[10] = z * z * nc + c;
  }
}
