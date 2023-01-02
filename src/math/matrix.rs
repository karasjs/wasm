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

pub fn multiply2(m1: &[f32; 16], m2: &[f32; 16], m: &mut [f32; 16]) -> () {
  if is_e(m1) {
    assign_m(m, m2);
  } else if is_e(m2) {
    assign_m(m, m1);
  } else {
    for i in 0..4 {
      let a0 = m1[i];
      let a1 = m1[i + 4];
      let a2 = m1[i + 8];
      let a3 = m1[i + 12];
      m[i] = a0 * m2[0] + a1 * m2[1] + a2 * m2[2] + a3 * m2[3];
      m[i + 4] = a0 * m2[4] + a1 * m2[5] + a2 * m2[6] + a3 * m2[7];
      m[i + 8] = a0 * m2[8] + a1 * m2[9] + a2 * m2[10] + a3 * m2[11];
      m[i + 12] = a0 * m2[12] + a1 * m2[13] + a2 * m2[14] + a3 * m2[15];
    }
  }
}

pub fn multiply_rotate_x(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let v = d2r(v);
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
  let v = d2r(v);
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
  let v = d2r(v);
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
  let tan = d2r(v).tan();
  m[4] += m[0] * tan;
  m[5] += m[1] * tan;
  m[6] += m[2] * tan;
  m[7] += m[3] * tan;
}

pub fn multiply_skew_y(m: &mut [f32; 16], v: f32) -> () {
  if v == 0.0 {
    return
  }
  let tan = d2r(v).tan();
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
  let r = d2r(a);
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

pub fn d2r(d: f32) -> f32 {
  d * 3.141592653589793 / 180.0
}

pub fn cal_rect_point(xa: f32, ya: f32, xb: f32, yb: f32, m: &[f32; 16])
  -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
  let (x1, y1, z1, w1) = cal_point(xa, ya, 0.0, 1.0, m);
  let (x3, y3, z3, w3) = cal_point(xb, yb, 0.0, 1.0, m);
  let mut x2 = 0.0;
  let mut y2 = 0.0;
  let mut z2 = 0.0;
  let mut w2 = 1.0;
  let mut x4 = 0.0;
  let mut y4 = 0.0;
  let mut z4 = 0.0;
  let mut w4 = 1.0;
  // 无旋转的时候可以少算2个点
  if w1 == 1.0 && w3 == 1.0
    && m[1] == 0.0 && m[2] == 0.0 && m[4] == 0.0 && m[6] == 0.0 && m[7] == 0.0 && m[8] == 0.0 {
    x2 = x3;
    y2 = y1;
    z2 = z3;
    x4 = x1;
    y4 = y3;
    z2 = z1;
    z4 = z1;
  } else {
    (x2, y2, z2, w2) = cal_point(xb, ya, 0.0, 1.0, m);
    (x4, y4, z4, w4) = cal_point(xa, yb, 0.0, 1.0, m);
  }
  (x1, y1, z1, w1, x2, y2, z2, w2, x3, y3, z3, w3, x4, y4, z4, w4)
}

pub fn cal_point(x: f32, y: f32, z: f32, w: f32, m: &[f32; 16]) -> (f32, f32, f32, f32) {
  if !is_e(m) {
    let a1 = m[0];
    let b1 = m[1];
    let c1 = m[2];
    let d1 = m[3];
    let a2 = m[4];
    let b2 = m[5];
    let c2 = m[6];
    let d2 = m[7];
    let a3 = m[8];
    let b3 = m[9];
    let c3 = m[10];
    let d3 = m[11];
    let a4 = m[12];
    let b4 = m[13];
    let c4 = m[14];
    let d4 = m[15];
    let mut x0 = if a1 == 1.0 { x } else { x * a1 };
    if a2 != 0.0 {
      x0 += y * a2;
    }
    x0 += if w == 1.0 { a4 } else { a4 * w };
    let mut y0 = if b1 == 1.0 { x } else { x * b1 };
    if b2 != 0.0 {
      y0 += y * b2;
    }
    y0 += if w == 1.0 { b4 } else { b4 * w };
    let mut z0 = 0_f32;
    let mut w0 = w;
    if d1 != 0.0 || d2 != 0.0 || d3 != 0.0 {
      w0 = x * d1 + y * d2 + z * d3 + d4 * w;
    } else if d4 != 1.0 {
      w0 *= d4;
    }
    if z != 0.0 {
      x0 += z * a3;
      y0 += z * b3;
      z0 = x * c1 + y * c2 + c4 + z * c3;
    } else if c1 != 0.0 || c2 != 0.0 || c4 != 0.0 {
      z0 = x * c1 + y * c2 + c4;
    }
    return (x0, y0, z0, w0)
  }
  (x, y, z, w)
}
