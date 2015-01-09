use std::collections::Bitv;
use geolocation::GeoLocation;
use boundingbox::BoundingBox;

static BASE32_CODES: [char; 32] = [
    '0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static BASE32_INDICES: [int; 75]=[
	 0,  1,  2,  3,  4,  5,  6,  7, // 30-37, '0'..'7'
	 8,  9, -1, -1, -1, -1, -1, -1, // 38-2F, '8','9'
	-1, -1, 10, 11, 12, 13, 14, 15, // 40-47, 'B'..'G'
	16, -1, 17, 18, -1, 19, 20, -1, // 48-4F, 'H','J','K','M','N'
	21, 22, 23, 24, 25, 26, 27, 28, // 50-57, 'P'..'W'
	29, 30, 31, -1, -1, -1, -1, -1, // 58-5F, 'X'..'Z'
	-1, -1, 10, 11, 12, 13, 14, 15, // 60-67, 'b'..'g'
	16, -1, 17, 18, -1, 19, 20, -1, // 68-6F, 'h','j','k','m','n'
	21, 22, 23, 24, 25, 26, 27, 28, // 70-77, 'p'..'w'
	29, 30, 31,                     // 78-7A, 'x'..'z'
];

/// Binary hash code for a given `GeoLocation` with specific precision
#[derive(Default, Copy, PartialEq)]
pub struct BinaryHash {
    bits : u64,
    precision : uint,
}

///
impl BinaryHash {
    /// Create an empty `BinaryHash`
    pub fn new() -> BinaryHash {
        BinaryHash{
            bits: 0u64,
            precision: 0u,
        }
    }

    /// Create a `BinaryHash` from a `Bitv`
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::Bitv;
    /// let a=0b11100110;
    /// let bv = Bitv::from_bytes(&[a]);
    /// assert_eq!(geohashrust::BinaryHash::from_bitv(&bv).to_string(), "11100110");
    /// ```
    pub fn from_bitv(bv: &Bitv) -> BinaryHash {
        let mut output=BinaryHash::new();
        for b in bv.iter() {
            output.push(b)
        }
        output
    }

    /// Create a `BinaryHash` from a String
    ///
    /// # Example
    ///
    /// ```
    /// let bh=geohashrust::BinaryHash::from_string("11100110");
    /// assert_eq!(bh.to_string(), "11100110");
    /// ```
    pub fn from_string(s: &str) -> BinaryHash {
        let mut output=BinaryHash::new();
        for c in s.chars() {
            match c {
                '0' => output.push(false),
                '1' => output.push(true),
                _ => panic!("Invalid binary code"),
            }
        }
        output
    }

    /// Encode a `GeoLocation` into binary hash
    ///
    /// # Example
    ///
    /// ```
    /// let l=geohashrust::GeoLocation{
    ///         latitude:31.23,
    ///         longitude:121.473,
    /// };
    /// let bh=geohashrust::BinaryHash::encode(l, 8);
    /// assert_eq!(bh.to_string(), "11100110");
    /// ```
    pub fn encode(l: GeoLocation, precision: uint) -> BinaryHash {
        let mut bbox = BoundingBox::from_coordinates(-90.0, 90.0, -180.0, 180.0);
        let mut islon = true;
        
        let mut output=BinaryHash::new();
        
        while output.len() < precision {
            if islon {
                let mid = (bbox.max_lon + bbox.min_lon) / 2.0;
                if l.longitude > mid {
                    output.push(true);
                    bbox.min_lon=mid;
                } else {
                    output.push(false);
                    bbox.max_lon=mid;
                }
            } else {
                let mid = (bbox.max_lat + bbox.min_lat) / 2.0;
                if l.latitude > mid  {
                    output.push(true);
                    bbox.min_lat = mid;
                } else {
                    output.push(false);
                    bbox.max_lat = mid;
                }
            }
            islon = !islon;
        }
        output
    }

