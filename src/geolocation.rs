use std::ops::Sub;

// The Earth's radius in kilometers.
static EARTH_RADIUS: f64 = 6371.009;

/// A geographic location.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct GeoLocation {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64
}

impl GeoLocation {
    /// Creates a new `GeoLocation` with latitude and longitude set to
    /// zero.
    ///
    /// # Example
    ///
    /// ```
    /// let l = geohashrust::GeoLocation::new();
    /// assert_eq!(l.latitude, 0.0);
    /// assert_eq!(l.longitude, 0.0);
    /// ```
    pub fn new() -> GeoLocation {
        GeoLocation {
            latitude: 0.0,
            longitude: 0.0
        }
    }

    /// Creates a new `GeoLocation` with `latitude` and `longitude`.
    ///
    /// # Example
    ///
    /// ```
    /// let l = geohashrust::GeoLocation::from_coordinates(48.1333, 11.5667);
    /// assert_eq!(l.latitude, 48.1333);
    /// assert_eq!(l.longitude, 11.5667);
    /// ```
    pub fn from_coordinates(latitude: f64, longitude: f64) -> GeoLocation {
        assert!(latitude.abs()<=90.0);
        assert!(longitude.abs()<=180.0);
        GeoLocation {
            latitude: latitude,
            longitude: longitude
        }
    }

    /// Returns the distance between `self` and `other` in meters. The
    /// calculation is done using the Haversine formula.
    ///
    /// # Example
    ///
    /// ```
    /// let new_york = geohashrust::GeoLocation::from_coordinates(40.7127, -74.0059);
    /// let helsinki = geohashrust::GeoLocation::from_coordinates(60.1708, 24.9375);
    /// assert_eq!(new_york.distance_to(&helsinki).round(), 6618.0);
    /// ```
    pub fn distance_to(&self, other: &GeoLocation) -> f64 {
        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let dlat = (other.latitude - self.latitude).to_radians();
        let dlon = (other.longitude - self.longitude).to_radians();

        let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() +
            lat1.cos() * lat2.cos() *
            (dlon / 2.0).sin() * (dlon / 2.0).sin();
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS * c
    }
}

/// Returns the distance between `self` and `other` in meters. The
/// calculation is done using the Haversine formula.
///
/// # Example
///
/// ```
/// let new_york = geohashrust::GeoLocation::from_coordinates(40.7127, -74.0059);
/// let helsinki = geohashrust::GeoLocation::from_coordinates(60.1708, 24.9375);
/// assert_eq!((new_york-helsinki).round(), 6618.0);
/// ```
impl Sub<GeoLocation> for GeoLocation {
    type Output = f64;
    fn sub(self, rhs: GeoLocation) -> f64 {
        self.distance_to(&rhs)
    }
}













