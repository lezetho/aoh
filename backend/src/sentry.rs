use sentry::ClientInitGuard;

pub fn init() -> ClientInitGuard {
    let mut opts = sentry::ClientOptions::new().maybe_release(sentry::release_name!());
    if let Ok(dsn) = std::env::var("SENTRY_DSN") {
        opts.dsn = dsn.parse().ok();
    }
    sentry::init(opts)
}
