use crate::api::{Api, ApiResult, Context, Pagination};

type DateTime = String; // ISO-8601
type LatLng = (f32, f32); // (latitude, longitude)

/// Activity Types
#[derive(Debug, Deserialize)]
pub enum ActivityType {
  Ride,
  Run,
  Swim,
  Hike,
  Walk,
  AlpineSki,
  BackcountrySki,
  Canoeing,
  Crossfit,
  EBikeRide,
  Elliptical,
  IceSkate,
  InlineSkate,
  Kayaking,
  Kitesurf,
  NordicSki,
  RockClimbing,
  RollerSki,
  Rowing,
  Snowboard,
  Snowshoe,
  StairStepper,
  StandUpPaddling,
  Surfing,
  WeightTraining,
  Windsurf,
  Workout,
  Yoga,
  Unknown,
}

/// All polylines are encoded with the following algorithm:
/// https://developers.google.com/maps/documentation/utilities/polylinealgorithm
#[derive(Debug, Default, Deserialize)]
pub struct PolylineMap {
  /// The identifier of the map
  pub id: String,

  /// The polyline of the map
  pub polyline: Option<String>,

  /// The summary polyline of the map
  pub summary_polyline: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct Activity {
  /// The unique identifier of the activity
  pub id: u64,

  /// The identifier of the upload that resulted in this activity
  pub upload_id: u64,

  /// The type of the activity
  #[serde(rename = "type")]
  pub activity_type: Option<ActivityType>,

  /// The name of the activity
  pub name: Option<String>,

  /// The activity's distance, in meters
  pub distance: Option<f32>,

  /// The activity's moving time, in seconds
  pub moving_time: Option<u32>,

  /// The activity's elapsed time, in seconds
  pub elapsed_time: Option<u32>,

  /// The activity's total elevation gain.
  pub total_elevation_gain: Option<f32>,

  /// The activity's highest elevation, in meters
  pub elev_high: Option<f32>,

  /// The activity's lowest elevation, in meters
  pub elev_low: Option<f32>,

  /// The time at which the activity was started.
  pub start_date: Option<DateTime>,

  /// The time at which the activity was started in the local timezone.
  pub start_date_local: Option<DateTime>,

  /// The timezone of the activity
  pub timezone: Option<String>,

  /// An instance of LatLng.
  pub start_latlng: Option<LatLng>,

  /// An instance of LatLng.
  pub end_latlng: Option<LatLng>,

  /// The number of achievements gained during this activity
  pub achievement_count: Option<u32>,

  /// The number of kudos given for this activity
  pub kudos_count: Option<u32>,

  /// The number of comments for this activity
  pub comment_count: Option<u32>,

  /// The number of athletes for taking part in a group activity
  pub athlete_count: Option<u32>,

  /// The number of Instagram photos for this activity
  pub photo_count: Option<u32>,

  /// The number of Instagram and Strava photos for this activity
  pub total_photo_count: Option<u32>,

  /// An instance of PolylineMap.
  pub map: Option<PolylineMap>,

  /// Whether this activity was recorded on a training machine
  pub trainer: Option<bool>,

  /// Whether this activity is a commute
  pub commute: Option<bool>,

  /// Whether this activity was created manually
  pub manual: Option<bool>,

  /// Whether this activity is private
  pub private: Option<bool>,

  /// Whether this activity is flagged
  pub flagged: Option<bool>,

  /// The activity's workout type
  pub workout_type: Option<u32>,

  /// The activity's average speed, in meters per second
  pub average_speed: Option<f32>,

  /// The activity's max speed, in meters per second
  pub max_speed: Option<f32>,

  /// Whether the logged-in athlete has kudoed this activity
  pub has_kudoed: Option<bool>,

  /// The id of the gear for the activity
  pub gear_id: Option<String>,

  /// The total work done in kilojoules during this activity. Rides only
  pub kilojoules: Option<f32>,

  /// Average power output in watts during this activity. Rides only
  pub average_watts: Option<f32>,

  /// Whether the watts are from a power meter, false if estimated
  pub device_watts: Option<bool>,

  /// Rides with power meter data only
  pub max_watts: Option<u32>,

  /// Similar to Normalized Power. Rides with power meter data only
  pub weighted_average_watts: Option<u32>,
}

// --

/// Get the latest activities for the logged in athlete
pub fn latest(api: &Api, context: &Context) -> ApiResult<Vec<Activity>> {
  api.get("/athlete/activities", context)?.json()
}

/// Get some activities that occurred prior to a given timestamp
///
/// Timestamp is given as seconds since epoch.
pub fn before(time: u64, api: &Api, context: &Context) -> ApiResult<Vec<Activity>> {
  let pagination = Pagination {
    before: time,
    ..Default::default()
  };
  api
    .get_paginated("/athlete/activities", context, &pagination)?
    .json()
}

/// Get some activities that occurred after a given timestamp
///
/// Timestamp is given as seconds since epoch.
pub fn after(time: u64, api: &Api, context: &Context) -> ApiResult<Vec<Activity>> {
  let pagination = Pagination {
    after: time,
    ..Default::default()
  };
  api
    .get_paginated("/athlete/activities", context, &pagination)?
    .json()
}

// --

/// Find a specific activity by its ID
pub fn by_id(id: u64, api: &Api, context: &Context) -> ApiResult<Activity> {
  let path = format!("/activities/{}", id);
  api.get(&path, context)?.json()
}
