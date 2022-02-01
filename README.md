# ENV Url

![License](https://img.shields.io/badge/license-MIT-green.svg)
[![Cargo](https://img.shields.io/crates/v/env-url.svg)](https://crates.io/crates/env-url)
[![Documentation](https://docs.rs/env-url/badge.svg)](https://docs.rs/env-url)

Env-composable service urls complete with key overrides as to facilitate maximum flexibility and to simplify integration with kubernetes.

 ## Env mapping behaviors

 | ENV                     |                                                           |
 | -----------------------:|:---------------------------------------------------------:|
 | {PREFIX}_URL            | set service url, disregarding other overrides             |
 | {PREFIX}_URL_ENV        | override `{PREFIX}_URL` env mapping                       |
 | {PREFIX}_SCHEME         | set url scheme component                                  |
 | {PREFIX}_SCHEME_ENV     | override `{PREFIX}_SCHEME` env mapping                    |
 | {PREFIX}_HOST           | set url host component                                    |
 | {PREFIX}_HOST_ENV       | override `{PREFIX}_HOST` env mapping                      |
 | {PREFIX}_PATH           | set url path component                                    |
 | {PREFIX}_PATH_ENV       | override `{PREFIX}_PATH` env mapping                      |
 | {PREFIX}_QUERY          | set url query component                                   |
 | {PREFIX}_QUERY_ENV      | override `{PREFIX}_QUERY` env mapping                     |
 | {PREFIX}_USERINFO       | set url userinfo component                                |
 | {PREFIX}_USERINFO_ENV   | override `{PREFIX}_USERINFO` env mapping                  |

 ## Example

 ```
 use env_url::*;

 #[derive(EnvURL)]
 #[env_url(env_prefix = "REDIS", default = "redis://127.0.0.1:6379")]
 pub struct RedisDB;

 let service_url = RedisDB::service_url();

 ```

## Kubernetes usage

As kubernetes can set services to env variables for service discovery, it's very useful to be able to override the env mappings like so

```
REDIS_HOST_ENV: STAGE_REDIS_MASTER_PORT_6379_TCP_ADDR
REDIS_PORT_ENV: STAGE_REDIS_MASTER_SERVICE_PORT_REDIS
```
