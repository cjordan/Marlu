// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Handling of Earth Coordinates (Latitude/Longitude/Height)

use std::fmt::Display;

use erfa_sys::{ERFA_GRS80, ERFA_WGS72, ERFA_WGS84};

use super::ErfaError;
use crate::{
    constants::{MWA_HEIGHT_M, MWA_LAT_RAD, MWA_LONG_RAD},
    XyzGeocentric,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
/// An earth position: Latitude, Longitude and Height [radians, meters]
pub struct LatLngHeight {
    /// Longitude \[radians\]
    pub longitude_rad: f64,
    /// Latitude \[radians\]
    pub latitude_rad: f64,
    /// Height above ellipsoid \[meters\]
    pub height_metres: f64,
}

/// Enum of erfa-compatible reference ellipsoids.
pub enum Ellipsoid {
    /// WGS84 reference ellipsoid
    WGS84 = ERFA_WGS84 as isize,
    /// GRS80 reference ellipsoid
    GRS80 = ERFA_GRS80 as isize,
    /// WGS72 reference ellipsoid
    WGS72 = ERFA_WGS72 as isize,
}

impl LatLngHeight {
    /// Provide a new [`LatLngHeight`] at the MWA's position.
    pub fn new_mwa() -> LatLngHeight {
        Self {
            longitude_rad: MWA_LONG_RAD,
            latitude_rad: MWA_LAT_RAD,
            height_metres: MWA_HEIGHT_M,
        }
    }

    /// Convert to [`XyzGeocentric`] via [`erfa_sys::eraGd2gc`] with the specified
    /// [`Ellipsoid`]
    ///
    /// # Errors
    ///
    /// Can return an [`ErfaError`] if [`erfa_sys::eraGd2gc`] fails.
    pub fn to_geocentric(self, ellipsoid: Ellipsoid) -> Result<XyzGeocentric, ErfaError> {
        let mut geocentric_vector: [f64; 3] = [0.0; 3];
        let status = unsafe {
            erfa_sys::eraGd2gc(
                ellipsoid as i32,               // ellipsoid identifier (Note 1)
                self.longitude_rad,             // longitude (radians, east +ve)
                self.latitude_rad,              // latitude (geodetic, radians, Note 3)
                self.height_metres,             // height above ellipsoid (geodetic, Notes 2,3)
                geocentric_vector.as_mut_ptr(), // geocentric vector (Note 2)
            )
        };
        if status != 0 {
            return Err(ErfaError {
                source_file: file!(),
                source_line: line!(),
                status,
                function: "eraGd2gc",
            });
        }
        Ok(XyzGeocentric {
            x: geocentric_vector[0],
            y: geocentric_vector[1],
            z: geocentric_vector[2],
        })
    }

    /// Convert to geocentric via the default [`Ellipsoid::WGS84`].
    ///
    /// # Errors
    ///
    /// Can return an [`ErfaError`] if [`erfa_sys::eraGd2gc`] fails.
    pub fn to_geocentric_wgs84(self) -> Result<XyzGeocentric, ErfaError> {
        self.to_geocentric(Ellipsoid::WGS84)
    }
}

impl Display for LatLngHeight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ longitude: {:.4}°, latitude: {:.4}°, height: {}m }}",
            self.longitude_rad.to_degrees(),
            self.latitude_rad.to_degrees(),
            self.height_metres
        )
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::AbsDiffEq for LatLngHeight {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.longitude_rad, &other.longitude_rad, epsilon)
            && f64::abs_diff_eq(&self.latitude_rad, &other.latitude_rad, epsilon)
            && f64::abs_diff_eq(&self.height_metres, &other.height_metres, epsilon)
    }
}

#[cfg(any(test, feature = "approx"))]
impl approx::RelativeEq for LatLngHeight {
    #[inline]
    fn default_max_relative() -> f64 {
        f64::EPSILON
    }

    #[inline]
    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        f64::relative_eq(
            &self.longitude_rad,
            &other.longitude_rad,
            epsilon,
            max_relative,
        ) && f64::relative_eq(
            &self.latitude_rad,
            &other.latitude_rad,
            epsilon,
            max_relative,
        ) && f64::relative_eq(
            &self.height_metres,
            &other.height_metres,
            epsilon,
            max_relative,
        )
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
    use approx::assert_abs_diff_eq;

    use crate::constants::{MWA_HEIGHT_M, MWA_LAT_RAD, MWA_LONG_RAD};

    use super::*;

    #[test]
    fn test_display_latlngheight() {
        let latlngheight = LatLngHeight {
            longitude_rad: 0.0,
            latitude_rad: 0.0,
            height_metres: 0.0,
        };
        let result = format!("{}", latlngheight);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_abs_diff_eq() {
        let latlngheight = LatLngHeight {
            longitude_rad: MWA_LONG_RAD * 0.9999999999,
            latitude_rad: MWA_LAT_RAD * 0.9999999999,
            height_metres: MWA_HEIGHT_M * 0.9999999999,
        };

        assert_abs_diff_eq!(latlngheight, LatLngHeight::new_mwa(), epsilon = 1e-7);
    }
}
