// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Interferometric direction-cosine coordinates.
//!
//! This coordinate system is discussed at length in Interferometry and
//! Synthesis in Radio Astronomy, Third Edition, Section 3: Analysis of the
//! Interferometer Response.

use std::f64::consts::TAU;

use super::uvw::UVW;

/// (l,m,n) direction-cosine coordinates. There are no units (i.e.
/// dimensionless).
///
/// This coordinate system is discussed at length in Interferometry and
/// Synthesis in Radio Astronomy, Third Edition, Section 3: Analysis of the
/// Interferometer Response.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub struct LMN {
    /// l coordinate \[dimensionless\]
    pub l: f64,
    /// m coordinate \[dimensionless\]
    pub m: f64,
    /// n coordinate \[dimensionless\]
    pub n: f64,
}

impl LMN {
    /// Get the dot product of a [`UVW`] with a [`LMN`], i.e. `2 * pi * (u * l +
    /// v * m + w * (n - 1))`
    pub fn dot(self, uvw: UVW) -> f64 {
        TAU * (uvw.u * self.l + uvw.v * self.m + uvw.w * (self.n - 1.0))
    }

    /// Subtract 1 from `n` and multiply each of (`l`,`m`,`n`) by 2pi. This is
    /// convenient for application with the radio interferometer measurement
    /// equation (RIME), as performing some multiplies and subtracts ahead of
    /// time could result in many fewer FLOPs.
    pub fn prepare_for_rime(self) -> LmnRime {
        LmnRime {
            l: TAU * self.l,
            m: TAU * self.m,
            n: TAU * (self.n - 1.0),
        }
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::AbsDiffEq for LMN {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.l, &other.l, epsilon)
            && f64::abs_diff_eq(&self.m, &other.m, epsilon)
            && f64::abs_diff_eq(&self.n, &other.n, epsilon)
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::RelativeEq for LMN {
    #[inline]
    fn default_max_relative() -> f64 {
        f64::EPSILON
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        f64::relative_eq(&self.l, &other.l, epsilon, max_relative)
            && f64::relative_eq(&self.m, &other.m, epsilon, max_relative)
            && f64::relative_eq(&self.n, &other.n, epsilon, max_relative)
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

/// A "radio interferometer measurement equation (RIME)"-ready version of
/// [`LMN`]; i.e. `LmnRime.l == 2 * pi * LMN.l`, `LmnRime.m == 2 * pi * LMN.m`,
/// `LmnRime.n == 2 * pi * (LMN.n - 1)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LmnRime {
    /// 2 * pi * l \[dimensionless\]
    pub l: f64,
    /// 2 * pi * m \[dimensionless\]
    pub m: f64,
    /// 2 * pi * (n - 1) \[dimensionless\]
    pub n: f64,
}

impl LmnRime {
    /// Get the dot product of a [`UVW`] with a [`LmnRime`], i.e. `2 * pi * (u *
    /// l + v * m + w * (n - 1))`
    pub fn dot(self, uvw: UVW) -> f64 {
        uvw.u * self.l + uvw.v * self.m + uvw.w * self.n
    }

    /// Convert this [`LmnRime`] to a [`LMN`].
    pub fn to_lmn(self) -> LMN {
        LMN {
            l: self.l / TAU,
            m: self.m / TAU,
            n: self.n / TAU + 1.0,
        }
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::AbsDiffEq for LmnRime {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.l, &other.l, epsilon)
            && f64::abs_diff_eq(&self.m, &other.m, epsilon)
            && f64::abs_diff_eq(&self.n, &other.n, epsilon)
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::RelativeEq for LmnRime {
    #[inline]
    fn default_max_relative() -> f64 {
        f64::EPSILON
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        f64::relative_eq(&self.l, &other.l, epsilon, max_relative)
            && f64::relative_eq(&self.m, &other.m, epsilon, max_relative)
            && f64::relative_eq(&self.n, &other.n, epsilon, max_relative)
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
    use approx::assert_abs_diff_eq;
    use std::f64::consts::PI;

    #[test]
    fn test_lmn_dot() {
        let lmn = LMN {
            l: 0.5,
            m: 0.5,
            n: 0.707,
        };
        let uvw = UVW {
            u: 1.0,
            v: 2.0,
            w: 3.0,
        };
        assert_abs_diff_eq!(lmn.dot(uvw), 3.9018580757585224);
    }

    #[test]
    fn test_lmn_prepare_for_rime() {
        let lmn = LMN {
            l: 0.5,
            m: 0.5,
            n: 0.707,
        };
        let lmn_rime = lmn.prepare_for_rime();
        assert_abs_diff_eq!(
            lmn_rime,
            LmnRime {
                l: PI,
                m: PI,
                n: -1.840973295003619
            }
        );
    }

    #[test]
    fn test_lmn_rime_dot() {
        let lmn = LmnRime {
            l: 0.5,
            m: 0.5,
            n: 0.707,
        };
        let uvw = UVW {
            u: 1.0,
            v: 2.0,
            w: 3.0,
        };
        assert_abs_diff_eq!(lmn.dot(uvw), 3.621);
    }

    #[test]
    fn test_lmn_rime_to_lmn() {
        let lmn = LMN {
            l: 0.5,
            m: 0.5,
            n: 0.707,
        };
        let lmn_rime = lmn.prepare_for_rime();
        let lmn2 = lmn_rime.to_lmn();
        assert_abs_diff_eq!(lmn, lmn2);
    }
}
