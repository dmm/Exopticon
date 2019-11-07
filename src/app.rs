// app.rs

use actix::Addr;
use actix_web::http::Method;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, ws, App, Error, HttpRequest, HttpResponse};

use chrono::Duration;

use crate::analysis_routes::create_analysis_engine;
use crate::auth_routes::{login, logout, AuthMiddleware, WebAuthMiddleware};
use crate::camera_group_routes::{
    create_camera_group, fetch_all_camera_groups, fetch_camera_group, update_camera_group,
};
use crate::camera_routes::{
    create_camera, discover, fetch_all_cameras, fetch_camera, fetch_ntp, fetch_time, ptz_direction,
    ptz_relative, set_ntp, set_time, update_camera,
};
use crate::models::DbExecutor;
use crate::observation_routes::fetch_observations_between;
use crate::static_routes;
use crate::static_routes::{fetch_static_file, index};
use crate::user_routes::create_user;
use crate::video_unit_routes::{fetch_video_unit, fetch_video_units_between};
use crate::ws_session::{WsSerialization, WsSession};

/// Struct representing main application state
pub struct RouteState {
    /// address of database actor
    pub db: Addr<DbExecutor>,
}

/// Route to return a websocket session using messagepack serialization
pub fn ws_route(req: &HttpRequest<RouteState>) -> Result<HttpResponse, Error> {
    debug!("Starting websocket session...");
    ws::start(req, WsSession::new(WsSerialization::MsgPack))
}

/// Route to return a websocket session using json serialization
pub fn ws_json_route(req: &HttpRequest<RouteState>) -> Result<HttpResponse, Error> {
    debug!("Starting json websocket session...");
    ws::start(req, WsSession::new(WsSerialization::Json))
}

/// helper function to create and returns the app after mounting all routes/resources
pub fn new(db: Addr<DbExecutor>, secret: &str) -> App<RouteState> {
    App::with_state(RouteState { db })
        // setup builtin logger to get nice logging for each request
        .middleware(Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("id")
                .path("/")
                //                .domain(domain.as_str())
                .max_age(Duration::days(7)) // just for testing
                .secure(false),
        ))
        .resource("/login", |r| {
            r.method(Method::GET).with(static_routes::login);
        })
        // routes for authentication
        .resource("/auth", |r| {
            r.method(Method::POST).with(login);
            r.method(Method::DELETE).with(logout);
        })
        // routes for static files
        .default_resource(|r| {
            r.middleware(WebAuthMiddleware);
            r.method(Method::GET).with(index);
        })
        .resource("/{script}.js", |r| {
            r.middleware(WebAuthMiddleware);
            r.method(Method::GET).with(static_routes::get_js_file);
        })
        .resource("/{scriptmap}.js.map", |r| {
            r.middleware(WebAuthMiddleware);
            r.method(Method::GET).with(static_routes::get_js_map_file);
        })
        .resource("/{stylesheet}.css", |r| {
            r.middleware(WebAuthMiddleware);
            r.method(Method::GET).with(static_routes::get_css_file);
        })
        .scope("/static", |s| {
            s.middleware(WebAuthMiddleware)
                .handler("/", fetch_static_file)
        })
        // v1 api scope
        .scope("/v1", |s| {
            s.middleware(AuthMiddleware)
                .resource("/ws", |r| r.route().f(ws_route))
                .resource("/ws_json", |r| r.route().f(ws_json_route))
                // routes to camera_group
                .resource("/camera_groups", |r| {
                    r.method(Method::POST).with(create_camera_group);
                    r.method(Method::GET).with(fetch_all_camera_groups);
                })
                .resource("/camera_groups/{id}", |r| {
                    r.method(Method::POST).with(update_camera_group);
                    r.method(Method::GET).with(fetch_camera_group);
                })
                // routes to camera
                .resource("/cameras", |r| {
                    r.method(Method::POST).with(create_camera);
                    r.method(Method::GET).with(fetch_all_cameras);
                })
                .resource("/cameras/discover", |r| {
                    r.method(Method::GET).with(discover)
                })
                .resource("/cameras/{id}", |r| {
                    r.method(Method::POST).with(update_camera);
                    r.method(Method::GET).with(fetch_camera);
                })
                .resource("/cameras/{id}/time", |r| {
                    r.method(Method::GET).with(fetch_time);
                    r.method(Method::POST).with(set_time);
                })
                .resource("/cameras/{id}/ntp", |r| {
                    r.method(Method::GET).with(fetch_ntp);
                    r.method(Method::POST).with(set_ntp);
                })
                .resource("/cameras/{id}/ptz/relative", |r| {
                    r.method(Method::POST).with(ptz_relative)
                })
                .resource("/cameras/{id}/ptz/{direction}", |r| {
                    r.method(Method::POST).with(ptz_direction);
                })
                .resource("/cameras/{camera_id}/video", |r| {
                    r.method(Method::GET).with_async(fetch_video_units_between);
                })
                // routes to observations
                .resource("/cameras/{camera_id}/observations", |r| {
                    r.method(Method::GET).with_async(fetch_observations_between);
                })
                // routes to video_unit
                .resource("/video_units/{id}", |r| {
                    r.method(Method::GET).with(fetch_video_unit);
                })
                // routes to user
                .resource("/users", |r| {
                    r.method(Method::POST).with(create_user);
                    //            r.method(Method::GET).with(fetch_all_users);
                })
                // routes to analysis engine
                .resource("/analysis_engines", |r| {
                    r.method(Method::POST).with(create_analysis_engine)
                })
        })
}
