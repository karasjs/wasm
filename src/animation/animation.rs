use std::f32;
use std::cmp;
use wasm_bindgen::prelude::*;
use crate::node::Node;
use crate::node::Root;
use crate::style::{style_key, style_unit};

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
  u: u8,
  d: f32,
}

impl FrameItem {
  fn new(k: u8, v: f32, u: u8, d: f32) -> FrameItem {
    FrameItem {
      k,
      v,
      u,
      d,
    }
  }
}

struct Frame {
  list: Vec<FrameItem>,
  time: f32,
  pub percent: f32,
}

impl Frame {
  fn new(time: f32) -> Frame {
    Frame {
      list: Vec::new(),
      time,
      percent: -1.0,
    }
  }
}

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  // The `console.log` is quite polymorphic, so we can bind it with multiple
  // signatures. Note that we need to use `js_name` to ensure we always call
  // `log` in JS.
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_u32(a: u32);

  // Multiple arguments too!
  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Animation {
  node: *mut Node,
  frames: Vec<Frame>,
  frames_r: Vec<Frame>,
  direction: u8,
  pub duration: f32,
  pub fps: usize,
  pub delay: f32,
  pub end_delay: f32,
  pub fill: u8,
  pub playback_rate: f32,
  pub iterations: usize,
  pub area_start: f32,
  pub area_duration: f32,
  pub easing: u8,
  bezier: [f32; 4],
  pub current_time: f32,
  next_time: f32,
  pub play_count: usize,
  first_enter: bool,
  in_fps: bool,
  fps_time: f32,
  is_delay: bool,
  out_begin_delay: bool,
  begin: bool,
  pub index: usize,
  pub percent: f32,
}

#[wasm_bindgen]
impl Animation {
  pub fn new(node: *mut Node, direction: u8, duration: f32, fps: usize,
             delay: f32, end_delay: f32, fill: u8, playback_rate: f32,
             iterations: usize, area_start: f32, area_duration: f32, easing: u8) -> Animation {
    let frames = Vec::new();
    Animation {
      node,
      frames,
      frames_r: Vec::new(),
      direction,
      duration,
      fps,
      delay,
      end_delay,
      fill,
      playback_rate,
      iterations,
      area_start,
      area_duration,
      easing,
      bezier: [0.0, 0.0, 1.0, 1.0],
      current_time: 0.0,
      next_time: 0.0,
      play_count: 0,
      first_enter: true,
      in_fps: false,
      fps_time: 0.0,
      is_delay: false,
      out_begin_delay: false,
      begin: false,
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

  pub fn add_frame(&mut self, is_reverse: bool, time: f32) -> () {
    if is_reverse {
      self.frames_r.push(Frame::new(time));
    } else {
      self.frames.push(Frame::new(time));
    }
  }

  pub fn add_item(&mut self, is_reverse: bool, k: u8, v: f32, u: u8, d: f32) -> () {
    let fs = if is_reverse { &mut self.frames_r } else { &mut self.frames };
    let wf = fs.last_mut();
    match wf {
      Some(x) => {
        x.list.push(FrameItem::new(k, v, u, d));
      },
      None => panic!(),
    }
  }

  pub fn on_frame(&mut self, mut diff: f32) -> () {
    self.current_time = self.next_time;
    let mut current_time = self.current_time;
    let dur = if self.area_duration > 0.0 { f32::min(self.area_duration, self.duration) } else { self.duration };
    // 播放时间累加，并且考虑播放速度加成
    if self.playback_rate != 1.0 && self.playback_rate > 0.0 {
      diff *= self.playback_rate;
    }
    // 用本帧和上帧时间差，计算累加运行时间currentTime，以便定位当前应该处于哪个时刻
    self.next_time += diff;
    // 增加的fps功能，当<60时计算跳帧，每帧运行依旧累加时间，达到fps时重置，第一帧强制不跳
    if !self.first_enter && self.fps > 0 && self.fps != 60 {
      self.fps_time += diff;
      diff = self.fps_time;
      if diff < 1000.0 / (self.fps as f32) {
        return;
      }
    }
    self.first_enter = false;
    // delay仅第一次生效等待
    if self.current_time < self.delay - self.area_start {
      if (self.fill == BACKWARDS || self.fill == BOTH) && !self.is_delay {
        // TODO
      }
      self.is_delay = true;
      return;
    }
    self.is_delay = false;
    // 减去delay，计算在哪一帧
    current_time -= self.delay - self.area_start;
    if self.out_begin_delay {
      self.out_begin_delay = false;
      self.begin = true;
    }
    // 超过duration非尾轮需处理回到开头，触发新一轮动画事件，这里可能时间间隔非常大直接跳过几轮
    let mut play_count = (current_time / dur) as usize;
    if self.iterations > 0 && self.iterations - 1 < play_count {
      play_count = self.iterations - 1;
    }
    current_time -= dur * (play_count as f32);
    let mut is_reverse = false;
    // 如果发生轮换，需重新确定正反向
    if self.play_count < play_count {
      self.begin = true;
      self.play_count = play_count;
      if self.direction == ALTERNATE || self.direction == ALTERNATE_REVERSE {
        let is_even = play_count % 2 == 0;
        if self.direction == ALTERNATE {
          is_reverse = !is_even;
        } else {
          is_reverse = is_even;
        }
      }
    }
    let is_last_count = if self.iterations == 0 {
      false
    } else {
      play_count >= self.iterations - 1
    };
    let current_frames = if is_reverse { &self.frames_r } else { &self.frames };
    // 只有2帧可优化，否则2分查找当前帧
    let len = current_frames.len();
    let i = if len == 2 {
      if current_time < dur { 0 } else { 1 }
    } else {
      binary_search(0, len - 1, current_time, current_frames)
    };
    // 最后一帧结束动画，仅最后一轮才会进入，需处理endDelay
    let is_last_frame = is_last_count && i == len - 1;
    let mut percent = 0_f32;
    if is_last_frame {
      // 无需任何处理
    }
    // 否则根据目前到下一帧的时间差，计算百分比，再反馈到变化数值上
    else if len == 2 {
      percent = current_time / dur;
    } else {
      let time = current_frames[i].time;
      let total = current_frames[i + 1].time - time;
      percent = (current_time - time) / total;
    }
    let in_end_delay = false;
    let current_frame = &current_frames[i];
    // 对比前后两帧是否为同一关键帧，不是则清除之前关键帧上的percent标识为-1，这样可以识别跳帧和本轮第一次进入此帧
    // if !self.last_frame.is_null() {
    //   let mut last_frame = unsafe { &mut *self.last_frame };
    //   let not_same_frame = current_frame == last_frame;
    //   if not_same_frame {
    //     last_frame.percent = -1.0;
    //     self.last_frame = current_frame as *mut Frame;
    //   }
    // }
    // else {
    //   self.last_frame = current_frame as *mut Frame;
    // }
  }
}

fn binary_search(mut i: usize, mut j: usize, time: f32, frames: &Vec<Frame>) -> usize {
  while i < j {
    if i == j - 1 {
      if frames[i].time <= time {
        return j
      }
      return i
    }
    let middle = i + ((j - i) >> 1);
    let frame = &frames[middle];
    if frame.time > time {
      j = cmp::max(middle - 1, i);
    } else {
      i = cmp::min(middle, j);
    }
  }
  i
}
