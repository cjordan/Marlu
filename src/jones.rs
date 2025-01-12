// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Code for general Jones matrix math.
//!
//! It's not ideal to use LAPACK for matrix multiplies or inverses, because it
//! is not possible to optimise only for 2x2 matrices. Here, we supply the math
//! for these special cases.
//!
//! Parts of the code are derived from Torrance Hodgson's `MWAjl`:
//! <https://github.com/torrance/MWAjl/blob/master/src/matrix2x2.jl>

use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Complex;
use num_traits::{float::FloatCore, Float, Num, NumAssign, Zero};

#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Jones<F: Float + Num>([Complex<F>; 4]);

impl<F: Float> Jones<F> {
    /// Return an identity matrix. All imaginary parts are zero.
    #[inline]
    pub fn identity() -> Self {
        Self::from([
            Complex::new(F::one(), F::zero()),
            Complex::new(F::zero(), F::zero()),
            Complex::new(F::zero(), F::zero()),
            Complex::new(F::one(), F::zero()),
        ])
    }

    /// Return a matrix with all real and imaginary parts set to NaN
    /// [`Jones::any_nan`()] will return `true` for this Jones matrix.
    #[inline]
    pub fn nan() -> Self {
        Self::from([
            Complex::new(F::nan(), F::nan()),
            Complex::new(F::nan(), F::nan()),
            Complex::new(F::nan(), F::nan()),
            Complex::new(F::nan(), F::nan()),
        ])
    }

    /// From an input Jones matrix, get a copy that has been Hermitian
    /// conjugated (`J^H`).
    #[inline]
    pub fn h(self) -> Self {
        Self::from([
            self[0].conj(),
            self[2].conj(),
            self[1].conj(),
            self[3].conj(),
        ])
    }

    /// Multiply by a Jones matrix which gets Hermitian conjugated (`J^H`).
    #[inline]
    pub fn mul_hermitian(self, b: Self) -> Self {
        self * b.h()
    }

    /// Get the inverse of the Jones matrix (`J^I`).
    ///
    /// Ideally, `J^I . J = I`. However it's possible that `J` is singular, in
    /// which case the contents of `J^I` are all NaN.
    #[inline]
    pub fn inv(self) -> Self {
        let inv_det = Complex::new(F::one(), F::zero()) / (self[0] * self[3] - self[1] * self[2]);
        Self::from([
            inv_det * self[3],
            -inv_det * self[1],
            -inv_det * self[2],
            inv_det * self[0],
        ])
    }

    /// Call [`Complex::norm_sqr()`] on each element of a Jones matrix.
    #[inline]
    pub fn norm_sqr(self) -> [F; 4] {
        [
            self[0].norm_sqr(),
            self[1].norm_sqr(),
            self[2].norm_sqr(),
            self[3].norm_sqr(),
        ]
    }

    #[inline]
    pub fn axb(a: Self, b: Self) -> Self {
        a * b
    }

    #[inline]
    pub fn axbh(a: Self, b: Self) -> Self {
        a * b.h()
    }

    #[inline]
    pub fn to_complex_array(self) -> [Complex<F>; 4] {
        self.0
    }

    #[inline]
    pub fn to_float_array(self) -> [F; 8] {
        [
            self.0[0].re,
            self.0[0].im,
            self.0[1].re,
            self.0[1].im,
            self.0[2].re,
            self.0[2].im,
            self.0[3].re,
            self.0[3].im,
        ]
    }
}

impl<F: Float + FloatCore> Jones<F> {
    /// Are any elements of this [Jones] NaN?
    #[inline]
    pub fn any_nan(self) -> bool {
        self.iter().any(|f| f.is_nan())
    }
}

impl<F: Float + NumAssign> Jones<F> {
    #[inline]
    pub fn plus_axb(c: &mut Self, a: Self, b: Self) {
        c[0] += a[0] * b[0] + a[1] * b[2];
        c[1] += a[0] * b[1] + a[1] * b[3];
        c[2] += a[2] * b[0] + a[3] * b[2];
        c[3] += a[2] * b[1] + a[3] * b[3];
    }

