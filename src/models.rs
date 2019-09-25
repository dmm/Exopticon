// models.rs
use actix::{Actor, SyncContext};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// This is db executor actor. can be run in parallel
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

// Actors communicate exclusively by exchanging messages.
// The sending actor can optionally wait for the response.
// Actors are not referenced directly, but by means of addresses.
// Any rust type can be an actor, it only needs to implement the Actor trait.
impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

use crate::schema::{camera_groups, cameras, observations, users, video_files, video_units};

/// Full camera group model. Represents a full row returned from the
/// database.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "camera_groups"]
#[serde(rename_all = "camelCase")]
pub struct CameraGroup {
    /// camera group id
    pub id: i32,
    /// camera group name
    pub name: String,
    /// full path to video storage path, e.g. /mnt/video/8/
    pub storage_path: String,
    /// maximum allowed storage size in bytes
    pub max_storage_size: i64,
    /// insertion time
    pub inserted_at: NaiveDateTime,
    /// update time
    pub updated_at: NaiveDateTime,
}

/// Represents a camera group creation request
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "camera_groups"]
pub struct CreateCameraGroup {
    /// camera group name
    pub name: String,
    /// full path to camera group storage, e.g. /mnt/video/8
    pub storage_path: String,
    /// maximum allowed storage size in bytes
    pub max_storage_size: i64,
}

/// Represents a camera group update request
#[derive(AsChangeset, Debug, Deserialize, Identifiable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "camera_groups"]
pub struct UpdateCameraGroup {
    /// id of camera group to update
    pub id: i32,
    /// if provided, updated name for camera group
    pub name: Option<String>,
    /// if provided, updated storage path for camera group
    pub storage_path: Option<String>,
    /// if provided, updated storage size for camera group
    pub max_storage_size: Option<i64>,
}

/// Represents a request to fetch a camera group
pub struct FetchCameraGroup {
    /// id of camera group to fetch
    pub id: i32,
}

/// Represents a request to fetch all camera groups
pub struct FetchAllCameraGroup {}

/// Represents a request to fetch all cameras groups and associated
/// cameras
pub struct FetchAllCameraGroupAndCameras {}

/// Represents a camera group and its associated cameras
#[derive(Serialize)]
pub struct CameraGroupAndCameras(pub CameraGroup, pub Vec<Camera>);

/// Represents a request to fetch up to `count` files from the
/// specified camera group
pub struct FetchCameraGroupFiles {
    /// id of camera group to fetch associated files from
    pub camera_group_id: i32,
    /// maximum number of files to return
    pub count: i64,
}

/// Full camera model, represents database row
#[derive(
    Identifiable, PartialEq, Associations, Debug, Serialize, Deserialize, Queryable, Insertable,
)]
#[belongs_to(CameraGroup)]
#[serde(rename_all = "camelCase")]
#[table_name = "cameras"]
pub struct Camera {
    /// id of camera
    pub id: i32,
    /// id of associated camera group
    pub camera_group_id: i32,
    /// name of camera
    pub name: String,
    /// ip address associated with camera, e.g. 192.168.0.53
    pub ip: String,
    /// port used for ONVIF protocol
    pub onvif_port: i32,
    /// MAC address of camera, e.g. 9C-84-AE-0E-33-5A
    pub mac: String,
    /// username for ONVIF and RTSP authentication
    pub username: String,
    /// plaintext password for ONVIF and RTSP authentication
    pub password: String,
    /// url for rtsp stream
    pub rtsp_url: String,
    /// ptz type, either onvif or onvif_continuous
    pub ptz_type: String,
    /// ONVIF profile token for ptz
    pub ptz_profile_token: String,
    /// whether camera capture is enabled.
    pub enabled: bool,
    /// insertion time
    pub inserted_at: NaiveDateTime,
    /// update time
    pub updated_at: NaiveDateTime,
}

/// Represents a request to create a camera
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "cameras"]
pub struct CreateCamera {
    /// id of camera group to associate with new camera
    pub camera_group_id: i32,
    /// name of camera
    pub name: String,
    /// ip address associated with camera, e.g. 192.168.0.53
    pub ip: String,
    /// port used for ONVIF protocol
    pub onvif_port: i32,
    /// MAC address of camera, e.g. 9C-84-AE-0E-33-5A
    pub mac: String,
    /// username for ONVIF and RTSP authentication
    pub username: String,
    /// plaintext password for ONVIF and RTSP authentication
    pub password: String,
    /// url for rtsp stream
    pub rtsp_url: String,
    /// ptz type, either onvif or onvif_continuous
    pub ptz_type: String,
    /// ONVIF profile token for ptz
    pub ptz_profile_token: String,
    /// whether camera capture is enabled.
    pub enabled: bool,
}

