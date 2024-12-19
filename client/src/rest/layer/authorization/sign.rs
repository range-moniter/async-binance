use general::result::BinanceResult;

#[cfg(feature = "sign_with_hmac")]
pub(crate) fn sign(payload: String, key: &str) -> BinanceResult<String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    let mut mac = Hmac::<Sha256>::new_from_slice(key.to_string().as_bytes()).unwrap();
    mac.update(payload.to_string().as_bytes());
    let result = mac.finalize();
    Ok(format!("{:x}", result.into_bytes()))
}

#[cfg(not(feature = "sign_with_hmac"))]
pub(crate) fn sign(payload: String, key: &str) -> BinanceResult<String> {

    use base64::{engine::general_purpose, Engine as _};
    use ed25519_dalek::pkcs8::DecodePrivateKey;
    use ed25519_dalek::SigningKey;
    use ed25519_dalek::{Signature as Ed25519Signature, Signer};

    let private_key = SigningKey::from_pkcs8_pem(key)?;
    let signature: Ed25519Signature = private_key.sign(payload.as_bytes());
    Ok(general_purpose::STANDARD.encode(signature.to_bytes()))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign() {
        let res = sign("symbol=BTCUSDT&recvWindow=5000".to_string(), "");
        println!("{}", res.unwrap());
    }
}