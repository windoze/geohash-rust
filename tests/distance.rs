extern crate geohash;

use std::num::Float;
use geohash::GeoLocation;

#[test]
fn test_geolocation() {
    let l = GeoLocation::new();
    assert_eq!(l.latitude, 0.0);
    assert_eq!(l.longitude, 0.0);
    let l = GeoLocation::from_coordinates(48.1333, 11.5667);
    assert_eq!(l.latitude, 48.1333);
    assert_eq!(l.longitude, 11.5667);
}

#[test]
fn distance() {
    let new_york = GeoLocation::from_coordinates(40.7127, -74.0059);
    let helsinki = GeoLocation::from_coordinates(60.1708, 24.9375);
    let munich = GeoLocation::from_coordinates(48.1333, 11.5667);

    assert_eq!(new_york.distance_to(helsinki).round(), 6618.337890486902.round());
    assert_eq!(munich.distance_to(helsinki).round(), 1590.1646151045206.round());
}

#[test]
fn sub_distance() {
    let new_york = GeoLocation::from_coordinates(40.7127, -74.0059);
    let helsinki = GeoLocation::from_coordinates(60.1708, 24.9375);
    let munich = GeoLocation::from_coordinates(48.1333, 11.5667);

    assert_eq!((new_york-helsinki).round(), 6618.337890486902.round());
    assert_eq!((munich-helsinki).round(), 1590.1646151045206.round());
}

