extern crate geohashrust;

use geohashrust::{GeoLocation, BinaryHash, encode, decode};

#[test]
fn test_encode() {
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 1)=="w");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 2)=="wt");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 3)=="wtw");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 4)=="wtw7");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 5)=="wtw77");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 6)=="wtw77z");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 7)=="wtw77zs");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 8)=="wtw77zs2");
	assert!(encode(GeoLocation::from_coordinates(31.55, 121.46), 9)=="wtw77zs2p");

	let l=GeoLocation{
	        latitude:31.16373922,
	        longitude:121.62585927,
	};
	assert_eq!(encode(l, 1u8), "w");
	assert_eq!(encode(l, 2u8), "wt");
	assert_eq!(encode(l, 3u8), "wtw");
	assert_eq!(encode(l, 4u8), "wtw3");
	assert_eq!(encode(l, 5u8), "wtw3r");
	assert_eq!(encode(l, 6u8), "wtw3r9");
	assert_eq!(encode(l, 7u8), "wtw3r9j");
	assert_eq!(encode(l, 8u8), "wtw3r9jj");
	assert_eq!(encode(l, 9u8), "wtw3r9jjz");
	assert_eq!(encode(l, 10u8), "wtw3r9jjzy");
	assert_eq!(encode(l, 11u8), "wtw3r9jjzyj");
	assert_eq!(encode(l, 12u8), "wtw3r9jjzyjc");
}

#[test]
fn test_binary_hash() {
    assert_eq!(BinaryHash::from_string("111001100111100").to_string(), "111001100111100");
    assert_eq!(BinaryHash::from_string("111001100111100000111100010001100011111111").to_string(), "111001100111100000111100010001100011111111");

    let l=GeoLocation{
            latitude:31.23,
            longitude:121.473,
    };
    assert_eq!(BinaryHash::encode(l, 8).to_string(), "11100110");
    assert_eq!(BinaryHash::encode(l, 15).to_string(), "111001100111100");
    assert_eq!(BinaryHash::encode(l, 42).to_string(), "111001100111100000111100010001100011111111");

	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 5).to_string(), "11100");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 10).to_string(), "1110011001");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 15).to_string(), "111001100111100");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 20).to_string(), "11100110011110000111");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 25).to_string(), "1110011001111000011100111");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 30).to_string(), "111001100111100001110011111111");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 35).to_string(), "11100110011110000111001111111111000");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 40).to_string(), "1110011001111000011100111111111100000010");
	assert_eq!(BinaryHash::encode(GeoLocation::from_coordinates(31.55, 121.46), 45).to_string(), "111001100111100001110011111111110000001010101");

    assert!(BinaryHash::from_string("11100").decode().contains(GeoLocation::from_coordinates(21.0, 113.0)));
    assert!(!BinaryHash::from_string("11100").decode().contains(GeoLocation::from_coordinates(81.0, 113.0)));
    assert!(BinaryHash::from_string("111001100111100").decode().contains(GeoLocation::from_coordinates(31.6, 121.6)));
    assert!(!BinaryHash::from_string("111001100111100").decode().contains(GeoLocation::from_coordinates(51.0, 121.6)));

    assert!(BinaryHash::decode_string("11100").contains(GeoLocation::from_coordinates(21.0, 113.0)));
    assert!(!BinaryHash::decode_string("11100").contains(GeoLocation::from_coordinates(81.0, 113.0)));
    assert!(BinaryHash::decode_string("111001100111100").contains(GeoLocation::from_coordinates(31.6, 121.6)));
    assert!(!BinaryHash::decode_string("111001100111100").contains(GeoLocation::from_coordinates(51.0, 121.6)));
}

#[test]
fn test_decode() {
	let p=GeoLocation::from_coordinates(31.55, 121.46);
	assert!(decode("w").contains(p));
	assert!(decode("wt").contains(p));
	assert!(decode("wtw").contains(p));
	assert!(decode("wtw7").contains(p));
	assert!(decode("wtw77").contains(p));
	assert!(decode("wtw77z").contains(p));
	assert!(decode("wtw77zs").contains(p));
	assert!(decode("wtw77zs2").contains(p));
	assert!(decode("wtw77zs2p").contains(p));

	assert!(decode("w").contains(GeoLocation::from_coordinates(21.0, 113.0)));
	assert!(decode("wtw3r9").contains(GeoLocation::from_coordinates(31.1655, 121.624)));
	assert!(decode("wtw3r9jjz").contains(GeoLocation::from_coordinates(31.163728, 121.625841)));
	assert!(!decode("wtw3r9jjz").contains(GeoLocation::from_coordinates(32.163728, 121.625841)));
	assert!(decode("wtw3r9jjzyjc").contains(GeoLocation::from_coordinates(31.16373922, 121.62585927)));
	assert!(!decode("wtw3r9jjzyjc").contains(GeoLocation::from_coordinates(31.16373922, 121.63585927)));
}
















