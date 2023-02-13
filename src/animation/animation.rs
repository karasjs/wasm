use std::f64;
use std::cmp;
use wasm_bindgen::prelude::*;
use crate::{wasm_ptr};
use crate::node::Node;
use crate::animation::{Bezier, easing};

pub const DEFAULT: u8 = 0;
pub const LINEAR: u8 = 1;
pub const EASE_IN: u8 = 2;
pub const EASE_OUT: u8 = 3;
pub const EASE: u8 = 4;
pub const EASE_IN_OUT: u8 = 5;
pub const EASE_CUSTOM: u8 = 6;

pub const NORMAL: u8 = 0;
pub const REVERSE: u8 = 1;
pub const ALTERNATE: u8 = 2;
pub const ALTERNATE_REVERSE: u8 = 3;

pub const NONE: u8 = 0;
pub const FORWARDS: u8 = 1;
pub const BACKWARDS: u8 = 2;
pub const BOTH: u8 = 3;

pub const IDLE: u8 = 0;
pub const RUNNING: u8 = 1;
pub const PAUSED: u8 = 2;
pub const FINISH: u8 = 3;

struct FrameItem {
  k: usize,
  v: f64,
  u: usize,
  d: f64,
}

impl FrameItem {
  fn new(k: usize, v: f64, u: usize, d: f64) -> FrameItem {
    FrameItem {
      k,
      v,
      u,
      d,
    }
  }
}

struct Frame {
  pub list: Vec<FrameItem>,
  time: f64,
  easing: u8,
  bezier: easing::BezierEnum,
}

impl Frame {
  fn new(time: f64, easing: u8, bezier: easing::BezierEnum) -> Frame {
    Frame {
      list: Vec::new(),
      time,
      easing,
      bezier,
    }
  }
}

struct Style {
  k: usize,
  v: f64,
  u: usize,
}

impl Style {
  fn new(k: usize, v: f64, u: usize) -> Style {
    Style {
      k,
      v,
      u,
    }
  }
}

pub(crate) struct Transition {
  pub k: usize,
  pub v: f64,
  pub u: usize,
}

#[wasm_bindgen]
pub struct Animation {
  node: *mut Node,
  frames: Vec<Frame>,
  frames_r: Vec<Frame>,
  direction: u8,
  pub duration: f64,
  pub fps: usize,
  pub delay: f64,
  pub end_delay: f64,
  pub fill: u8,
  pub playback_rate: f64,
  pub iterations: usize,
  pub area_start: f64,
  pub area_duration: f64,
  pub easing: u8,
  bezier: [f64; 4],
  pub current_time: f64,
  pub play_count: usize,
  pub play_state: u8,
  pub first_play: bool,
  is_reverse: bool,
  in_fps: bool,
  fps_time: f64,
  is_delay: bool,
  is_end_delay: bool,
  begin: bool,
  end: bool,
  pub finished: bool,
  last_index: usize,
  last_percent: f64,
  pub index: isize,
  pub percent: f64,
  transition: Vec<Transition>,
  origin: Vec<Style>,
}

#[wasm_bindgen]
impl Animation {
  pub fn new(node: *mut Node, direction: u8, duration: f64, fps: usize,
             delay: f64, end_delay: f64, fill: u8, playback_rate: f64,
             iterations: usize, area_start: f64, area_duration: f64, easing: u8) -> Animation {
    let node = wasm_ptr::transform_mut(node);
    Animation {
      node,
      frames: Vec::new(),
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
      play_count: 0,
      play_state: 0,
      first_play: true,
      is_reverse: direction == REVERSE || direction == ALTERNATE_REVERSE,
      in_fps: false,
      fps_time: 0.0,
      is_delay: false,
      is_end_delay: false,
      begin: true,
      end: false,
      finished: false,
      last_index: 0,
      last_percent: 0.0,
      index: -1,
      percent: -1.0,
      transition: Vec::new(),
      origin: Vec::new(),
    }
  }

  pub fn set_bezier(&mut self, c1: f64, c2: f64, c3: f64, c4: f64) -> () {
    self.bezier[0] = c1;
    self.bezier[1] = c2;
    self.bezier[2] = c3;
    self.bezier[3] = c4;
    self.easing = EASE_CUSTOM;
  }

