use jsonwebtoken::{EncodingKey, DecodingKey};

pub static API: &str = "api";

pub enum Roles{
  Admin=1,
  Ops=2,
  Marketing=3,
}
impl PartialEq for Roles{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

pub fn get_env_key()->String{
  std::env::var("TOKEN_KEY")
      .unwrap_or("jaajkfsklñe".to_owned())
}

pub fn encoding_key()->EncodingKey{
  EncodingKey::from_secret(get_env_key().as_bytes())
}
pub fn decoding_key()->DecodingKey{
  DecodingKey::from_secret(get_env_key().as_bytes())
}