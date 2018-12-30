use reqwest::Error as HttpError;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::string::ToString;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::Config;

// --

pub type AuthResult<T> = Result<T, HttpError>;

// --

#[derive(Debug, Deserialize)]
pub struct AccessToken(String);

impl From<&'static str> for AccessToken {
  fn from(token: &'static str) -> AccessToken {
    AccessToken(token.to_string())
  }
}

impl From<&String> for AccessToken {
  fn from(token: &String) -> AccessToken {
    AccessToken(token.clone())
  }
}

impl PartialEq<str> for AccessToken {
  fn eq(&self, other: &str) -> bool {
    self.0 == other
  }
}

impl Display for AccessToken {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

// --

#[derive(Debug, Deserialize)]
pub struct RefreshToken(String);

impl From<&'static str> for RefreshToken {
  fn from(token: &'static str) -> RefreshToken {
    RefreshToken(token.to_string())
  }
}

impl From<&String> for RefreshToken {
  fn from(token: &String) -> RefreshToken {
    RefreshToken(token.clone())
  }
}

impl PartialEq<str> for RefreshToken {
  fn eq(&self, other: &str) -> bool {
    self.0 == other
  }
}

// --

#[derive(Debug, Deserialize)]
pub struct Login {
  pub access_token: AccessToken,
  pub refresh_token: RefreshToken,
  expires_at: u64,
}

impl Login {
  /// Checks if the access token has expired
  pub fn is_expired(&self) -> bool {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    self.expires_at < now
  }

  /// Checks if the access token expires within the given timeout
  pub fn will_expire_soon(&self, timeout: Option<Duration>) -> bool {
    let default_timeout = Duration::new(10 * 60, 0); // 10 minutes
    let timeout = timeout.or(Some(default_timeout)).unwrap();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let eat = Duration::from_secs(self.expires_at);
    eat < (now + timeout)
  }
}

// --

/// Log into the application using the code returned from OAuth flow
pub fn login(code: &String, config: &Config) -> AuthResult<Login> {
  let client = reqwest::Client::new();
  let mut body = HashMap::new();
  let client_id = config.client_id.to_string();
  body.insert("client_id", client_id.as_str());
  body.insert("client_secret", config.client_secret.as_str());
  body.insert("code", code.as_str());
  body.insert("grant_type", "authorization_code");
  let mut res = client
    .post("https://www.strava.com/oauth/token")
    .json(&body)
    .send()?;
  Ok(res.json()?)
}

/// Recreate an access token from a refresh token
pub fn refresh_token(token: &RefreshToken, config: &Config) -> AuthResult<Login> {
  let client = reqwest::Client::new();
  let mut body = HashMap::new();
  let client_id = config.client_id.to_string();
  body.insert("client_id", client_id.as_str());
  body.insert("client_secret", config.client_secret.as_str());
  body.insert("refresh_token", token.0.as_str());
  body.insert("grant_type", "refresh_token");
  let mut res = client
    .post("https://www.strava.com/oauth/token")
    .json(&body)
    .send()?;
  Ok(res.json()?)
}

/// Revoke access to the application for the authenticated user
pub fn deauthorize(token: &AccessToken) -> AuthResult<()> {
  let client = reqwest::Client::new();
  let mut body = HashMap::new();
  body.insert("access_token", token.0.as_str());
  let res = client
    .post("https://www.strava.com/oauth/deauthorize")
    .json(&body)
    .send();
  match res {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  fn make_login(expires_at: u64) -> Login {
    Login {
      access_token: AccessToken::from("foo"),
      refresh_token: RefreshToken::from("bar"),
      expires_at,
    }
  }

  #[test]
  fn valid_login_is_not_expired() {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let valid_login = make_login(now + 60);
    assert!(!valid_login.is_expired());
  }

  #[test]
  fn old_login_is_expired() {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let login = make_login(now - 60);
    assert!(login.is_expired());
  }

  #[test]
  fn will_expire_soon_default() {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let over_10_minutes = make_login(now + 12 * 60);
    let under_10_minutes = make_login(now + 8 * 60);
    assert!(!over_10_minutes.will_expire_soon(None));
    assert!(under_10_minutes.will_expire_soon(None));
  }

  #[test]
  fn will_expire_soon_custom_timeout() {
    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_secs();
    let over_30_minutes = make_login(now + 32 * 60);
    let under_30_minutes = make_login(now + 28 * 60);
    let timeout = Duration::from_secs(30 * 60);
    assert!(!over_30_minutes.will_expire_soon(Some(timeout)));
    assert!(under_30_minutes.will_expire_soon(Some(timeout)));
  }

  #[test]
  fn deserialize_login() {
    let json = r#"{
      "access_token":   "foobar",
      "refresh_token":  "eggspam",
      "expires_at":     1234567890
    }"#;
    let login: Login = serde_json::from_str(&json).unwrap();
    assert_eq!(&login.access_token, "foobar");
    assert_eq!(&login.refresh_token, "eggspam");
    assert_eq!(login.expires_at, 1234567890);
  }
}
