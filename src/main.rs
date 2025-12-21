use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use hextool::{Convert, Hex};
use k256::{
    AffinePoint, Scalar, elliptic_curve::group::prime::PrimeCurveAffine,
    elliptic_curve::point::AffineCoordinates,
};
use sha2::{Digest, Sha256};

fn main() {
    let g = AffinePoint::generator();
    let secret = Sha256::digest(b"augustinhandsome"); // here we'd put any shared secret
    let mut message = Hex::convert("Hello world", false, false);
    let bytes16: [u8; 16] = secret[0..16].try_into().unwrap();

    let scalar = u128::from_be_bytes(bytes16);
    let key = (g * Scalar::from(scalar)).to_affine();
    let nonce = [0x12; 12];

    let mut cipher = ChaCha20::new(&key.x(), &nonce.into());
    // encrypt message
    unsafe {
        cipher.apply_keystream(message.as_bytes_mut());
    }
}
