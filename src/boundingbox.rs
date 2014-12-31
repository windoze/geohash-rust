use std::num::FloatMath;
use geolocation::GeoLocation;

#[deriving(Default, Copy, PartialEq)]
pub struct BoundingBox {
	pub min_lat : f64,
	pub max_lat : f64,
	pub min_lon : f64,
	pub max_lon : f64,
}

impl BoundingBox {
    /// Create a new `BoudingBox` with default values
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::new();
    /// assert!(b.min_lat==0.0);
    /// assert!(b.min_lon==0.0);
    /// assert!(b.max_lat==0.0);
    /// assert!(b.max_lon==0.0);
    /// ```
    pub fn new() -> BoundingBox {
        BoundingBox {
            min_lat: 0.0,
            max_lat: 0.0,
            min_lon: 0.0,
            max_lon: 0.0,
        }
    }

    pub fn from_coordinates(minlat:f64, maxlat:f64, minlon:f64, maxlon:f64) -> BoundingBox {
        BoundingBox {
            min_lat: minlat.min(maxlat),
            max_lat: minlat.max(maxlat),
            min_lon: minlon.min(maxlon),
            max_lon: minlon.max(maxlon),
        }
    }

    /// Creates a new `BoundingBox` with 2 GeoLocations
    ///
    /// # Example
    ///
    /// ```
    /// let box1=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude:23.0,
    ///         longitude:89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude:67.0,
    ///         longitude:45.0,
    ///     },
    /// );
    /// assert!(box1.min_lat==23.0);
    /// assert!(box1.min_lon==45.0);
    /// assert!(box1.max_lat==67.0);
    /// assert!(box1.max_lon==89.0);
    /// ```
    pub fn from_geolocations(p1 : GeoLocation, p2 : GeoLocation) -> BoundingBox {
        BoundingBox {
            min_lat: p1.latitude.min(p2.latitude),
            max_lat: p1.latitude.max(p2.latitude),
            min_lon: p1.longitude.min(p2.longitude),
            max_lon: p1.longitude.max(p2.longitude),
        }
    }

    /// Creates a new `BoundingBox` with the merge of 2 BoundingBoxes
    ///
    /// # Example
    ///
    /// ```
    /// let box1=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// let box2=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 123.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 145.0,
    ///     },
    /// );
    /// let box3=geohash::BoundingBox::merged(box1, box2);
    /// assert_eq!(box3.min_lat, 23.0);
    /// assert_eq!(box3.min_lon, 45.0);
    /// assert_eq!(box3.max_lat, 123.0);
    /// assert_eq!(box3.max_lon, 145.0);
    /// ```
    pub fn merged(one: BoundingBox, other: BoundingBox) -> BoundingBox {
    	BoundingBox {
    		min_lat: one.min_lat.min(other.min_lat),
    		max_lat: one.max_lat.max(other.max_lat),
    		min_lon: one.min_lon.min(other.min_lon),
    		max_lon: one.max_lon.max(other.max_lon),
    	}
    }

    /// Get the center point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.center().latitude, 45.0);
    /// assert_eq!(b.center().longitude, 67.0);
    /// ```
    pub fn center(&self) -> GeoLocation {
        GeoLocation {
            latitude: (self.min_lat+self.max_lat)/2.0,
            longitude: (self.min_lon+self.max_lon)/2.0,
        }
    }

    /// Get the top-left point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.top_left().latitude, 67.0);
    /// assert_eq!(b.top_left().longitude, 45.0);
    /// ```
    pub fn top_left(&self) -> GeoLocation {
        GeoLocation {
            latitude: self.max_lat,
            longitude: self.min_lon,
        }
    }

    /// Get the top-right point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.top_right().latitude, 67.0);
    /// assert_eq!(b.top_right().longitude, 89.0);
    /// ```
    pub fn top_right(&self) -> GeoLocation {
        GeoLocation {
            latitude: self.max_lat,
            longitude: self.max_lon,
        }
    }

    /// Get the bottom-left point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.bottom_left().latitude, 23.0);
    /// assert_eq!(b.bottom_left().longitude, 45.0);
    /// ```
    pub fn bottom_left(&self) -> GeoLocation {
    	GeoLocation {
    		latitude: self.min_lat,
    		longitude: self.min_lon,
    	}
    }

    /// Get the bottom-right point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.bottom_right().latitude, 23.0);
    /// assert_eq!(b.bottom_right().longitude, 89.0);
    /// ```
    pub fn bottom_right(&self) -> GeoLocation {
        GeoLocation {
            latitude: self.min_lat,
            longitude: self.max_lon,
        }
    }

    /// Get the latitude range of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.latitude_range(), 44.0);
    /// ```
    pub fn latitude_range(&self) -> f64 {
    	self.max_lat - self.min_lat
    }

    /// Get the longitude range of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 99.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.longitude_range(), 54.0);
    /// ```
    pub fn longitude_range(&self) -> f64 {
    	self.max_lon - self.min_lon
    }

    /// Get the latitude error from the center point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.latitude_error(), 22.0);
    /// ```
    pub fn latitude_error(&self) -> f64 {
        self.latitude_range()/2.0
    }

    /// Get the longitude error from the center point of the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 99.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert_eq!(b.longitude_error(), 27.0);
    /// ```
    pub fn longitude_error(&self) -> f64 {
        self.longitude_range()/2.0
    }

    /// Test if a `GeoLocation` is in the bounding box
    ///
    /// # Example
    ///
    /// ```
    /// let b=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 99.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// assert!(b.contains(geohash::GeoLocation::from_coordinates(33.0, 55.0)));
    /// assert!(!b.contains(geohash::GeoLocation::from_coordinates(13.0, 55.0)));
    /// ```
    pub fn contains(&self, point: GeoLocation) -> bool {
    	(point.latitude >= self.min_lat) && (point.latitude <= self.max_lat)
    	&& (point.longitude >= self.min_lon) && (point.longitude <= self.max_lon)
    }

    /// Merge another `BoundingBox` into this one
    ///
    /// # Example
    ///
    /// ```
    /// let mut box1=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 23.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 45.0,
    ///     },
    /// );
    /// let box2=geohash::BoundingBox::from_geolocations(
    ///     geohash::GeoLocation{
    ///         latitude: 123.0,
    ///         longitude: 89.0,
    ///     },
    ///     geohash::GeoLocation{
    ///         latitude: 67.0,
    ///         longitude: 145.0,
    ///     },
    /// );
    /// box1.merge_with(box2);
    /// assert_eq!(box1.min_lat, 23.0);
    /// assert_eq!(box1.min_lon, 45.0);
    /// assert_eq!(box1.max_lat, 123.0);
    /// assert_eq!(box1.max_lon, 145.0);
    /// ```
    pub fn merge_with(&mut self, other: BoundingBox) {
    	if other.min_lat < self.min_lat {
    		self.min_lat=other.min_lat
    	}
    	if other.min_lon < self.min_lon {
    		self.min_lon=other.min_lon
    	}
    	if other.max_lat > self.max_lat {
    		self.max_lat=other.max_lat
    	}
    	if other.max_lon > self.max_lon {
    		self.max_lon=other.max_lon
    	}
    }
}