    /// Decode binary hash into a `BoundingBox`
    ///
    /// # Example
    ///
    /// ```
    /// let bh=geohashrust::BinaryHash::from_string("11100");
    /// let bbox=bh.decode();
    /// assert!(bbox.contains(geohashrust::GeoLocation::from_coordinates(21.0, 113.0)));
    /// ```
    pub fn decode(&self) -> BoundingBox {
        let mut output = BoundingBox::from_coordinates(-90.0, 90.0, -180.0, 180.0);
        let mut islon = true;
        
        for n in range(0u, self.precision) {
            if islon {
                let mid = (output.max_lon + output.min_lon) / 2.0;
                if self.test(n) {
                    output.min_lon = mid;
                } else {
                    output.max_lon = mid;
                }
            } else {
                let mid = (output.max_lat + output.min_lat) / 2.0;
                if self.test(n) {
                    output.min_lat = mid;
                } else {
                    output.max_lat = mid;
                }
            }
            islon = !islon;
        }
        output
    }

    /// Decode binary hash into a `BoundingBox`
    ///
    /// # Example
    ///
    /// ```
    /// let bbox=geohashrust::BinaryHash::decode_string("11100");
    /// assert!(bbox.contains(geohashrust::GeoLocation::from_coordinates(21.0, 113.0)));
    /// ```
    pub fn decode_string(s: &str) -> BoundingBox {
        BinaryHash::from_string(s).decode()
    }

    /// Convert `BinaryHash` to a `Bitv`
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::Bitv;
    /// let a=0b11100110;
    /// let bv = Bitv::from_bytes(&[a]);
    /// let bh=geohashrust::BinaryHash::from_string("11100110");
    /// assert_eq!(bh.to_bitv(), bv);
    /// ```
    pub fn to_bitv(&self) -> Bitv {
        let mut output=Bitv::with_capacity(self.precision);
        for n in range(0u, self.precision) {
            output.push(self.test(n))
        }
        output
    }

    /// Convert `BinaryHash` to a `String`
    ///
    /// # Example
    ///
    /// ```
    /// let bh=geohashrust::BinaryHash::from_string("11100110");
    /// assert_eq!(bh.to_string(), "11100110");
    /// ```
    pub fn to_string(&self) -> String {
        let mut output=String::with_capacity(self.precision);
        for n in range(0u, self.precision) {
            output.push(if self.test(n) {'1'} else {'0'})
        }
        output
    }

    /// Return the count of effective bits in the binary hash
    ///
    /// # Example
    ///
    /// ```
    /// let mut bh=geohashrust::BinaryHash::new();
    /// assert_eq!(bh.len(), 0u);
    /// bh.push(true);
    /// bh.push(true);
    /// assert_eq!(bh.len(), 2u);
    /// bh.push(false);
    /// bh.push(true);
    /// assert_eq!(bh.len(), 4u);
    /// ```
    pub fn len(&self) -> uint {
        self.precision
    }

    /// Test if the binary hash is empty
    ///
    /// # Example
    ///
    /// ```
    /// let mut bh=geohashrust::BinaryHash::new();
    /// assert!(bh.empty());
    /// bh.push(true);
    /// bh.push(true);
    /// bh.push(false);
    /// bh.push(true);
    /// assert!(!bh.empty());
    /// ```
    pub fn empty(&self) -> bool {
        self.precision == 0u
    }

    /// Test specific bit of the binary hash
    ///
    /// # Example
    ///
    /// ```
    /// let mut bh=geohashrust::BinaryHash::new();
    /// bh.push(true);
    /// bh.push(true);
    /// bh.push(false);
    /// bh.push(true);
    /// assert!(bh.test(0));
    /// assert!(bh.test(1));
    /// assert!(!bh.test(2));
    /// assert!(bh.test(3));
    /// ```
    pub fn test(&self, n: uint) -> bool {
        (self.bits & (1u64 << (self.precision-n-1u))) != 0
    }

    /// Push a bit into binary hash
    ///
    /// # Example
    ///
    /// ```
    /// let mut bh=geohashrust::BinaryHash::new();
    /// bh.push(true);
    /// bh.push(true);
    /// bh.push(false);
    /// bh.push(true);
    /// assert_eq!(bh.to_string(), "1101");
    /// ```
    pub fn push(&mut self, b: bool) {
        self.bits <<= 1u;
        self.bits |= if b {1u64} else {0u64};
        self.precision += 1u;
    }
}

