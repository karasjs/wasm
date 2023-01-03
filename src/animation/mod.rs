mod animation;
mod easing;

pub use animation::Animation;

pub use animation::LINEAR;
pub use animation::EASE_IN;
pub use animation::EASE_OUT;
pub use animation::EASE;
pub use animation::EASE_IN_OUT;

pub use animation::NORMAL;
pub use animation::REVERSE;
pub use animation::ALTERNATE;
pub use animation::ALTERNATE_REVERSE;

pub use animation::NONE;
pub use animation::FORWARDS;
pub use animation::BACKWARDS;
pub use animation::BOTH;

pub use animation::IDLE;
pub use animation::RUNNING;
pub use animation::PAUSED;
pub use animation::FINISH;

pub use easing::Bezier;
pub use easing::BezierEnum;
// pub use easing::BEZIER_LINEAR;
// pub use easing::BEZIER_EASE_IN;
// pub use easing::BEZIER_EASE;
// pub use easing::BEZIER_EASE_OUT;
// pub use easing::BEZIER_EASE_IN_OUT;