    #[inline]
    pub fn plus_ahxb(c: &mut Self, a: Self, b: Self) {
        c[0] += a[0].conj() * b[0] + a[2].conj() * b[2];
        c[1] += a[0].conj() * b[1] + a[2].conj() * b[3];
        c[2] += a[1].conj() * b[0] + a[3].conj() * b[2];
        c[3] += a[1].conj() * b[1] + a[3].conj() * b[3];
    }
}

impl<F: Float> Deref for Jones<F> {
    type Target = [Complex<F>; 4];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Float> DerefMut for Jones<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<F: Float> From<[Complex<F>; 4]> for Jones<F> {
    #[inline]
    fn from(arr: [Complex<F>; 4]) -> Self {
        Self(arr)
    }
}

impl<F: Float> From<[F; 8]> for Jones<F> {
    #[inline]
    fn from(arr: [F; 8]) -> Self {
        Self([
            Complex::new(arr[0], arr[1]),
            Complex::new(arr[2], arr[3]),
            Complex::new(arr[4], arr[5]),
            Complex::new(arr[6], arr[7]),
        ])
    }
}

impl<F: Float> Add<Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl<F: Float + NumAssign> AddAssign<Jones<F>> for Jones<F> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl<F: Float + NumAssign> AddAssign<&Jones<F>> for Jones<F> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl<F: Float> Sub<Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from([
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl<F: Float> Sub<&Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        Self::from([
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl<F: Float> Sub<&mut Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: &mut Self) -> Self {
        Self::from([
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl<F: Float + NumAssign> SubAssign<Jones<F>> for Jones<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl<F: Float + NumAssign> SubAssign<&Jones<F>> for Jones<F> {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

impl<F: Float> Mul<F> for Jones<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: F) -> Self {
        Jones::from([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
    }
}

impl<F: Float> Mul<Complex<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Complex<F>) -> Self {
        Jones::from([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
    }
}

impl<F: Float> Mul<Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::from([
            self[0] * rhs[0] + self[1] * rhs[2],
            self[0] * rhs[1] + self[1] * rhs[3],
            self[2] * rhs[0] + self[3] * rhs[2],
            self[2] * rhs[1] + self[3] * rhs[3],
        ])
    }
}

impl<F: Float> Mul<&Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        Self::from([
            self[0] * rhs[0] + self[1] * rhs[2],
            self[0] * rhs[1] + self[1] * rhs[3],
            self[2] * rhs[0] + self[3] * rhs[2],
            self[2] * rhs[1] + self[3] * rhs[3],
        ])
    }
}

impl<F: Float + NumAssign> MulAssign<F> for Jones<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: F) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

impl<F: Float + NumAssign> MulAssign<Complex<F>> for Jones<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: Complex<F>) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

impl<F: Float + NumAssign> MulAssign<Jones<F>> for Jones<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = [
            self[0] * rhs[0] + self[1] * rhs[2],
            self[0] * rhs[1] + self[1] * rhs[3],
            self[2] * rhs[0] + self[3] * rhs[2],
            self[2] * rhs[1] + self[3] * rhs[3],
        ];
    }
}

impl<F: Float + NumAssign> MulAssign<&Jones<F>> for Jones<F> {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 = [
            self[0] * rhs[0] + self[1] * rhs[2],
            self[0] * rhs[1] + self[1] * rhs[3],
            self[2] * rhs[0] + self[3] * rhs[2],
            self[2] * rhs[1] + self[3] * rhs[3],
        ];
    }
}

impl<F: Float> Div<F> for Jones<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: F) -> Self {
        Jones::from([self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs])
    }
}

impl<F: Float> Div<Complex<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Complex<F>) -> Self {
        Jones::from([self[0] / rhs, self[1] / rhs, self[2] / rhs, self[3] / rhs])
    }
}