/// Encode a `GeoLocation` into GeoHash with given precision
///
/// # Example
///
/// ```
/// let l=geohashrust::GeoLocation{
///         latitude:31.16373922,
///         longitude:121.62585927,
/// };
/// assert_eq!(geohashrust::encode(l, 7u), "wtw3r9j");
/// ```
pub fn encode(l: GeoLocation, precision: uint) -> String {
	let mut bbox = BoundingBox::from_coordinates(-90.0, 90.0, -180.0, 180.0);
    let mut islon = true;
    let mut num_bits = 0u;
    let mut hash_index = 0u;

    // Pre-Allocate the hash string
    let mut output=String::with_capacity(precision);
    
    while output.len() < precision {
        if islon {
            let mid = (bbox.max_lon + bbox.min_lon) / 2.0;
            if l.longitude > mid {
                hash_index = (hash_index << 1) + 1;
                bbox.min_lon=mid;
            } else {
                hash_index = (hash_index << 1) + 0;
                bbox.max_lon=mid;
            }
        } else {
            let mid = (bbox.max_lat + bbox.min_lat) / 2.0;
            if l.latitude > mid  {
                hash_index = (hash_index << 1) + 1;
                bbox.min_lat = mid;
            } else {
                hash_index = (hash_index << 1) + 0;
                bbox.max_lat = mid;
            }
        }
        islon = !islon;

        num_bits+=1;
        if num_bits%5==0 {
            output.push(BASE32_CODES[hash_index]);
            hash_index = 0;
        }
    }
    output
}

/// Decode a GeoHash into a `BoundingBox`
///
/// # Example
///
/// ```
/// let bbox=geohashrust::decode("wtw3r9jjz");
/// assert!(bbox.contains(geohashrust::GeoLocation::from_coordinates(31.163728, 121.625841)));
/// ```
pub fn decode(hash: &str) -> BoundingBox {
    let mut output = BoundingBox::from_coordinates(-90.0, 90.0, -180.0, 180.0);
    let mut islon = true;

    for c in hash.chars() {
        assert!(c>='0' && c<='z');
        let char_index = BASE32_INDICES[(c as uint)-48];

        for bits in range(0u, 5).rev() {
            let bit = ((char_index >> bits) & 1)==1;
            if islon {
                let mid = (output.max_lon + output.min_lon) / 2.0;
                if bit {
                    output.min_lon = mid;
                } else {
                    output.max_lon = mid;
                }
            } else {
                let mid = (output.max_lat + output.min_lat) / 2.0;
                if bit {
                    output.min_lat = mid;
                } else {
                    output.max_lat = mid;
                }
            }
            islon = !islon;
        }
    }
    println!("min_lat:{}, max_lat:{}, min_long:{}, max_lon:{}", output.min_lat, output.max_lat, output.min_lon, output.max_lon);
    output
}


/// Get the neighbor of GeoHash on specific direction
///
/// # Example
///
/// ```
/// assert_eq!(geohashrust::neighbor("wtw3s", (-1, -1)), "wtw37");
/// assert_eq!(geohashrust::neighbor("wtw3sjj", (1, -1)), "wtw3sjk");
/// ```
pub fn neighbor(hash: &str, direction: (int, int)) -> String {
	let b = decode(hash);
	let cp = b.center();
	encode(match direction {
		(dlat, dlon) => GeoLocation::from_coordinates(
			cp.latitude + b.latitude_range() * (dlat as f64),
			cp.longitude + b.longitude_range() * (dlon as f64),
		)
	}, hash.len())
}

/// Get a vector of neighbors for the GeoHash on all 8 directions, with itself as the first
///
/// # Example
///
/// ```
/// let ns=geohashrust::neighbors("wtw3s");
/// assert_eq!(ns[0], "wtw3s");
/// assert_eq!(ns[1], "wtw37");
/// assert_eq!(ns[2], "wtw3k");
/// assert_eq!(ns[3], "wtw3m");
/// assert_eq!(ns[4], "wtw3e");
/// assert_eq!(ns[5], "wtw3t");
/// assert_eq!(ns[6], "wtw3g");
/// assert_eq!(ns[7], "wtw3u");
/// assert_eq!(ns[8], "wtw3v");
/// ```
pub fn neighbors(hash: &str) -> Box<Vec<String>> {
	box vec![
		hash.to_string(),
		neighbor(hash, (-1, -1)),
		neighbor(hash, (-1,  0)),
		neighbor(hash, (-1,  1)),
		neighbor(hash, ( 0, -1)),
		neighbor(hash, ( 0,  1)),
		neighbor(hash, ( 1, -1)),
		neighbor(hash, ( 1,  0)),
		neighbor(hash, ( 1,  1)),
	]
}










