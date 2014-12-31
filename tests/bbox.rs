extern crate geohash;

use geohash::{GeoLocation, BoundingBox};
#[test]
fn box_create() {
    let b=BoundingBox::new();
    assert!(b.min_lat==0.0);
    assert!(b.min_lon==0.0);
    assert!(b.max_lat==0.0);
    assert!(b.max_lon==0.0);

    let b=BoundingBox::from_coordinates(34.0, 12.0, 78.0, 56.0);
    assert!(b.min_lat==12.0);
    assert!(b.min_lon==56.0);
    assert!(b.max_lat==34.0);
    assert!(b.max_lon==78.0);

	let b=BoundingBox::from_geolocations(
			GeoLocation{
				latitude:23.0,
				longitude:89.0,
			},
			GeoLocation{
				latitude:67.0,
				longitude:45.0,
			},
		);
	assert!(b.bottom_left().latitude==23.0);
	assert!(b.bottom_left().longitude==45.0);
	assert!(b.top_right().latitude==67.0);
	assert!(b.top_right().longitude==89.0);
}

#[test]
fn box_contains() {
	let box1=BoundingBox::from_geolocations(
			GeoLocation{
				latitude:23.0,
				longitude:89.0,
			},
			GeoLocation{
				latitude:67.0,
				longitude:45.0,
			},
		);
	// Inside
	assert!(box1.contains(GeoLocation{
				latitude:34.0,
				longitude:56.0,
	}));
	// Border
	assert!(box1.contains(GeoLocation{
				latitude:23.0,
				longitude:89.0,
	}));
	// Below
	assert!(!box1.contains(GeoLocation{
				latitude:12.0,
				longitude:67.0,
	}));
	// Above
	assert!(!box1.contains(GeoLocation{
				latitude:89.0,
				longitude:67.0,
	}));
	// Left
	assert!(!box1.contains(GeoLocation{
				latitude:34.0,
				longitude:23.0,
	}));
	// Right
	assert!(!box1.contains(GeoLocation{
				latitude:34.0,
				longitude:23.0,
	}));
	// Left-Below
	assert!(!box1.contains(GeoLocation{
				latitude:12.0,
				longitude:34.0,
	}));
}

#[test]
fn box_expand() {
	let mut box1=BoundingBox::from_geolocations(
			GeoLocation{
				latitude:23.0,
				longitude:89.0,
			},
			GeoLocation{
				latitude:67.0,
				longitude:45.0,
			},
		);
    let box2=BoundingBox::from_geolocations(
        GeoLocation{
            latitude:123.0,
            longitude:89.0,
        },
        GeoLocation{
            latitude:67.0,
            longitude:145.0,
        },
    );
	box1.merge_with(box2);
	assert!(box1.min_lat==23.0);
	assert!(box1.min_lon==45.0);
	assert!(box1.max_lat==123.0);
	assert!(box1.max_lon==145.0);
}