/// Represents a request to update existing camera
#[derive(AsChangeset, Debug, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "cameras"]
pub struct UpdateCamera {
    /// id of camera to update
    pub id: i32,
    /// if present, new camera group id
    pub camera_group_id: Option<i32>,
    /// if present, new camera name
    pub name: Option<String>,
    /// if present, new ip address
    pub ip: Option<String>,
    /// if present, new onvif port
    pub onvif_port: Option<i32>,
    /// if present, new MAC address
    pub mac: Option<String>,
    /// if present, new username for ONVIF and RTSP streaming
    pub username: Option<String>,
    /// if present, new plaintext password of ONVIF and RTSP streaming
    pub password: Option<String>,
    /// if present, new rtsp_url
    pub rtsp_url: Option<String>,
    /// if present, new ptz type
    pub ptz_type: Option<String>,
    /// if present, new ONVIF ptz profile token
    pub ptz_profile_token: Option<String>,
    /// if present, updates enabled status
    pub enabled: Option<bool>,
}

/// Represents a request to fetch a camera from the database
pub struct FetchCamera {
    /// id of desired camera record
    pub id: i32,
}

/// Represents a request to fetch all camera records from database
pub struct FetchAllCamera {}

/// Represents the results of a video unit api fetch.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputVideoUnit {
    /// id of video unit
    pub id: i32,
    /// id of associated camera
    pub camera_id: i32,
    /// monotonic index of video unit
    pub monotonic_index: i32,
    /// begin time in UTC
    pub begin_time: NaiveDateTime,
    /// end time in UTC
    pub end_time: NaiveDateTime,
    /// video files associated with this video unit
    pub files: Vec<VideoFile>,
    /// insertion time
    pub inserted_at: NaiveDateTime,
    /// update time
    pub updated_at: NaiveDateTime,
}

/// Full video unit model, represents entire database row
#[derive(Identifiable, Associations, Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
#[belongs_to(Camera)]
#[table_name = "video_units"]
pub struct VideoUnit {
    /// id of video unit
    pub id: i32,
    /// id of associated camera
    pub camera_id: i32,
    /// monotonic index
    pub monotonic_index: i32,
    /// begin time in UTC
    pub begin_time: NaiveDateTime,
    /// end time in UTC
    pub end_time: NaiveDateTime,
    /// insertion time
    pub inserted_at: NaiveDateTime,
    /// update time
    pub updated_at: NaiveDateTime,
}

/// Represents request to create new video unit record
#[derive(AsChangeset, Debug, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "video_units"]
pub struct CreateVideoUnit {
    /// id of associated camera
    pub camera_id: i32,
    /// monotonic index
    pub monotonic_index: i32,
    /// begin time in UTC
    pub begin_time: NaiveDateTime,
    /// end time in UTC
    pub end_time: NaiveDateTime,
}

/// Represents request to update video unit record
#[derive(AsChangeset, Debug, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "video_units"]
pub struct UpdateVideoUnit {
    /// id of video unit to update
    pub id: i32,
    /// if present, new associated camera id
    pub camera_id: Option<i32>,
    /// if present, new monotonic index
    pub monotonic_index: Option<i32>,
    /// if present, new begin time, in UTC
    pub begin_time: Option<NaiveDateTime>,
    /// if present, new end time, in UTC
    pub end_time: Option<NaiveDateTime>,
}

/// Represents a request to fetch a specified video unit
pub struct FetchVideoUnit {
    /// id of video unit to fetch
    pub id: i32,
}

/// Represents a request to fetch video units between specified times
pub struct FetchBetweenVideoUnit {
    /// id of camera to fetch video for
    pub camera_id: i32,
    /// in UTC
    pub begin_time: DateTime<Utc>,
    /// in UTC
    pub end_time: DateTime<Utc>,
}

/// Full video file model, represents full database row
#[derive(Queryable, Associations, Identifiable, Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "video_files"]
#[belongs_to(VideoUnit)]
pub struct VideoFile {
    /// id of video file
    pub id: i32,
    /// id of associated video unit
    pub video_unit_id: i32,
    /// filename of video file
    pub filename: String,
    /// size in bytes of video file
    pub size: i32,
    /// insertion time
    pub inserted_at: NaiveDateTime,
    /// update time
    pub updated_at: NaiveDateTime,
}

/// Represents request to create new video file
#[derive(AsChangeset, Debug, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "video_files"]
pub struct CreateVideoFile {
    /// id of video unit to own this video file
    pub video_unit_id: i32,
    /// filename for new video file
    pub filename: String,
    /// size in bytes of new video file
    pub size: i32,
}