  pub fn add_frame(&mut self, is_reverse: bool, time: f64, easing: u8, x1: f64, y1: f64, x2: f64, y2: f64) -> () {
    let bezier: easing::BezierEnum = if easing == EASE_IN {
      easing::BezierEnum::EaseIn
    } else if easing == EASE_OUT {
      easing::BezierEnum::EaseOut
    } else if easing == EASE {
      easing::BezierEnum::Ease
    } else if easing == EASE_IN_OUT {
      easing::BezierEnum::EaseInOut
    } else if easing == EASE_CUSTOM {
      easing::BezierEnum::Custom(Bezier::new(x1, y1, x2, y2))
    } else if easing == LINEAR {
      easing::BezierEnum::Linear
    } else {
      if self.easing == EASE_IN {
        easing::BezierEnum::EaseIn
      } else if self.easing == EASE_OUT {
        easing::BezierEnum::EaseOut
      } else if self.easing == EASE {
        easing::BezierEnum::Ease
      } else if self.easing == EASE_IN_OUT {
        easing::BezierEnum::EaseInOut
      } else if self.easing == EASE_CUSTOM {
        easing::BezierEnum::Custom(Bezier::new(self.bezier[0],self.bezier[1], self.bezier[2], self.bezier[3]))
      } else {
        easing::BezierEnum::Linear
      }
    };
    if is_reverse {
      self.frames_r.push(Frame::new(time, easing, bezier));
    } else {
      self.frames.push(Frame::new(time, easing, bezier));
    }
  }

  pub fn add_item(&mut self, is_reverse: bool, k: usize, v: f64, u: usize, d: f64) -> () {
    let fs = if is_reverse { &mut self.frames_r } else { &mut self.frames };
    let wf = fs.last_mut();
    match wf {
      Some(x) => {
        x.list.push(FrameItem::new(k, v, u, d));
      },
      None => panic!(),
    }
  }

  pub fn add_origin(&mut self, k: usize, v: f64, u: usize) -> () {
    self.origin.push(Style::new(k, v, u));
  }

  pub fn play(&mut self) {
    self.current_time = 0_f64;
    self.play_count = 0;
    self.play_state = RUNNING;
    self.first_play = true;
    self.begin = true;
    self.end = false;
    self.is_delay = false;
    self.is_end_delay = false;
  }

  // 和js不同，不设置currentFrames，用is_reverse标识
  pub fn init_current_frames(&mut self, play_count: usize) -> () {
    if self.direction == ALTERNATE || self.direction == ALTERNATE_REVERSE {
      let is_even = play_count % 2 == 0;
      if self.direction == ALTERNATE {
        self.is_reverse = !is_even;
      } else {
        self.is_reverse = is_even;
      }
    } else {
      self.is_reverse = self.direction == REVERSE;
    }
  }

  // 参数和js也不同，直接访问self以及last的判断
  pub fn cal_current(&mut self, dur: f64) -> bool {
    let current_frames = if self.is_reverse { &self.frames_r } else { &self.frames };
    let is_last_count = self.play_count >= self.iterations - 1;
    let length = current_frames.len();
    let play_count = self.play_count;
    let current_time = self.current_time - dur * (play_count as f64);
    // 只有2帧可优化，否则2分查找当前帧
    let index = if length == 2 {
      if current_time < dur { 0 } else { 1 }
    } else {
      binary_search(0, length - 1, current_time, current_frames)
    };
    let current_frame = &current_frames[index];
    // 最后一帧结束动画，仅最后一轮才会进入，需处理endDelay
    let is_last_frame = is_last_count && index == length - 1;
    let mut percent = 0_f64;
    if is_last_frame {
      // 无需任何处理
    }
    // 否则根据目前到下一帧的时间差，计算百分比，再反馈到变化数值上
    else if length == 2 {
      percent = current_time / self.duration; // 不能是dur，按照原本计算
    } else {
      let time = current_frame.time;
      let total = current_frames[index + 1].time - time;
      percent = (current_time - time) / total;
    }
    self.transition.clear();
    // 最后结束特殊处理
    if is_last_frame {
      if self.fill == FORWARDS || self.fill == BOTH {
        // 第一次进入endDelay触发后续不再，并且设置__end标识在after触发END事件
        if !self.is_end_delay {
          self.is_end_delay = true;
          self.end = true;
          let node = unsafe { & *self.node };
          self.transition = cal_last_style(node, current_frame);
        }
        // 有可能刚进endDelay（只有1ms很短）就超过直接finish了，所以只用时间对比
        if current_time >= dur + self.end_delay {
          self.play_count += 1;
          self.finished = true;
        }
      } else {
        // 恢复originStyle
        self.end = true;
        self.play_count += 1;
        self.finished = true;
        let node = unsafe { & *self.node };
        for item in self.origin.iter() {
          if !node.equal_style(item.k, item.v, item.u) {
            self.transition.push(Transition {
              k: item.k,
              v: item.v,
              u: item.u,
            });
          }
        }
      }
    } else {
      // 对比前后两帧是否为同一关键帧，不是则清除之前关键帧上的percent标识为-1，这样可以识别跳帧和本轮第一次进入此帧
      // 这里和js不同，由于不需要回调，前置写在这里判断是否需要计算transition
      if self.index == -1 || (index as isize) != (self.index as isize) || percent != self.percent {
        self.index = index as isize;
        self.percent = percent;
        self.transition = cal_intermediate_style(current_frame, percent);
      }
      // 和js不同无需处理，等待root刷新计算调用
    }
    self.transition.len() > 0
  }

