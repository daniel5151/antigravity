//! [https://xkcd.com/353/](https://xkcd.com/353/)
//!
//! ![](https://imgs.xkcd.com/comics/python.png)

/// 🐦
///
/// # Safety
///
/// Must not be called prior to sampling everything in the medicine cabinet.
pub unsafe fn fly() {
    webbrowser::open("https://xkcd.com/353").expect("Gravity too strong.");
}

/// Compute geohash() using the Munroe algorithm.
///
/// ```
/// use antigravity::geohash;
///
/// assert_eq!(geohash(37.421542, -122.085589, b"2005-05-26-10458.68"), (37.857713267707005, -122.54454306955928));
/// ```
pub fn geohash(latitude: f64, longitude: f64, datedow: &[u8]) -> (f64, f64) {
    fn u8_arr_to_f64(arr: &[u8]) -> f64 {
        let mut ans = 0.0;

        let mut mul = 1.0;
        for elem in arr {
            mul /= 256.0;
            ans += *elem as f64 * mul;
        }

        ans
    }

    let hash = md5::compute(datedow);

    let hash_lat_end = u8_arr_to_f64(&hash[0..8]);
    let hash_lon_end = u8_arr_to_f64(&hash[8..16]);

    // handles 0.0 correctly since +0.0.signum() is 1.0
    let hash_lat = latitude.trunc() + hash_lat_end * latitude.signum();
    let hash_lon = longitude.trunc() + hash_lon_end * longitude.signum();

    (hash_lat, hash_lon)
}
