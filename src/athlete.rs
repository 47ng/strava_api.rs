use crate::api::{Api, ApiResult, Context};

#[derive(Debug, Deserialize)]
pub enum Gender {
  #[serde(rename = "M")]
  Male,
  #[serde(rename = "F")]
  Female,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FriendshipStatus {
  Pending,
  Accepted,
  Blocked,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeasurementPreference {
  Feet,
  Meters,
}

#[derive(Debug, Deserialize)]
pub struct Athlete {
  /// The unique identifier of the athlete
  pub id: i32,

  /// The athlete's first name
  pub firstname: Option<String>,

  /// The athlete's last name
  pub lastname: Option<String>,

  /// URL to a 62x62 pixel profile picture
  pub profile_medium: Option<String>,

  /// URL to a 124x124 pixel profile picture
  pub profile: Option<String>,

  /// The athlete's city
  pub city: Option<String>,

  /// The athlete's state or geographical region
  pub state: Option<String>,

  /// The athlete's country
  pub country: Option<String>,

  /// The athlete's sex.
  /// May take one of the following values: "M", "F"
  pub sex: Option<Gender>,

  /// Whether the currently logged-in athlete follows this athlete.
  pub friend: Option<FriendshipStatus>,

  /// Whether this athlete follows the currently logged-in athlete.
  pub follower: Option<FriendshipStatus>,

  /// Whether the athlete has any Summit subscription
  pub summit: Option<bool>,

  /// The time at which the athlete was created
  pub created_at: Option<String>,

  /// The time at which the athlete was last updated
  pub updated_at: Option<String>,

  /// The number of other athletes who follow this athlete
  pub follower_count: Option<i32>,

  /// The number of other athletes that this athlete follows
  pub friend_count: Option<i32>,

  /// The number or athletes mutually followed by this athlete
  /// and the currently logged-in athlete
  pub mutual_friend_count: Option<i32>,

  /// The athlete's preferred unit system.
  /// May take one of the following values: `feet` or `meters`
  pub measurement_preference: Option<MeasurementPreference>,

  /// The athlete's email address
  /// (undocumented by Strava but may still be returned for some accounts)
  pub email: Option<String>,

  /// The athlete's FTP (Functional Threshold Power)
  pub ftp: Option<i32>,

  /// The athlete's weight
  pub weight: Option<f32>,
}

// --

/// Return the currently logged in athlete
///
/// Identity corresponds to the access token passed in Context.
pub fn current(api: &Api, context: &Context) -> ApiResult<Athlete> {
  api.get("/athlete", context)?.json()
}