  pub fn before(&mut self, mut diff: f64) -> bool {
    let dur = if self.area_duration > 0.0 {
      f64::min(self.area_duration, self.duration)
    } else {
      self.duration
    };
    // 播放时间累加，并且考虑播放速度加成
    if self.playback_rate != 1.0 {
      diff *= self.playback_rate;
    }
    // 用本帧和上帧时间差，计算累加运行时间currentTime，以便定位当前应该处于哪个时刻
    self.current_time += diff;
    let mut current_time = self.current_time;
    // 增加的fps功能，当<60时计算跳帧，每帧运行依旧累加时间，达到fps时重置，第一帧强制不跳
    if !self.first_play && self.fps > 0 && self.fps != 60 && self.fps != 120 {
      self.fps_time += diff;
      diff = self.fps_time;
      if diff < 1000.0 / (self.fps as f64) {
        self.in_fps = true;
        return false
      }
    }
    if current_time < self.delay - self.area_start {
      self.begin = false; // 默认是true，delay置false防触发
      // 即便不刷新，依旧执行帧回调，同时标明让后续第一帧响应begin
      self.is_delay = true;
      return false
    }
    // 减去delay，计算在哪一帧
    current_time -= self.delay - self.area_start;
    if self.is_delay {
      self.is_delay = false;
      self.begin = true;
    }
    // 超过duration非尾轮需处理回到开头，触发新一轮动画事件，这里可能时间间隔非常大直接跳过几轮
    let mut play_count = (current_time / dur) as usize;
    if play_count > self.iterations - 1 {
      play_count = self.iterations - 1;
    }
    current_time -= dur * (play_count as f64);
    // 如果发生轮换，需重新确定正反向
    if self.play_count < play_count {
      self.begin = true;
      self.play_count = play_count;
      self.init_current_frames(play_count);
    }
    self.cal_current(dur)
  }

  // 用01248来枚举当前状态，返回是否finish
  pub fn after(&mut self) -> bool {
    let node = unsafe { & *self.node };
    let root = unsafe { &mut *node.root };
    if self.in_fps {
      self.in_fps = false;
      root.add_am_state(0);
      return false
    }
    let mut n = 1;
    let mut res = false;
    if self.begin {
      self.begin = false;
      n += 2;
    }
    if self.end {
      self.end = false;
      n += 4;
    }
    if self.finished {
      self.begin = false;
      self.end = false;
      self.is_delay = false;
      self.is_end_delay = false;
      self.finished = false;
      n += 8;
      res = true;
    }
    root.add_am_state(n);
    res
  }

  pub fn goto_stop(&mut self, v: f64, dur: f64) -> bool {
    self.play_state = PAUSED;
    self.current_time = v;
    self.play_count = (v / dur).floor() as usize;
    if self.play_count > self.iterations - 1 {
      self.play_count = self.iterations - 1;
    }
    self.init_current_frames(self.play_count);
    let res = self.cal_current(dur);
    if res {
      let node = unsafe { &mut *self.node };
      node.cal_trans(self);
    }
    res
  }

  pub(crate) fn get_transition(&mut self) -> &Vec<Transition> {
    &self.transition
  }

  pub(crate) fn clear(&mut self) -> () {
    self.frames.clear();
    self.frames_r.clear();
    self.transition.clear();
  }
}

fn binary_search(mut i: usize, mut j: usize, time: f64, frames: &Vec<Frame>) -> usize {
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

fn cal_intermediate_style(current_frame: &Frame, mut percent: f64) -> Vec<Transition> {
  // bezier计算percent
  percent = match &current_frame.bezier {
    easing::BezierEnum::Ease => {
      easing::EASE.timing_function(percent)
    }
    easing::BezierEnum::EaseIn => {
      easing::EASE_IN.timing_function(percent)
    }
    easing::BezierEnum::EaseOut => {
      easing::EASE_OUT.timing_function(percent)
    }
    easing::BezierEnum::EaseInOut => {
      easing::EASE_IN_OUT.timing_function(percent)
    }
    easing::BezierEnum::Custom(b) => {
      b.timing_function(percent)
    }
    _ => {
      percent
    }
  };
  let mut ts: Vec<Transition> = Vec::new();
  for item in current_frame.list.iter() {
    if item.d != 0.0 {
      ts.push(Transition {
        k: item.k,
        v: item.v + item.d * percent,
        u: item.u,
      });
    }
  }
  ts
}

fn cal_last_style(node: &Node, current_frame: &Frame) -> Vec<Transition> {
  let mut ts: Vec<Transition> = Vec::new();
  for item in current_frame.list.iter() {
    if !node.equal_style(item.k, item.v, item.u) {
      ts.push(Transition {
        k: item.k,
        v: item.v,
        u: item.u,
      });
    }
  }
  ts
}