/// Represents request to update video file
#[derive(AsChangeset, Debug, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "video_files"]
pub struct UpdateVideoFile {
    /// id of video file to update
    pub id: i32,
    /// if present, new id of associated video unit
    pub video_unit_id: Option<i32>,
    /// if present, new filename
    pub filename: Option<String>,
    /// if present, new file size
    pub size: Option<i32>,
}

/// Represents a request to create a new video unit and file pair
pub struct CreateVideoUnitFile {
    /// id of camera associated with new video unit and file
    pub camera_id: i32,
    /// monotonic index
    pub monotonic_index: i32,
    /// begin time, in UTC
    pub begin_time: NaiveDateTime,
    /// video file filename
    pub filename: String,
}

/// Represents request to update a video unit and video file pair
pub struct UpdateVideoUnitFile {
    /// id of video unit
    pub video_unit_id: i32,
    /// end time, in UTC
    pub end_time: NaiveDateTime,
    /// id of video file
    pub video_file_id: i32,
    /// video file size in bytes
    pub size: i32,
}

/// Represents request to fetch oldest video unit/video file pairs
pub struct FetchOldVideoUnitFile {
    /// id of camera unit to fetch from
    pub camera_group_id: i32,
    /// number of video unit/video file pairs to fetch
    pub count: i64,
}

/// Represents request to delete video units
pub struct DeleteVideoUnitFiles {
    /// vec of ids of video units to delete
    pub video_unit_ids: Vec<i32>,
    /// vec of id of video files to delete
    pub video_file_ids: Vec<i32>,
}

/// Represents a request to fetch empty video files, video files
/// without a size specified.
pub struct FetchEmptyVideoFile;

/// Represents an observation derived from a frame of video
#[derive(Queryable, Associations, Identifiable, Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "observations"]
#[belongs_to(VideoUnit)]
pub struct Observation {
    /// id of Observation
    pub id: i64,
    /// id of owning video unit
    pub video_unit_id: i32,
    /// offset from beginning of video unit, starts at 0
    pub frame_offset: i32,
    /// Identifies the type of observation, eg Person, Motion, Deer
    pub tag: String,
    /// Details associated with observation, eg John, Male, whatever
    pub details: String,
    /// A value between 0-100 representing the percentage certainty of
    /// the observation.
    pub score: i16,
    /// upper-left x coordinate
    pub ul_x: i16,
    /// upper-left y coordinate
    pub ul_y: i16,
    /// lower-right x coordinate
    pub lr_x: i16,
    /// lower-right y coordinate
    pub lr_y: i16,
    /// Time that observation record was inserted
    pub inserted_at: DateTime<Utc>,
}

/// Represents a request to create a single observation.
#[derive(AsChangeset, Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "observations"]
pub struct CreateObservation {
    /// id of owning video unit
    pub video_unit_id: i32,
    /// offset from beginning of video unit, starts at 0
    pub frame_offset: i32,
    /// Identifies the type of observation, eg Person, Motion, Deer
    pub tag: String,
    /// Details associated with observation, eg John, Male, whatever
    pub details: String,
    /// A value between 0-100 representing the percentage certainty of
    /// the observation.
    pub score: i16,
    /// upper-left x coordinate
    pub ul_x: i16,
    /// upper-left y coordinate
    pub ul_y: i16,
    /// lower-right x coordinate
    pub lr_x: i16,
    /// lower-right y coordinate
    pub lr_y: i16,
}

/// Represents a request to create one or more observation records.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateObservations {
    /// Vec of observations to create
    pub observations: Vec<CreateObservation>,
}

/// Represents a request to query `Observations`
#[derive(Debug, Serialize, Deserialize)]
pub struct FetchObservations {
    /// camera id to fetch observations for
    pub camera_id: i32,
    /// beginning time - inclusive
    pub begin_time: DateTime<Utc>,
    /// end time - exclusive
    pub end_time: DateTime<Utc>,
}

/// Full user model struct, represents full value from database.
#[derive(Queryable, Associations, Identifiable, Serialize)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct User {
    /// user id
    pub id: i32,
    ///  username
    pub username: String,
    /// hashed password
    pub password: String,
    /// Olson timezone, e.g. America/Chicago
    pub timezone: String,
    /// insertion date time
    pub inserted_at: NaiveDateTime,
    /// modified date time
    pub updated_at: NaiveDateTime,
}

/// User model without password. This is used as a return value for
/// user operations.
#[derive(Serialize)]
pub struct SlimUser {
    /// User id
    pub id: i32,
    /// username
    pub username: String,
    /// Olson database timezone, e.g. America/Chicago
    pub timezone: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            timezone: user.timezone,
        }
    }
}

/// Create new user message
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "users"]
pub struct CreateUser {
    /// username
    pub username: String,
    /// plaintext password
    pub password: String,
    /// Olson database timezone, e.g. America/Chicago
    pub timezone: String,
}
