use std::ptr;
use wasm_bindgen::prelude::*;
use crate::math::{assign_m, is_e, multiply2};
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
    }
  }

  pub fn add(&mut self, node: *mut Node) -> () {
    self.nodes.push(node);
  }

  pub fn remove(&mut self, i: usize) -> () {
    self.nodes.remove(i);
  }

  pub fn clear(&mut self) -> () {
    self.nodes.clear();
  }

  pub fn size(&self) -> usize {
    self.nodes.len()
  }

  pub fn resize(&mut self, width: f32, height: f32) -> () {
    self.width = width;
    self.height = height;
  }

  // 每帧raf优先存调用，传入运行时间，后续节点动画来计算transition
  pub fn on_frame(&mut self, diff: f32) -> () {
    let mut count = 0;
    let len = self.nodes.len();
    while count < len {
      let mut node = unsafe { &mut *self.nodes[count] };
      node.on_frame(diff);
      count += 1;
    }
  }

  pub fn refresh(&mut self) -> usize {
    let mut count = 1;
    let len = self.nodes.len();
    if len == 0 {
      return len
    }
    self.rl.resize(len, refresh_level::NONE);
    self.me.resize(len, [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    self.op.resize(len, 0.0);
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
    self.rl.push(root.refresh_level);
    self.me.push(c2.clone());
    root.opacity = root.get_op();
    self.op.push(root.opacity);
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
      self.rl.push(node.refresh_level);
      self.me.push(m2.clone());
      node.opacity = p.opacity * node.get_op();
      self.op.push(node.opacity);
      count += 1;
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
}
