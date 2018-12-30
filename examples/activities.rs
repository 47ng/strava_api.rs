#[macro_use]
extern crate serde_derive;

use envy;
use strava_api::api::{Api, Context};
use strava_api::auth::AccessToken;
use strava_api::activities;

#[derive(Debug, Deserialize)]
struct Env {
  access_token: String
}

/// Set the `STRAVA_ACCESS_TOKEN` environment variable to your access token
/// before running with `cargo run --example activities`
fn main() {
  let env = envy::prefixed("STRAVA_").from_env::<Env>().unwrap();
  let context = Context { access_token: AccessToken::from(&env.access_token) };
  let api = Api::new("https://www.strava.com/api/v3");
  let my_recent_activities = activities::latest(&api, &context);
  println!("{:#?}", my_recent_activities);
}
