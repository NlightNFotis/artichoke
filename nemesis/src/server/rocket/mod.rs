//! Nemesis server based on [Rocket](rocket).

use crate::server::Builder;
use crate::Error;

pub mod request;
pub mod routes;

pub fn launcher(builder: Builder) -> Result<(), Error> {
    let mut launcher = rocket::ignite();
    for (_path, mount) in &builder.mounts.0 {
        launcher = launcher.mount(
            mount.path.as_str(),
            routes![routes::app_get, routes::app_get_root],
        );
    }
    launcher = launcher.manage(builder.mounts);
    for (path, _asset) in &builder.assets.0 {
        launcher = launcher.mount(path.as_str(), routes![routes::static_asset]);
    }
    launcher = launcher.manage(builder.assets);
    for (path, _asset) in &builder.html.0 {
        launcher = launcher.mount(path.as_str(), routes![routes::html_asset]);
    }
    launcher = launcher.manage(builder.html);
    let err = launcher.launch();
    // This log is only reachable if Rocket has an error during startup,
    // otherwise `rocket::ignite().launch()` blocks forever.
    error!("Failed to launch rocket: {}", err);
    Err(Error::FailedLaunch(err.to_string()))
}
