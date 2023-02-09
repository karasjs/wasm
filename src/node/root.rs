use std::f64;
use wasm_bindgen::prelude::*;
use crate::{wasm_ptr};
use crate::math::{assign_m, multiply2};
use crate::node::Node;
use crate::refresh::refresh_level;

pub const CANVAS: u8 = 0;
pub const WEBGL: u8 = 2;

#[wasm_bindgen]
pub struct Root {
  pub mode: u8,
  pub width: f64,
  pub height: f64,
  pub font_size: f64,
  nodes: Vec<*mut Node>, // 对应js的Root下structs先序遍历的节点列表
  rl: Vec<usize>,
  me: Vec<[f64; 16]>,
  op: Vec<f64>,
  vt: Vec<[f64; 16]>,
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
    let node = wasm_ptr::transform_mut(node);
    self.nodes.push(node);
    let node = unsafe { &mut *node };
    node.set_root(self);
  }

  pub fn remove_node(&mut self, i: usize) -> () {
    self.nodes.remove(i);
  }

  pub fn set_node(&mut self, i: usize, node: *mut Node) -> () {
    self.nodes[i] = node;
  }

  pub fn insert_node(&mut self, i: usize, node: *mut Node) -> () {
    self.nodes.insert(i, node);
    let node = unsafe { &mut *node };
    node.set_root(self);
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

  pub fn resize(&mut self, width: f64, height: f64) -> () {
    self.width = width;
    self.height = height;
  }

  // 每帧raf优先存调用，传入运行时间，后续节点动画来计算transition
  pub fn on_frame(&mut self, diff: f64) -> usize {
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
  pub fn refresh(&mut self) -> () {
    let mut count = 0;
    let len = self.nodes.len();
    self.rl.resize(len, refresh_level::NONE);
    self.me.resize(len, [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    self.op.resize(len, 1.0);
    self.vt.resize(len, [0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
    let mut p_list: Vec<usize> = Vec::new();
    let mut last_lv: usize = 0;
    let mut parent: usize = 0; // 存下标，取op/me上的
    // 先设置第0个root，后续则是循环
    let root = unsafe { &mut *self.nodes[0] };
    let cx = root.offset_width * 0.5;
    let cy = root.offset_height * 0.5;
    // 节点列表
    while count < len {
      let node = unsafe { &mut *self.nodes[count] };
      let lv = node.lv;
      if lv == 0 {}
      // lv变大说明是child
      else if lv > last_lv {
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
          & *(node.m_ptr() as *const [f64; 16] as *mut [f64; 16]),
          &mut *(node.me_ptr() as *const [f64; 16] as *mut [f64; 16]),
        )
      };
      // 除了root的子节点需要预乘matrix
      if count == 0 {
        assign_m(m2, m1);
        node.opacity = node.get_op();
        self.rl[count] = node.refresh_level;
        assign_m(&mut self.me[count], m2);
        self.op[count] = node.opacity;
      }
      // 文字节点直接用父的matrix和opacity
      else if node.is_text {
        let p = unsafe { & *self.nodes[parent] };
        let pm = unsafe { & *(p.me_ptr() as *const [f64; 16] as *mut [f64; 16]) };
        self.rl[count] = p.refresh_level;
        assign_m(&mut self.me[count], pm);
        self.op[count] = p.opacity;
      }
      else {
        let p = unsafe { & *self.nodes[parent] };
        let pm = unsafe { & *(p.me_ptr() as *const [f64; 16] as *mut [f64; 16]) };
        multiply2(pm, m1, m2);
        node.opacity = p.opacity * node.get_op();
        self.rl[count] = node.refresh_level;
        assign_m(&mut self.me[count], m2);
        self.op[count] = node.opacity;
      }
      // 和js不同不跳total，因为matrix等所有数据都存在这里只一份
      count += 1;
    }
  }

  pub fn rl_ptr(&self) -> *const usize {
    self.rl.as_ptr()
  }

  pub fn me_ptr(&self) -> *const [f64; 16] {
    self.me.as_ptr()
  }

  pub fn op_ptr(&self) -> *const f64 {
    self.op.as_ptr()
  }

  pub fn vt_ptr(&self) -> *const [f64; 16] {
    self.vt.as_ptr()
  }
}

fn convert_coords2_gl(mut x: f64, mut y: f64, mut z: f64, w: f64, cx: f64, cy: f64, tz: f64) -> (f64, f64, f64, f64) {
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
