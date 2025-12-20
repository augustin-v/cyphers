use k256::{
    AffinePoint, Scalar, elliptic_curve::group::prime::PrimeCurveAffine,
    elliptic_curve::point::AffineCoordinates,
};
use sha2::{Digest, Sha256};
fn main() {
    let g = AffinePoint::generator();
    let secret = Sha256::digest(b"augustinhandsome"); // here we'd put any shared secret

    let bytes16: [u8; 16] = secret[0..16].try_into().unwrap();
    let scalar = u128::from_be_bytes(bytes16);
    let key = (g * Scalar::from(scalar)).to_affine();

    println!("key is {:?}", key.x());
}
