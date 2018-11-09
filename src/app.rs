// app.rs
use actix::prelude::*;
use actix_web::{http::Method, middleware::Logger, ws, App, Error, HttpRequest, HttpResponse};

use camera_group_routes::{
    create_camera_group, fetch_all_camera_groups, fetch_camera_group, update_camera_group,
};
use camera_routes::{create_camera, fetch_all_cameras, fetch_camera, update_camera};
use models::DbExecutor;
use video_unit_routes::{fetch_video_unit, fetch_video_units_between};
use ws_session::WsSession;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn ws_route(req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, WsSession::default())
}

// helper function to create and returns the app after mounting all routes/resources
pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { db })
        // setup builtin logger to get nice logging for each request
        .middleware(Logger::default())
        .resource("/ws", |r| r.route().f(ws_route))
        // routes for authentication
        .resource("/auth", |_r| {})
        // routes to camera_group
        .resource("/v1/camera_group", |r| {
            r.method(Method::POST).with(create_camera_group);
            r.method(Method::GET).with(fetch_all_camera_groups);
        }).resource("/v1/camera_group/{id}", |r| {
            r.method(Method::POST).with(update_camera_group);
            r.method(Method::GET).with(fetch_camera_group);
        })
        // routes to camera
        .resource("/v1/camera", |r| {
            r.method(Method::POST).with(create_camera);
            r.method(Method::GET).with(fetch_all_cameras);
        }).resource("/v1/camera/{id}", |r| {
            r.method(Method::POST).with(update_camera);
            r.method(Method::GET).with(fetch_camera);
        })
        // routes to video_unit
        .resource("/v1/video_unit/{id}", |r| {
            r.method(Method::GET).with(fetch_video_unit);
        }).resource("/v1/video_unit/between/{begin}/{end}", |r| {
            r.method(Method::GET).with(fetch_video_units_between);
        })
}
