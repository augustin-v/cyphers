use std::hash::{DefaultHasher, Hash, Hasher};

use k256::{AffinePoint, Scalar, elliptic_curve::group::prime::PrimeCurveAffine};
fn main() {
    let g = AffinePoint::generator();
    let mut hasher = DefaultHasher::default();
    b"augustinhandsome".hash(&mut hasher); // here we'd put any shared secret
    let secret = hasher.finish();
    let key = g * Scalar::from(secret);
    println!("My key is the x coordinate in the point : {:?}", key);
}
