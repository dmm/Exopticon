//! Exopticon is a free video surveillance system

// to avoid the warning from diesel macros
#![allow(proc_macro_derive_resolution_fallback)]
#![deny(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::integer_arithmetic)]
#![allow(clippy::missing_inline_in_public_items)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::implicit_return)]

/// Exopticon is a free video surveillance system
extern crate actix;
extern crate actix_web;
extern crate askama;
extern crate base64;
#[macro_use]
extern crate base64_serde;
extern crate bytes;
extern crate chrono;
extern crate dotenv;
extern crate env_logger;
extern crate futures;
extern crate r2d2;
extern crate serde;
extern crate uuid;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate mime_guess;
extern crate rand;
extern crate rmp;
extern crate rmp_serde;
#[macro_use]
extern crate rust_embed;
extern crate serde_bytes;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tokio_process;

/// Actix route specification
mod app;

/// Implements authentication logic
mod auth_handler;

/// Implements auth routes
mod auth_routes;

/// Implements analysis actor
mod analysis_actor;

/// Implements analysis routes
mod analysis_routes;

/// Implements analysis supervisor
mod analysis_supervisor;

/// implements camera group api logic
mod camera_group_handler;

/// Implements camera group routes
mod camera_group_routes;

/// Implements camera api logic
mod camera_handler;

/// Implements camera api routes
mod camera_routes;

/// Actor that captures video from a camera
mod capture_actor;

/// Actor that supervises capture actors
mod capture_supervisor;

/// Error type
mod errors;

/// Actor that deletes excess files for a camera group
mod file_deletion_actor;

/// Actor that supervises files deletion workers
mod file_deletion_supervisor;

/// Actor message structs
mod models;

/// Implements `DbExecutor` handler for creating users
mod register_handler;

/// Root supervisor, lauches `CaptureSupervisor` and `DeletionSupervisor`
mod root_supervisor;

/// Struct map writer so rmps-serde will output maps
mod struct_map_writer;

/// Database schema, generated by diesel
mod schema;

/// Routes for handling static files
mod static_routes;

/// Routes for handling users
mod user_routes;

/// Utility functions
mod utils;

/// Implements handlers for `DbExecutor` concerning `VideoFile`s
mod video_file_handler;

/// Implements handlers for `DbExecutor` concerning `VideoUnit`s
mod video_unit_handler;

/// Implements routes for video units
mod video_unit_routes;

/// Implements camera frame pub/sub
mod ws_camera_server;

/// Implements a websocket session
mod ws_session;

use crate::models::DbExecutor;
use actix::prelude::*;
use actix_web::server;
use base64::encode;
use dialoguer::{Input, PasswordInput};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use rand::Rng;
use std::env;

use crate::models::{CreateCameraGroup, CreateUser};
use crate::root_supervisor::{ExopticonMode, RootSupervisor};

/// Interactively prompts the operator and adds a new user with the
/// details provided. This is for bootstrapping users on a new
/// install. It should be run before the main system is started.
///
/// # Arguments
///
/// * `sys` - The actix system runner
/// * `address` - The address of the `DbExecutor`
///
fn add_user(
    sys: &mut actix::SystemRunner,
    address: &Addr<DbExecutor>,
) -> Result<bool, std::io::Error> {
    let username = Input::new()
        .with_prompt("Enter username for initial user")
        .interact()?;

    let password = PasswordInput::new()
        .with_prompt("Enter password for initial user")
        .interact()?;

    let fut2 = address.send(CreateUser {
        username,
        password,
        timezone: String::from("UTC"),
    });

    match sys.block_on(fut2) {
        Ok(_) => (),
        Err(err) => {
            error!("Error creating user! {}", err);
        }
    }
    info!("Created user!");
    Ok(true)
}

/// Adds a camera group. This is really only for setting up initial
/// camera groups for boostrapping.. It should be run before the full
/// system is started.
///
/// # Arguments
///
/// * `sys` - The actix system runner
/// * `address` - The address of the `DbExecutor`
///
fn add_camera_group(
    sys: &mut actix::SystemRunner,
    address: &Addr<DbExecutor>,
) -> Result<bool, std::io::Error> {
    let storage_path = Input::new()
        .with_prompt("Enter storage path for recorded video")
        .interact()?;

    let max_storage_size: i64 = Input::new()
        .with_prompt("Enter max space used at this path, in megabytes")
        .interact()?;

    let fut = address.send(CreateCameraGroup {
        name: String::from("default"),
        storage_path,
        max_storage_size,
    });
    match sys.block_on(fut) {
        Ok(_) => (),
        Err(err) => {
            error!("Error creating camera group! {}", err);
        }
    }
    info!("Created camera group!");
    Ok(true)
}

fn main() {
    env_logger::init();

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut sys = actix::System::new("Exopticon");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let address: Addr<DbExecutor> = SyncArbiter::start(4, move || DbExecutor(pool.clone()));

    let db_address = address.clone();
    let setup_address = address.clone();
    // secret is a random 32 character long base 64 string
    let secret: String =
        env::var("SECRET_KEY").unwrap_or_else(|_| encode(&rand::thread_rng().gen::<[u8; 24]>()));

    server::new(move || app::new(address.clone(), &secret))
        .bind("0.0.0.0:3000")
        .expect("Can not bind to '0.0.0.0:3000'")
        .start();

    let mut mode = ExopticonMode::Run;
    let mut add_user_flag = false;
    let mut add_camera_group_flag = false;
    // Prints each argument on a separate line
    for argument in env::args() {
        match argument.as_ref() {
            "--standby" => {
                info!("Runtime mode is standby...");
                mode = ExopticonMode::Standby;
            }
            "--add-user" => {
                add_user_flag = true;
            }
            "--add-camera-group" => {
                add_camera_group_flag = true;
            }
            _ => (),
        }
    }

    let root_supervisor = RootSupervisor::new(mode, db_address);

    root_supervisor.start();

    if add_user_flag && add_user(&mut sys, &setup_address).is_err() {
        error!("Error creating user!");
        return;
    }

    if add_camera_group_flag && add_camera_group(&mut sys, &setup_address).is_err() {
        error!("Error creating camera group!");
        return;
    }

    sys.run();
}
