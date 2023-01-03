use std::f32;
use wasm_bindgen::prelude::*;
use crate::math::{assign_m, cal_rect_point, multiply2};
use crate::node::Node;
use crate::refresh::refresh_level;

pub const CANVAS: u8 = 0;
pub const WEBGL: u8 = 2;

#[wasm_bindgen]
pub struct Root {
  pub mode: u8,
  pub width: f32,
  pub height: f32,
  pub font_size: f32,
  nodes: Vec<*mut Node>, // 对应js的Root下structs先序遍历的节点列表
  rl: Vec<usize>,
  me: Vec<[f32; 16]>,
  op: Vec<f32>,
  vt: Vec<[f32; 16]>,
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
      rl: Vec::new(),
      me: Vec::new(),
      op: Vec::new(),
      vt: Vec::new(),
    }
  }

  pub fn add_node(&mut self, node: *mut Node) -> () {
    self.nodes.push(node);
  }

  pub fn remove_node(&mut self, i: usize) -> () {
    self.nodes.remove(i);
  }

  pub fn set_node(&mut self, i: usize, node: *mut Node) -> () {
    self.nodes[i] = node;
  }

  pub fn insert_node(&mut self, i: usize, node: *mut Node) -> () {
    self.nodes.insert(i, node);
  }

  pub fn clear(&mut self) -> () {
    self.nodes.clear();
    self.rl.clear();
    self.me.clear();
    self.op.clear();
    self.vt.clear();
  }

  pub fn size(&self) -> usize {
    self.nodes.len()
  }

  pub fn resize(&mut self, width: f32, height: f32) -> () {
    self.width = width;
    self.height = height;
  }

  // 每帧raf优先存调用，传入运行时间，后续节点动画来计算transition
  pub fn on_frame(&mut self, diff: f32) -> usize {
    let mut count = 0;
    let mut res = 0;
    let len = self.nodes.len();
    while count < len {
      let node = unsafe { &mut *self.nodes[count] };
      res += node.on_frame(diff);
      count += 1;
    }
    res
  }

  // 每帧刷新前调用，计算节点列表的matrix和opacity
  pub fn refresh(&mut self) -> usize {
    let mut count = 1;
    let len = self.nodes.len();
    if len == 0 {
      return len
    }
    self.rl.resize(len, refresh_level::NONE);
    self.me.resize(len, [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    self.op.resize(len, 1.0);
    self.vt.resize(len, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    let mut p_list: Vec<usize> = Vec::new();
    let mut last_lv: usize = 0;
    let mut parent: usize = 0; // 存下标，取op/me上的
    // 先设置第0个root，后续则是循环
    let root = unsafe { &mut * self.nodes[0] };
    let (c1, c2) = unsafe {
      (
        & *(root.m_ptr() as *const [f32; 16] as *mut [f32; 16]),
        &mut *(root.me_ptr() as *const [f32; 16] as *mut [f32; 16]),
      )
    };
    assign_m(c2, c1);
    self.rl[0] = root.refresh_level;
    assign_m(&mut self.me[0], c2);
    root.opacity = root.get_op();
    self.op[0] = root.opacity;
    let cx = root.offset_width * 0.5;
    let cy = root.offset_height * 0.5;
    // webgl需计算节点的坐标
    if self.mode == WEBGL {
      let (x1, y1, z1, w1,
        x2, y2, z2, w2,
        x3, y3, z3, w3,
        x4, y4, z4, w4)
        = cal_rect_point(root.xa, root.yb, root.xb, root.ya, c2);
      let mut z = f32::max(z1.abs(), z2.abs());
      z = f32::max(z, z3.abs());
      z = f32::max(z, z4.abs());
      if z != 0.0 {
        z = f32::max(z, (cx * cx + cy * cy).sqrt());
      }
      let (x1, y1, z1, w1) = convert_coords2_gl(x1, y1, z1, w1, cx, cy, z);
      let (x2, y2, z2, w2) = convert_coords2_gl(x2, y2, z2, w2, cx, cy, z);
      let (x3, y3, z3, w3) = convert_coords2_gl(x3, y3, z3, w3, cx, cy, z);
      let (x4, y4, z4, w4) = convert_coords2_gl(x4, y4, z4, w4, cx, cy, z);
      self.vt[0] = [
        x1, y1, z1, w1,
        x2, y2, z2, w2,
        x3, y3, z3, w3,
        x4, y4, w4, z4,
      ];
    }
    // 节点列表
    while count < len {
      let node = unsafe { &mut * self.nodes[count] };
      let lv = node.lv;
      // lv变大说明是child
      if lv > last_lv {
        parent = count - 1;
        p_list.push(parent);
      }
      // 变小可能是parent或另一棵子树
      else if lv < last_lv {
        let diff = last_lv - lv;
        p_list.truncate(p_list.len() - diff);
        parent = p_list[lv - 1];
      }
      last_lv = lv;
      // 不变是sibling无需特殊处理
      // else {}
      let (m1, m2) = unsafe {
        (
          & *(node.m_ptr() as *const [f32; 16] as *mut [f32; 16]),
          &mut *(node.me_ptr() as *const [f32; 16] as *mut [f32; 16]),
        )
      };
      let p = unsafe { & *self.nodes[parent] };
      let pm = unsafe { & *(p.m_ptr() as *const [f32; 16] as *mut [f32; 16]) };
      multiply2(pm, m1,m2);
      self.rl[count] = node.refresh_level;
      assign_m(&mut self.me[count], m2);
      node.opacity = p.opacity * node.get_op();
      self.op[count] = node.opacity;
      // webgl需计算节点的坐标
      if self.mode == WEBGL {
        let (x1, y1, z1, w1,
          x2, y2, z2, w2,
          x3, y3, z3, w3,
          x4, y4, z4, w4)
          = cal_rect_point(node.xa, node.yb, node.xb, node.ya, m2);
        let mut z = f32::max(z1.abs(), z2.abs());
        z = f32::max(z, z3.abs());
        z = f32::max(z, z4.abs());
        if z != 0.0 {
          z = f32::max(z, (cx * cx + cy * cy).sqrt());
        }
        let (x1, y1, z1, w1) = convert_coords2_gl(x1, y1, z1, w1, cx, cy, z);
        let (x2, y2, z2, w2) = convert_coords2_gl(x2, y2, z2, w2, cx, cy, z);
        let (x3, y3, z3, w3) = convert_coords2_gl(x3, y3, z3, w3, cx, cy, z);
        let (x4, y4, z4, w4) = convert_coords2_gl(x4, y4, z4, w4, cx, cy, z);
        self.vt[count] = [
          x1, y1, z1, w1,
          x2, y2, z2, w2,
          x3, y3, z3, w3,
          x4, y4, z4, w4,
        ];
      }
      // 有total的局部根节点可以跳过子节点
      count += 1;
      if node.total > 0 {
        count += node.total;
      }
    }
    len
  }

  pub fn rl_ptr(&self) -> *const usize {
    self.rl.as_ptr()
  }

  pub fn me_ptr(&self) -> *const [f32; 16] {
    self.me.as_ptr()
  }

  pub fn op_ptr(&self) -> *const f32 {
    self.op.as_ptr()
  }

  pub fn vt_ptr(&self) -> *const [f32; 16] {
    self.vt.as_ptr()
  }
}

fn convert_coords2_gl(mut x: f32, mut y: f32, mut z: f32, w: f32, cx: f32, cy: f32, tz: f32) -> (f32, f32, f32, f32) {
  if w != 1.0 {
    x /= w;
    y /= w;
    z /= w;
  }
  if x == cx {
    x = 0.0;
  } else {
    x = (x - cx) / cx;
  }
  if y == cy {
    y = 0.0;
  } else {
    y = (cy - y) / cy;
  }
  if tz != 0.0 {
    z /= -tz;
  }
  if w != 1.0 {
    x *= w;
    y *= w;
    z *= w;
  }
  (x, y, z, w)
}
