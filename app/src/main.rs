use base64::engine::{Engine, general_purpose};
use chacha20::{
    ChaCha20,
    cipher::{KeyIvInit, StreamCipher},
};
use hextool::{Convert, Hex};
use k256::{
    AffinePoint, Scalar, elliptic_curve::group::prime::PrimeCurveAffine,
    elliptic_curve::point::AffineCoordinates,
};
use macros::{clear, xyprint};
use rand::Rng;
use sha2::{Digest, Sha256};
fn main() {
    let g = AffinePoint::generator();
    let secret = Sha256::digest(b"augustinhandsome"); // here we'd put any shared secret

    // clear stdout macro
    clear!();
    display_square();
    prompt();

    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).expect("msg err");

    let mut message = Hex::convert(&msg, false, false);
    let bytes16: [u8; 16] = secret[0..16].try_into().unwrap();

    let scalar = u128::from_be_bytes(bytes16);
    let key = (g * Scalar::from(scalar)).to_affine();

    let nonce: [u8; 12] = rand::rng().random();

    let mut cipher = ChaCha20::new(&key.x(), &nonce.into());
    // encrypt message
    clear!();
    unsafe {
        cipher.apply_keystream(message.as_bytes_mut());
        let encrypted_b64 = general_purpose::STANDARD.encode(message.as_bytes());
        println!("\nSuccess! Here is your encrypted message:\n\n{encrypted_b64}");
        println!("And the nonce: {:?}", nonce);
        println!("Your friend needs both to decrypt the message.");
    }
}

//1fn encrypt(msg: &str, secret: &str)

fn display_square() {
    // print at x,y
    for i in 1..=80 {
        xyprint!(i, 20, "--");
        xyprint!(i, 2, "--");
    }
    for i in 2..=20 {
        xyprint!(1, i, "+");
        xyprint!(81, i, "+");
    }
}

fn prompt() {
    xyprint!(10, 5, "Enter your message: ");
}