impl<F: Float> Div<Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        let inv_det = Complex::new(F::one(), F::zero()) / (rhs[0] * rhs[3] - rhs[1] * rhs[2]);
        Self::from([
            (self[0] * rhs[3] - self[1] * rhs[2]) * inv_det,
            (self[1] * rhs[0] - self[0] * rhs[1]) * inv_det,
            (self[2] * rhs[3] - self[3] * rhs[2]) * inv_det,
            (self[3] * rhs[0] - self[2] * rhs[1]) * inv_det,
        ])
    }
}

impl<F: Float> Div<&Jones<F>> for Jones<F> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: &Self) -> Self {
        let inv_det = Complex::new(F::one(), F::zero()) / (rhs[0] * rhs[3] - rhs[1] * rhs[2]);
        Self::from([
            (self[0] * rhs[3] - self[1] * rhs[2]) * inv_det,
            (self[1] * rhs[0] - self[0] * rhs[1]) * inv_det,
            (self[2] * rhs[3] - self[3] * rhs[2]) * inv_det,
            (self[3] * rhs[0] - self[2] * rhs[1]) * inv_det,
        ])
    }
}

impl<F: Float + NumAssign> DivAssign<F> for Jones<F> {
    #[inline]
    fn div_assign(&mut self, rhs: F) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}

impl<F: Float + NumAssign> DivAssign<Complex<F>> for Jones<F> {
    #[inline]
    fn div_assign(&mut self, rhs: Complex<F>) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}

impl<F: Float + NumAssign> DivAssign<Jones<F>> for Jones<F> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let inv_det = Complex::new(F::one(), F::zero()) / (rhs[0] * rhs[3] - rhs[1] * rhs[2]);
        *self = Self::from([
            (self[0] * rhs[3] - self[1] * rhs[2]) * inv_det,
            (self[1] * rhs[0] - self[0] * rhs[1]) * inv_det,
            (self[2] * rhs[3] - self[3] * rhs[2]) * inv_det,
            (self[3] * rhs[0] - self[2] * rhs[1]) * inv_det,
        ]);
    }
}

impl<F: Float + NumAssign> DivAssign<&Jones<F>> for Jones<F> {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        let inv_det = Complex::new(F::one(), F::zero()) / (rhs[0] * rhs[3] - rhs[1] * rhs[2]);
        *self = Self::from([
            (self[0] * rhs[3] - self[1] * rhs[2]) * inv_det,
            (self[1] * rhs[0] - self[0] * rhs[1]) * inv_det,
            (self[2] * rhs[3] - self[3] * rhs[2]) * inv_det,
            (self[3] * rhs[0] - self[2] * rhs[1]) * inv_det,
        ]);
    }
}

impl<F: Float> Zero for Jones<F> {
    #[inline]
    fn zero() -> Self {
        Self::from([Complex::zero(); 4])
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }
}

impl From<Jones<f32>> for Jones<f64> {
    #[inline]
    fn from(j_c32: Jones<f32>) -> Self {
        Self::from([
            Complex::new(j_c32[0].re as _, j_c32[0].im as _),
            Complex::new(j_c32[1].re as _, j_c32[1].im as _),
            Complex::new(j_c32[2].re as _, j_c32[2].im as _),
            Complex::new(j_c32[3].re as _, j_c32[3].im as _),
        ])
    }
}

impl From<&Jones<f32>> for Jones<f64> {
    #[inline]
    fn from(j_c32: &Jones<f32>) -> Self {
        Self::from([
            Complex::new(j_c32[0].re as _, j_c32[0].im as _),
            Complex::new(j_c32[1].re as _, j_c32[1].im as _),
            Complex::new(j_c32[2].re as _, j_c32[2].im as _),
            Complex::new(j_c32[3].re as _, j_c32[3].im as _),
        ])
    }
}

