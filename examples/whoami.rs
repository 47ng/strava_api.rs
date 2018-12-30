#[macro_use]
extern crate serde_derive;

use envy;
use strava_api::api::{Api, Context};
use strava_api::athlete;
use strava_api::auth::AccessToken;

#[derive(Debug, Deserialize)]
struct Env {
  access_token: String,
}

/// Set the `STRAVA_ACCESS_TOKEN` environment variable to your access token
/// before running with `cargo run --example whoami`
fn main() {
  let env = envy::prefixed("STRAVA_").from_env::<Env>().unwrap();
  let context = Context {
    access_token: AccessToken::from(env.access_token),
  };
  let api = Api::new("https://www.strava.com/api/v3");
  let myself = athlete::current(&api, &context).unwrap();
  println!("{:#?}", myself);
}
