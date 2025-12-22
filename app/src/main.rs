use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use hextool::{Convert, Hex};
use k256::{
    AffinePoint, Scalar, elliptic_curve::group::prime::PrimeCurveAffine,
    elliptic_curve::point::AffineCoordinates,
};
use macros::xy;
use sha2::{Digest, Sha256};
fn main() {
    tete();
    let g = AffinePoint::generator();
    let secret = Sha256::digest(b"augustinhandsome"); // here we'd put any shared secret

    for _ in 0..10 {
        horizontal_line();
    }

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

//1fn encrypt(msg: &str, secret: &str)

fn horizontal_line() {
    println!("--");
}
fn vertical_line() {
    println!("|");
}
#[xy(x = 10, y = 1)]
fn tete() {}
