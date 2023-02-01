use lazy_static::lazy_static;

const NEWTON_ITERATIONS: usize = 4;
const NEWTON_MIN_SLOPE: f64 = 0.001;
const SUBDIVISION_PRECISION: f64 = 0.0000001;
const SUBDIVISION_MAX_ITERATIONS: usize = 10;

const K_SPLINE_TABLE_SIZE: usize = 11;
const K_SAMPLE_STEP_SIZE: f64= 1.0 / (K_SPLINE_TABLE_SIZE as f64 - 1.0);

fn a(a1: f64, a2: f64) -> f64 {
  1.0 - 3.0 * a2 + 3.0 * a1
}

fn b(a1: f64, a2: f64) -> f64 {
  3.0 * a2 - 6.0 * a1
}

fn c(a1: f64) -> f64 {
  3.0 * a1
}

fn cal_c_bezier(t: f64, a1: f64, a2: f64) -> f64 {
  ((a(a1, a2) * t + b(a1, a2)) * t + c(a1)) * t
}

fn get_slop(t: f64, a1: f64, a2: f64) -> f64 {
  3.0 * a(a1, a2) * t * t + 2.0 * b(a2, a2) * t + c(a1)
}

fn binary_subdivide(x: f64, mut a: f64, mut b: f64, x1: f64, x2: f64) -> f64 {
  let mut current_x = 0_f64;
  let mut current_t = 0_f64;
  let mut i = 0;
  loop {
    current_t = a + (b - a) / 2.0;
    current_x = cal_c_bezier(current_t, x1, x2) - x;
    if current_x > 0.0 {
      b = current_t;
    } else {
      a = current_t;
    }
    if current_x.abs() <= SUBDIVISION_PRECISION {
      break;
    }
    i += 1;
    if i >= SUBDIVISION_MAX_ITERATIONS {
      break;
    }
  }
  current_t
}

fn newton_raphson_iterate(x: f64, mut gt: f64, x1: f64, x2: f64) -> f64 {
  let mut i = 0;
  while i < NEWTON_ITERATIONS {
    let current_slop = get_slop(gt, x1, x2);
    if current_slop == 0.0 {
      return gt
    }
    let current_x = cal_c_bezier(gt, x1, x2) - x;
    gt -= current_x / current_slop;
    i += 1;
  }
  gt
}

fn linear_easing(x: f64) -> f64 {
  x
}

pub struct Bezier {
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  sample_values: [f64; K_SPLINE_TABLE_SIZE],
}

impl Bezier {
  pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Bezier {
    let mut sample_values = [0_f64; K_SPLINE_TABLE_SIZE];
    let mut i = 0;
    while i < K_SPLINE_TABLE_SIZE {
      sample_values[i] = cal_c_bezier(i as f64 * K_SAMPLE_STEP_SIZE, x1, x2);
      i += 1;
    }
    Bezier {
      x1,
      y1,
      x2,
      y2,
      sample_values,
    }
  }

  fn get_t_for_x(&self, x: f64) -> f64 {
    let mut interval_start = 0_f64;
    let mut current_sample = 1;
    let last_sample = K_SPLINE_TABLE_SIZE - 1;
    while current_sample != last_sample && self.sample_values[current_sample] <= x {
      interval_start += K_SAMPLE_STEP_SIZE;
      current_sample += 1;
    }
    current_sample -= 1;

    let dist = (x - self.sample_values[current_sample]) /
      (self.sample_values[current_sample + 1] - self.sample_values[current_sample]);
    let gt = interval_start + dist * K_SAMPLE_STEP_SIZE;

    let initial_slope = get_slop(gt, self.x1, self.x2);
    return if initial_slope >= NEWTON_MIN_SLOPE {
      newton_raphson_iterate(x, gt, self.x1, self.x2)
    } else if initial_slope == 0.0 {
      gt
    } else {
      binary_subdivide(x, interval_start, interval_start + K_SAMPLE_STEP_SIZE, self.x1, self.x2)
    }
  }

  pub fn timing_function(&self, x: f64) -> f64 {
    if x == 0.0 || x == 1.0 {
      return x
    }
    let v = self.get_t_for_x(x);
    cal_c_bezier(v, self.y1, self.y2)
  }
}

pub fn bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> Bezier {
  Bezier::new(x1, y1, x2, y2)
}

pub enum BezierEnum {
  Linear,
  EaseIn,
  EaseOut,
  Ease,
  EaseInOut,
  Custom(Bezier),
}

lazy_static! {
  pub static ref LINEAR: Bezier = Bezier::new(0.0, 0.0, 1.0, 1.0);
  pub static ref EASE_IN: Bezier = Bezier::new(0.42, 0.0, 1.0, 1.0);
  pub static ref EASE_OUT: Bezier = Bezier::new(0.0, 0.0, 0.58, 1.0);
  pub static ref EASE: Bezier = Bezier::new(0.25, 0.1, 0.25, 1.0);
  pub static ref EASE_IN_OUT: Bezier = Bezier::new(0.42, 0.0, 0.58, 1.0);
}
