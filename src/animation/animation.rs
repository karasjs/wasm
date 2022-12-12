use wasm_bindgen::prelude::*;
use crate::node::Node;
use crate::node::Root;

struct FrameItem {
  k: u8,
  v: f32,
  d: f32,
}

impl FrameItem {
  fn new(k: u8, v: f32) -> FrameItem {
    FrameItem {
      k,
      v,
      d: 0.0,
    }
  }
}

struct Frame {
  list: Vec<FrameItem>,
}

impl Frame {
  fn new() -> Frame {
    Frame {
      list: Vec::new(),
    }
  }
}

#[wasm_bindgen]
pub struct Animation {
  node: *mut Node,
  root: *mut Root,
  frames: Vec<Frame>,
  frames_r: Vec<Frame>,
  direction: u8,
  duration: usize,
  current_time: usize,
  next_time: usize,
  iterations: usize,
  play_count: usize,
}

#[wasm_bindgen]
impl Animation {
  pub fn new(node: *mut Node, root: *mut Root, direction: u8, duration: usize, iterations: usize) -> Animation {
    Animation {
      node,
      root,
      frames: Vec::new(),
      frames_r: Vec:: new(),
      direction,
      duration,
      current_time: 0,
      next_time: 0,
      iterations,
      play_count: 0,
    }
  }

  pub fn add_frame(&mut self) -> () {
    self.frames.push(Frame::new());
    self.frames_r.push(Frame::new());
  }

  pub fn add(&mut self, k: u8, v: f32) -> () {
    let wf = self.frames.last_mut();
    match wf {
      Some(x) => {
        x.list.push(FrameItem::new(k, v));
      }
      None => panic!(),
    }
    let wf = self.frames_r.last_mut();
    match wf {
      Some(x) => {
        x.list.push(FrameItem::new(k, v));
      }
      None => panic!(),
    }
  }

  pub fn gen(&mut self) -> () {
    self.frames_r.reverse();
    Animation::cal_transition(&mut self.frames);
    Animation::cal_transition(&mut self.frames_r);
  }

  fn cal_transition(frames: &mut Vec<Frame>) -> () {
    let mut count = frames.len();
    if count > 1 {
      let len2 = frames[0].list.len();
      if len2 > 0 {
        while count > 1 {
          count -= 1;
          let mut o = &mut frames[count];
          let mut count2 = 0;
          while count2 < len2 {
            let nv = o.list[count2].v;
            o = &mut frames[count - 1];
            let pv = o.list[count2].v;
            o.list[count2].d = nv - pv;
            count2 += 1;
            o = &mut frames[count];
          }
        }
      }
    }
  }

  pub fn on_frame(&mut self, index: usize, percent: f32) -> () {
    //
  }
}