impl From<Jones<f64>> for Jones<f32> {
    #[inline]
    fn from(j_c64: Jones<f64>) -> Self {
        Self::from([
            Complex::new(j_c64[0].re as _, j_c64[0].im as _),
            Complex::new(j_c64[1].re as _, j_c64[1].im as _),
            Complex::new(j_c64[2].re as _, j_c64[2].im as _),
            Complex::new(j_c64[3].re as _, j_c64[3].im as _),
        ])
    }
}

impl From<&Jones<f64>> for Jones<f32> {
    #[inline]
    fn from(j_c64: &Jones<f64>) -> Self {
        Self::from([
            Complex::new(j_c64[0].re as _, j_c64[0].im as _),
            Complex::new(j_c64[1].re as _, j_c64[1].im as _),
            Complex::new(j_c64[2].re as _, j_c64[2].im as _),
            Complex::new(j_c64[3].re as _, j_c64[3].im as _),
        ])
    }
}

impl std::fmt::Display for Jones<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[[{:e}{:+e}j, {:e}{:+e}j] [{:e}{:+e}j, {:e}{:+e}j]]",
            self[0].re,
            self[0].im,
            self[1].re,
            self[1].im,
            self[2].re,
            self[2].im,
            self[3].re,
            self[3].im,
        )
    }
}

impl std::fmt::Display for Jones<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[[{:e}{:+e}j, {:e}{:+e}j] [{:e}{:+e}j, {:e}{:+e}j]]",
            self[0].re,
            self[0].im,
            self[1].re,
            self[1].im,
            self[2].re,
            self[2].im,
            self[3].re,
            self[3].im,
        )
    }
}

impl std::fmt::Debug for Jones<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[[{:e}{:+e}j, {:e}{:+e}j] [{:e}{:+e}j, {:e}{:+e}j]]",
            self[0].re,
            self[0].im,
            self[1].re,
            self[1].im,
            self[2].re,
            self[2].im,
            self[3].re,
            self[3].im,
        )
    }
}

impl std::fmt::Debug for Jones<f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[[{:e}{:+e}j, {:e}{:+e}j] [{:e}{:+e}j, {:e}{:+e}j]]",
            self[0].re,
            self[0].im,
            self[1].re,
            self[1].im,
            self[2].re,
            self[2].im,
            self[3].re,
            self[3].im,
        )
    }
}

#[cfg(any(test, feature = "approx"))]
impl<F: Float + approx::AbsDiffEq> approx::AbsDiffEq for Jones<F>
where
    F::Epsilon: Copy,
{
    type Epsilon = F::Epsilon;

    #[inline]
    fn default_epsilon() -> F::Epsilon {
        F::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: F::Epsilon) -> bool {
        self.into_iter()
            .zip(other.into_iter())
            .all(|(s, o)| Complex::<F>::abs_diff_eq(&s, &o, epsilon))
    }
}

