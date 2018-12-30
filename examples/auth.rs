#[macro_use]
extern crate serde_derive;

use envy;
use strava_api::auth::{self, RefreshToken};
use strava_api::config::Config;

#[derive(Debug, Deserialize)]
struct Env {
  refresh_token: String,
}

/// Set the `STRAVA_REFRESH_TOKEN` environment variable to your access token
/// as well as `STRAVA_CLIENT_ID` and `STRAVA_CLIENT_SECRET` for configuration
/// before running with `cargo run --example auth`
fn main() {
  let env = envy::prefixed("STRAVA_").from_env::<Env>().unwrap();
  let config = Config::from_env().unwrap();
  let refresh_token = RefreshToken::from(&env.refresh_token);
  let login = auth::refresh_token(&refresh_token, &config).unwrap();
  println!("{:#?}", login);
  println!("Is expired:       {}", login.is_expired());
  println!("Will expire soon: {}", login.will_expire_soon(None));
}
