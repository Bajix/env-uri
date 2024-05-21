extern crate darling;
extern crate syn;

use darling::{FromDeriveInput, FromMeta};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Lit};
use url::Url;

struct ServiceUrl(Url);

impl FromMeta for ServiceUrl {
  fn from_value(value: &Lit) -> Result<Self, darling::Error> {
    if let Lit::Str(ref lit_str) = *value {
      match Url::parse(lit_str.value().as_str()) {
        Ok(url) => Ok(ServiceUrl(url)),
        Err(err) => Err(darling::Error::custom(err).with_span(&lit_str.span())),
      }
    } else {
      Err(darling::Error::unexpected_lit_type(value))
    }
  }
}

#[derive(FromDeriveInput)]
#[darling(attributes(env_url))]
struct ServiceURLOpt {
  env_prefix: String,
  default: ServiceUrl,
}

#[proc_macro_derive(EnvURL, attributes(env_url))]
pub fn derive_service_uri(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let ServiceURLOpt {
    env_prefix,
    default: ServiceUrl(url),
  } = match FromDeriveInput::from_derive_input(&input) {
    Ok(attr) => attr,
    Err(err) => {
      return err.write_errors().into();
    }
  };

  let prefixed_url_env_key = format!("{}_URL_ENV", env_prefix);
  let prefixed_url_key = format!("{}_URL", env_prefix);
  let prefixed_scheme_env_key = format!("{}_SCHEMA_ENV", env_prefix);
  let prefixed_scheme_key = format!("{}_SCHEMA_ENV", env_prefix);
  let prefixed_host_env_key = format!("{}_HOST_ENV", env_prefix);
  let prefixed_host_key = format!("{}_HOST", env_prefix);
  let prefixed_port_env_key = format!("{}_PORT_ENV", env_prefix);
  let prefixed_port_key = format!("{}_PORT", env_prefix);
  let prefixed_path_env_key = format!("{}_PATH_ENV", env_prefix);
  let prefixed_path_key = format!("{}_PATH", env_prefix);
  let prefixed_query_env_key = format!("{}_QUERY_ENV", env_prefix);
  let prefixed_query_key = format!("{}_QUERY", env_prefix);
  let prefixed_userinfo_env_key = format!("{}_USERINFO_ENV", env_prefix);
  let prefixed_userinfo_key = format!("{}_USERINFO", env_prefix);

  let default_scheme = url.scheme();
  let default_host = url.host_str().unwrap();
  let default_port = match url.port() {
    Some(port) => port.to_string(),
    None => String::from(""),
  };

  let default_path = url.path();
  let default_query = url.query().unwrap_or("");

  let default_userinfo = match url.password() {
    Some(password) => format!("{}:{}", url.username(), password),
    None => String::from(url.username()),
  };

  let expanded = quote! {
    impl env_url::ServiceURL for #ident {
      fn service_url() -> Result<env_url::url::Url, env_url::url::ParseError> {
        let service_url_env = std::env::var(#prefixed_url_env_key).unwrap_or_else(|_| String::from(#prefixed_url_key));

        let service_url = std::env::var(service_url_env).unwrap_or_else(|_| {
          let scheme = {
            let host_env =
              std::env::var(#prefixed_scheme_env_key).unwrap_or_else(|_| String::from(#prefixed_scheme_key));

            std::env::var(host_env).unwrap_or_else(|_| String::from(#default_scheme))
          };

          let host = {
            let host_env =
              std::env::var(#prefixed_host_env_key).unwrap_or_else(|_| String::from(#prefixed_host_key));

            std::env::var(host_env).unwrap_or_else(|_| String::from(#default_host))
          };

          let port = {
            let port_env = std::env::var(#prefixed_port_env_key).unwrap_or_else(|_| String::from(#prefixed_port_key));

            std::env::var(port_env).unwrap_or_else(|_| String::from(#default_port))
          };

          let path = {
            let path_env =
              std::env::var(#prefixed_path_env_key).unwrap_or_else(|_| String::from(#prefixed_path_key));

            std::env::var(path_env).unwrap_or_else(|_| String::from(#default_path))
          };

          let userinfo = {
            let userinfo_env =
              std::env::var(#prefixed_userinfo_env_key).unwrap_or_else(|_| String::from(#prefixed_userinfo_key));

            std::env::var(userinfo_env).unwrap_or_else(|_| String::from(#default_userinfo))
          };

          let query = {
            let query_env =
              std::env::var(#prefixed_query_env_key).unwrap_or_else(|_| String::from(#prefixed_query_key));

            std::env::var(query_env).unwrap_or_else(|_| String::from(#default_query))
          };

          match (userinfo.is_empty(), query.is_empty()) {
            (false, true) =>  format!(
              "{}://{}@{}:{}{}",
              scheme, userinfo, host, port, path
            ),
            (false, false) => format!(
              "{}://{}@{}:{}{}?{}",
              scheme, userinfo, host, port, path, query
            ),
            (true, true) => format!("{}://{}:{}{}", scheme, host, port, path),
            (true, false) => format!("{}://{}:{}{}?{}", scheme, host, port, path, query)
          }
        });

        env_url::Url::parse(&service_url)
      }
    }
  };

  expanded.into()
}