#[cfg(any(test, feature = "approx"))]
impl<F: Float + approx::AbsDiffEq + approx::RelativeEq> approx::RelativeEq for Jones<F>
where
    F::Epsilon: Copy,
{
    #[inline]
    fn default_max_relative() -> F::Epsilon {
        F::default_epsilon()
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: F::Epsilon, max_relative: F::Epsilon) -> bool {
        self.into_iter().zip(other.into_iter()).all(|(s, o)| {
            F::relative_eq(&s.re, &o.re, epsilon, max_relative)
                && F::relative_eq(&s.im, &o.im, epsilon, max_relative)
        })
    }

    #[inline]
    fn relative_ne(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        !Self::relative_eq(self, other, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{c32, c64};
    use approx::assert_abs_diff_eq;

    fn one_through_eight() -> Jones<f64> {
        Jones([
            c64::new(1.0, 2.0),
            c64::new(3.0, 4.0),
            c64::new(5.0, 6.0),
            c64::new(7.0, 8.0),
        ])
    }

    #[test]
    fn test_add() {
        let a = one_through_eight();
        let b = one_through_eight();
        let c = a + b;
        let expected_c = Jones([
            c64::new(2.0, 4.0),
            c64::new(6.0, 8.0),
            c64::new(10.0, 12.0),
            c64::new(14.0, 16.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_sub() {
        let a = one_through_eight();
        let b = one_through_eight();
        let c = a - b;
        let expected_c = Jones::default();
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_mul() {
        let i = c64::new(1.0, 2.0);
        let a = Jones([i, i + 1.0, i + 2.0, i + 3.0]);
        let b = Jones([i * 2.0, i * 3.0, i * 4.0, i * 5.0]);
        let c = a * b;
        let expected_c = Jones([
            c64::new(-14.0, 32.0),
            c64::new(-19.0, 42.0),
            c64::new(-2.0, 56.0),
            c64::new(-3.0, 74.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_mul_assign() {
        let i = c64::new(1.0, 2.0);
        let mut a = Jones([i, i + 1.0, i + 2.0, i + 3.0]);
        let b = Jones([i * 2.0, i * 3.0, i * 4.0, i * 5.0]);
        a *= b;
        let expected = Jones([
            c64::new(-14.0, 32.0),
            c64::new(-19.0, 42.0),
            c64::new(-2.0, 56.0),
            c64::new(-3.0, 74.0),
        ]);
        assert_abs_diff_eq!(a, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_mul_hermitian() {
        let ident = Jones::identity();
        let a = Jones([
            c64::new(1.0, 2.0),
            c64::new(3.0, 4.0),
            c64::new(5.0, 6.0),
            c64::new(7.0, 8.0),
        ]);
        // A^H is the conjugate transpose.
        let result = ident.mul_hermitian(a);
        let expected = Jones([
            c64::new(1.0, -2.0),
            c64::new(5.0, -6.0),
            c64::new(3.0, -4.0),
            c64::new(7.0, -8.0),
        ]);
        assert_abs_diff_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_div() {
        let a = Jones([
            c64::new(1.0, 0.0),
            c64::new(2.0, 0.0),
            c64::new(3.0, 0.0),
            c64::new(4.0, 0.0),
        ]);
        let b = Jones([
            c64::new(2.0, 0.0),
            c64::new(3.0, 0.0),
            c64::new(4.0, 0.0),
            c64::new(5.0, 0.0),
        ]);
        let expected = Jones([
            c64::new(1.5, 0.0),
            c64::new(-0.5, 0.0),
            c64::new(0.5, 0.0),
            c64::new(0.5, 0.0),
        ]);
        assert_abs_diff_eq!(a.div(&b), expected, epsilon = 1e-10);

        let a = Jones([
            c32::new(-1295920.9, -1150667.5),
            c32::new(-1116357.0, 1234393.3),
            c32::new(-4028358.8, -281923.3),
            c32::new(-325126.38, 3929351.3),
        ]);
        let b = Jones([
            c32::new(1377080.5, 0.0),
            c32::new(5765.743, -1371240.9),
            c32::new(5765.743, 1371240.9),
            c32::new(1365932.0, 0.0),
        ]);
        let expected = Jones([
            c32::new(-107.08988, -72.43006),
            c32::new(72.34611, -106.29652),
            c32::new(-169.56223, 57.398113),
            c32::new(-57.14347, -167.58751),
        ]);
        assert_abs_diff_eq!(a.div(&b), expected, epsilon = 1e-5);
    }

    #[test]
    fn test_div_singular() {
        let a = Jones([
            c64::new(1.0, 0.0),
            c64::new(2.0, 0.0),
            c64::new(2.0, 0.0),
            c64::new(4.0, 0.0),
        ]);
        let b = a;
        assert!((a / b).any_nan());
    }

    #[test]
    fn test_inv() {
        let a = Jones([
            c64::new(1.0, 2.0),
            c64::new(3.0, 4.0),
            c64::new(5.0, 6.0),
            c64::new(7.0, 8.0),
        ]);
        let result = a.inv() * a;
        let expected = Jones::identity();
        assert_abs_diff_eq!(result, expected, epsilon = 1e-10);
    }

    #[test]
    fn test_inv_singular() {
        let a = Jones([
            c64::new(1.0, 0.0),
            c64::new(2.0, 0.0),
            c64::new(2.0, 0.0),
            c64::new(4.0, 0.0),
        ]);
        assert!(a.inv().any_nan());
    }

    #[test]
    fn test_any_nan_works() {
        let j: Jones<f64> = Jones::nan();
        assert!(j.iter().any(|f| f.is_nan()));
        assert!(j.any_nan());

        let mut j: Jones<f64> = Jones::from([0.0; 8]);
        assert!(!j.any_nan());
        for i in 0..4 {
            j[i] = c64::new(f64::NAN, 0.0);
            assert!(j.any_nan());
            j[i] = c64::zero();
            assert!(!j.any_nan());

            j[i] = c64::new(0.0, f64::NAN);
            assert!(j.any_nan());
            j[i] = c64::zero();
            assert!(!j.any_nan());
        }
    }

    #[test]
    fn test_axb() {
        let i = c64::new(1.0, 2.0);
        let a = Jones([i, i + 1.0, i + 2.0, i + 3.0]);
        let b = Jones([i * 2.0, i * 3.0, i * 4.0, i * 5.0]);
        let c = Jones::axb(a, b);
        let expected_c = Jones([
            c64::new(-14.0, 32.0),
            c64::new(-19.0, 42.0),
            c64::new(-2.0, 56.0),
            c64::new(-3.0, 74.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_axbh() {
        let i = c64::new(1.0, 2.0);
        let a = Jones([i, i + 1.0, i + 2.0, i + 3.0]);
        let b = Jones([i * 2.0, i * 3.0, i * 4.0, i * 5.0]);
        let c = Jones::axbh(a, b);
        let expected_c = Jones([
            c64::new(28.0, -6.0),
            c64::new(50.0, -10.0),
            c64::new(38.0, -26.0),
            c64::new(68.0, -46.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_plus_axb() {
        let a = one_through_eight();
        let b = one_through_eight();
        let mut c = Jones::default();
        Jones::plus_axb(&mut c, a, b);
        let expected_c = Jones([
            c64::new(-12.0, 42.0),
            c64::new(-16.0, 62.0),
            c64::new(-20.0, 98.0),
            c64::new(-24.0, 150.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_plus_ahxb() {
        let a = one_through_eight();
        let b = one_through_eight();
        let mut c = Jones::default();
        Jones::plus_ahxb(&mut c, a, b);
        let expected_c = Jones([
            c64::new(66.0, 0.0),
            c64::new(94.0, -4.0),
            c64::new(94.0, 4.0),
            c64::new(138.0, 0.0),
        ]);
        assert_abs_diff_eq!(c, expected_c, epsilon = 1e-10);
    }

    #[test]
    fn test_from_eight_floats() {
        assert_abs_diff_eq!(
            one_through_eight(),
            Jones::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
        );
    }

    #[test]
    fn test_to_complex_array() {
        let j = one_through_eight();
        let j2 = j.to_complex_array();
        assert_abs_diff_eq!(j[0], j2[0]);
        assert_abs_diff_eq!(j[1], j2[1]);
        assert_abs_diff_eq!(j[2], j2[2]);
        assert_abs_diff_eq!(j[3], j2[3]);
    }

    #[test]
    fn test_to_float_array() {
        let j = one_through_eight();
        let j2 = j.to_float_array();
        assert_abs_diff_eq!(j[0].re, j2[0]);
        assert_abs_diff_eq!(j[0].im, j2[1]);
        assert_abs_diff_eq!(j[1].re, j2[2]);
        assert_abs_diff_eq!(j[1].im, j2[3]);
        assert_abs_diff_eq!(j[2].re, j2[4]);
        assert_abs_diff_eq!(j[2].im, j2[5]);
        assert_abs_diff_eq!(j[3].re, j2[6]);
        assert_abs_diff_eq!(j[3].im, j2[7]);
    }
}
