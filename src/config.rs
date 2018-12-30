extern crate envy;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub client_id: u64,
  pub client_secret: String,
}

impl Config {
  /// Create a Config object from environment variables
  ///
  /// Requires the following environment variables to be set:
  ///   STRAVA_CLIENT_ID      (number)
  ///   STRAVA_CLIENT_SECRET  (string)
  pub fn from_env() -> Result<Config, envy::Error> {
    envy::prefixed("STRAVA_").from_env::<Config>()
  }
}
