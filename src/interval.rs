use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Add, Neg, Sub};

use crate::float::Float;
use crate::get_state;

pub struct Interval<T> where T: Float {
    pub low: T,
    pub high: T,
    _private: (),
}

fn get_fpu_state() -> i32 {
    unsafe { get_state() }
}

#[inline(always)]
fn check_rounding_mode() {
    debug_assert!(get_fpu_state() != 0,
                  "Interval arithmetic is useless with default rounding mode.\n \
                Use interval_computation to perform your calculations");
}

impl<T: Float> Interval<T> {
    pub fn new() -> Self {
        Self::from_values(T::zero(), T::zero())
    }

    fn from_values(low: T, high: T) -> Self {
        check_rounding_mode();
        Interval { low, high, _private: () }
    }
}

impl<T: Float + Clone> Interval<T> {
    pub fn from(value: T) -> Self {
        Self::from_values(value.clone(), value.clone())
    }
}

impl<T: Float + Clone> Clone for Interval<T> {
    fn clone(&self) -> Self {
        Self::from_values(self.low.clone(), self.high.clone())
    }
}

impl<T: Float + Copy> Copy for Interval<T> {}

impl<T: Float + Display> Display for Interval<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}; {}]", self.low, self.high)
    }
}

impl<T: Float> Add for &Interval<T> {
    type Output = Interval<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::from_values(
            T::add_consuming((&self.low).neg(), (&rhs.low).neg()).neg_consuming(),
            T::add(&self.high, &rhs.high),
        )
    }
}

impl<T: Float> Add for Interval<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { &self + &rhs }
}

impl<T: Float> Neg for &Interval<T> {
    type Output = Interval<T>;

    fn neg(self) -> Self::Output {
        Self::Output::from_values((&self.high).neg(), (&self.low).neg())
    }
}

impl<T: Float> Neg for Interval<T> {
    type Output = Self;
    fn neg(self) -> Self::Output { -&self }
}

impl<T: Float> Sub for &Interval<T> {
    type Output = Interval<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(&rhs.neg())
    }
}

impl<T: Float> Sub for Interval<T> {
    type Output = Interval<T>;

    fn sub(self, rhs: Self) -> Self::Output { &self - &rhs }
}