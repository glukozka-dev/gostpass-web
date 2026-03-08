use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding};
use anyhow::Result;
const  RSA_KEY_BITS:usize = 4096;
const TOKEN: &str = "1234";
const ADM_TOKEN: &str = "4321";

pub  fn create_user_keys() -> Result<[String; 2]> {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, RSA_KEY_BITS)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    let str_pub = pub_key.to_public_key_pem(LineEnding::LF)?;
    let str_priv = priv_key.to_pkcs8_pem(LineEnding::LF)?.to_string();
    Ok([str_pub, str_priv])
}

pub fn auth_check(auth_token: String) -> Result<(), Box<dyn std::error::Error>> {
    if auth_token == TOKEN {
        return  Ok(());
    }
    else {
        return  Err("Wrong token".into());
    }
}

pub fn adm_auth_check(auth_token: String) -> Result<(), Box<dyn std::error::Error>> {
    if auth_token == ADM_TOKEN {
        return  Ok(());
    }
    else {
        return  Err("Wrong token".into());
    }
}