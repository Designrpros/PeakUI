use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct AnimationConfig {
    pub spring: Spring,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            spring: Spring::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 170.0,
            damping: 15.0,
            mass: 1.0,
        }
    }
}

impl Spring {
    pub fn bouncy() -> Self {
        Self {
            stiffness: 200.0,
            damping: 10.0,
            mass: 1.0,
        }
    }

    pub fn soft() -> Self {
        Self {
            stiffness: 120.0,
            damping: 20.0,
            mass: 1.0,
        }
    }
}

use iced::{Color, Point};

pub trait Interpolatable: Sized + Copy {
    fn lerp(self, other: Self, t: f32) -> Self;
}

impl Interpolatable for f32 {
    fn lerp(self, other: Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolatable for Color {
    fn lerp(self, other: Self, t: f32) -> Self {
        Color {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

impl Interpolatable for Point {
    fn lerp(self, other: Self, t: f32) -> Self {
        Point {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MotionState {
    pub value: f32,
    pub velocity: f32,
    pub target: f32,
}

impl MotionState {
    pub fn new(initial: f32) -> Self {
        Self {
            value: initial,
            velocity: 0.0,
            target: initial,
        }
    }

    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    pub fn update(&mut self, delta: Duration, spring: Spring) -> bool {
        let dt = delta.as_secs_f32();
        if dt <= 0.0 {
            return false;
        }

        // Spring physics: F = -k(x) - c(v)
        let x = self.value - self.target;
        let force = -spring.stiffness * x - spring.damping * self.velocity;

        let acceleration = force / spring.mass;
        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        // Check if we've settled
        if (self.value - self.target).abs() < 0.001 && self.velocity.abs() < 0.001 {
            self.value = self.target;
            self.velocity = 0.0;
            return false; // Settled
        }

        true // Still moving
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TransitionStyle {
    Fade,
    Scale,
    Slide,
}

#[derive(Debug, Clone, Copy)]
pub struct Transition {
    pub opacity: f32,
    pub scale: f32,
    pub translation_x: f32,
    pub translation_y: f32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            opacity: 1.0,
            scale: 1.0,
            translation_x: 0.0,
            translation_y: 0.0,
        }
    }
}

impl Transition {
    pub fn entry(style: TransitionStyle) -> Self {
        match style {
            TransitionStyle::Fade => Self {
                opacity: 0.0,
                ..Default::default()
            },
            TransitionStyle::Scale => Self {
                scale: 0.8,
                opacity: 0.0,
                ..Default::default()
            },
            TransitionStyle::Slide => Self {
                translation_y: 20.0,
                opacity: 0.0,
                ..Default::default()
            },
        }
    }

    pub fn identity() -> Self {
        Self::default()
    }
}
