use wasm_bindgen::prelude::*;
use crate::node::Node;
use crate::node::Root;

pub const LINEAR: u8 = 0;
pub const EASE_IN: u8 = 1;
pub const EASE_OUT: u8 = 2;
pub const EASE: u8 = 3;
pub const EASE_IN_OUT: u8 = 4;
pub const EASE_CUSTOM: u8 = 5;

pub const NORMAL: u8 = 0;
pub const REVERSE: u8 = 1;
pub const ALTERNATE: u8 = 2;
pub const ALTERNATE_REVERSE: u8 = 3;

pub const NONE: u8 = 0;
pub const FORWARDS: u8 = 1;
pub const BACKWARDS: u8 = 2;
pub const BOTH: u8 = 3;

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
  delay: usize,
  end_delay: usize,
  fill: u8,
  playback_rate: f32,
  easing: u8,
  bezier: [f32; 4],
  iterations: usize,
  pub current_time: usize,
  next_time: usize,
  pub play_count: usize,
  pub index: usize,
  pub percent: f32,
}

#[wasm_bindgen]
impl Animation {
  pub fn new(node: *mut Node, root: *mut Root, direction: u8, duration: usize,
             delay: usize, end_delay: usize, fill: u8, playback_rate: f32,
             easing: u8, iterations: usize) -> Animation {
    Animation {
      node,
      root,
      frames: Vec::new(),
      frames_r: Vec::new(),
      direction,
      duration,
      delay,
      end_delay,
      fill,
      playback_rate,
      iterations,
      easing,
      bezier: [0.0, 0.0, 1.0, 1.0],
      current_time: 0,
      next_time: 0,
      play_count: 0,
      index: 0,
      percent: 0.0,
    }
  }

  pub fn set_bezier(&mut self, c1: f32, c2: f32, c3: f32, c4: f32) -> () {
    self.bezier[0] = c1;
    self.bezier[1] = c2;
    self.bezier[2] = c3;
    self.bezier[3] = c4;
    self.easing = EASE_CUSTOM;
  }

  pub fn add_frame(&mut self) -> () {
    self.frames.push(Frame::new());
    self.frames_r.push(Frame::new());
  }

  pub fn add_item(&mut self, k: u8, v: f32) -> () {
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

  pub fn on_frame(&mut self, delta: usize) -> () {
    let root = unsafe {
      &mut *self.root
    };
  }
}
