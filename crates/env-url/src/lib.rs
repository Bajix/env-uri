//!
//! This crate provides env-composable service urls complete with key overrides as to facilitate flexibility and to simplify integration with kubernetes.
//!
//! ## Env mapping behaviors
//!
//! | ENV                     |                                                           |
//! | -----------------------:|:---------------------------------------------------------:|
//! | {PREFIX}_URL            | set service url, disregarding other overrides             |
//! | {PREFIX}_URL_ENV        | override `{PREFIX}_URL` env mapping                       |
//! | {PREFIX}_SCHEME         | set url scheme component                                  |
//! | {PREFIX}_SCHEME_ENV     | override `{PREFIX}_SCHEME` env mapping                    |
//! | {PREFIX}_PATH           | set url path component                                    |
//! | {PREFIX}_PATH_ENV       | override `{PREFIX}_PATH` env mapping                      |
//! | {PREFIX}_QUERY          | set url query component                                   |
//! | {PREFIX}_QUERY_ENV      | override `{PREFIX}_QUERY` env mapping                     |
//! | {PREFIX}_USERNAME       | set url username component (if password set)              |
//! | {PREFIX}_USERNAME_ENV   | override `{PREFIX}_USERNAME` env mapping                  |
//! | {PREFIX}_PASSWORD       | set url password (username as "default" if empty)         |
//! | {PREFIX}_PASSWORD_ENV   | override `{PREFIX}_PASSWORD` env mapping                  |
//!
//! ## Example
//!
//! ```
//! use env_url::*;
//!
//! #[derive(EnvURL)]
//! #[env_url(env_prefix = "REDIS", default = "redis://127.0.0.1:6379")]
//! pub struct RedisDB;
//!
//! let service_url = RedisDB::service_url();
//!
//! ```
//!
#[doc(hidden)]
pub extern crate url;

#[doc(hidden)]
pub extern crate log;

extern crate self as env_url;

pub use derive_env_url::*;
pub use url::{ParseError, Url};
pub trait ServiceURL {
  fn service_url() -> Result<Url, ParseError>;
}

#[cfg(test)]
#[ctor::ctor]
fn setup_test_env() {
  env_logger::init();
  std::env::remove_var("REDIS_URL");
  std::env::remove_var("REDIS_URL_ENV");
  std::env::remove_var("REDIS_SCHEME");
  std::env::remove_var("REDIS_SCHEME_ENV");
  std::env::remove_var("REDIS_PATH");
  std::env::remove_var("REDIS_PATH_ENV");
  std::env::remove_var("REDIS_QUERY");
  std::env::remove_var("REDIS_QUERY_ENV");
  std::env::remove_var("REDIS_USERNAME");
  std::env::remove_var("REDIS_USERNAME_ENV");
  std::env::remove_var("REDIS_PASSWORD");
  std::env::remove_var("REDIS_PASSWORD_ENV");
}
#[cfg(test)]
mod tests {
  use env_url::*;

  #[derive(EnvURL)]
  #[env_url(env_prefix = "REDIS", default = "redis://127.0.0.1:6379")]
  struct RedisDB;

  #[test]
  fn it_creates_url() {
    let url = RedisDB::service_url().unwrap();

    assert_eq!(url.as_str(), "redis://127.0.0.1:6379");
  }
}
