// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use crate::error::ConfigurationError;

#[derive(Debug)]
pub struct PlatformConfig {
    endpoint: Endpoint,
    rest_api_endpoint: String,
    database_url: String,
    #[cfg(feature = "splinter-support")]
    admin_key_dir: String,
}

impl PlatformConfig {
    pub fn endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    pub fn rest_api_endpoint(&self) -> &str {
        &self.rest_api_endpoint
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    #[cfg(feature = "splinter-support")]
    pub fn admin_key_dir(&self) -> &str {
        &self.admin_key_dir
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Endpoint {
    backend: Backend,
    url: String,
}

impl Endpoint {
    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn is_sawtooth(&self) -> bool {
        self.backend == Backend::Sawtooth
    }

    pub fn is_splinter(&self) -> bool {
        self.backend == Backend::Splinter
    }
}

impl From<&str> for Endpoint {
    fn from(s: &str) -> Self {
        let s = s.to_lowercase();

        if s.starts_with("splinter:") {
            let url = s.replace("splinter:", "");
            Endpoint {
                backend: Backend::Splinter,
                url,
            }
        } else if s.starts_with("sawtooth:") {
            let url = s.replace("sawtooth:", "");
            Endpoint {
                backend: Backend::Sawtooth,
                url,
            }
        } else {
            Endpoint {
                backend: Backend::Sawtooth,
                url: s,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Backend {
    Splinter,
    Sawtooth,
}

pub struct PlatformConfigBuilder {
    endpoint: Option<Endpoint>,
    rest_api_endpoint: Option<String>,
    database_url: Option<String>,
    #[cfg(feature = "splinter-support")]
    admin_key_dir: Option<String>,
}

impl Default for PlatformConfigBuilder {
    fn default() -> Self {
        Self {
            endpoint: Some(Endpoint {
                url: "tcp://127.0.0.1:4004".to_owned(),
                backend: Backend::Sawtooth,
            }),
            rest_api_endpoint: Some("127.0.0.1:8080".to_owned()),
            database_url: Some("postgres://dgc-platform:dgc_platform_example@localhost/dgc-platform".to_owned()),
            #[cfg(feature = "splinter-support")]
            admin_key_dir: Some("/etc/dgc-platform/keys".to_owned()),
        }
    }
}

impl PlatformConfigBuilder {
    pub fn with_cli_args(&mut self, matches: &clap::ArgMatches<'_>) -> Self {
        Self {
            endpoint: matches
                .value_of("connect")
                .map(Endpoint::from)
                .or_else(|| self.endpoint.take()),

            rest_api_endpoint: matches
                .value_of("bind")
                .map(ToOwned::to_owned)
                .or_else(|| self.rest_api_endpoint.take()),

            database_url: matches
                .value_of("database_url")
                .map(ToOwned::to_owned)
                .or_else(|| self.database_url.take()),

            #[cfg(feature = "splinter-support")]
            admin_key_dir: matches
                .value_of("admin_key_dir")
                .map(ToOwned::to_owned)
                .or_else(|| self.admin_key_dir.take()),
        }
    }

    pub fn build(mut self) -> Result<PlatformConfig, ConfigurationError> {
        Ok(PlatformConfig {
            endpoint: self
                .endpoint
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("endpoint".to_owned()))?,
            rest_api_endpoint: self
                .rest_api_endpoint
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("rest_api_endpoint".to_owned()))?,
            database_url: self
                .database_url
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("database_url".to_owned()))?,
            #[cfg(feature = "splinter-support")]
            admin_key_dir: self
                .admin_key_dir
                .take()
                .ok_or_else(|| ConfigurationError::MissingValue("admin_key_dir".to_owned()))?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn build_with_args() {
        let matches = clap::App::new("testapp")
            .arg(clap::Arg::with_name("connect").short("C").takes_value(true))
            .arg(clap::Arg::with_name("bind").short("b").takes_value(true))
            .get_matches_from(vec![
                "testapp",
                "-C",
                "validator:4004",
                "-b",
                "rest_api:8080",
            ]);

        let config = PlatformConfigBuilder::default()
            .with_cli_args(&matches)
            .build()
            .expect("Unable to build configuration");

        assert_eq!("validator:4004", config.endpoint().url());
        assert_eq!("rest_api:8080", config.rest_api_endpoint());
    }

    #[test]
    fn build_with_missing_args() {
        let matches = clap::App::new("testapp")
            .arg(clap::Arg::with_name("connect").short("C").takes_value(true))
            .arg(clap::Arg::with_name("bind").short("b").takes_value(true))
            .get_matches_from(vec!["testapp"]);

        let config = PlatformConfigBuilder::default()
            .with_cli_args(&matches)
            .build()
            .expect("Unable to build configuration");

        assert_eq!("tcp://127.0.0.1:4004", config.endpoint().url());
        assert_eq!("127.0.0.1:8080", config.rest_api_endpoint());
    }

    #[test]
    fn test_endpoint_splinter_prefix() {
        let endpoint = Endpoint::from("splinter:tcp://localhost:8080");
        assert_eq!(
            endpoint,
            Endpoint {
                backend: Backend::Splinter,
                url: "tcp://localhost:8080".into()
            }
        );
    }

    #[test]
    fn test_endpoint_sawtooth_prefix() {
        let endpoint = Endpoint::from("sawtooth:tcp://localhost:8080");
        assert_eq!(
            endpoint,
            Endpoint {
                backend: Backend::Sawtooth,
                url: "tcp://localhost:8080".into()
            }
        );
    }

    #[test]
    fn test_endpoint_no_prefix() {
        let endpoint = Endpoint::from("tcp://localhost:8080");
        assert_eq!(
            endpoint,
            Endpoint {
                backend: Backend::Sawtooth,
                url: "tcp://localhost:8080".into()
            }
        );
    }

    #[test]
    fn test_endpoint_capitals() {
        let endpoint = Endpoint::from("SAWTOOTH:TCP://LOCALHOST:8080");
        assert_eq!(
            endpoint,
            Endpoint {
                backend: Backend::Sawtooth,
                url: "tcp://localhost:8080".into()
            }
        );
    }

    #[test]
    fn test_endpoint_no_protocol() {
        let endpoint = Endpoint::from("splinter:localhost:8080");
        assert_eq!(
            endpoint,
            Endpoint {
                backend: Backend::Splinter,
                url: "localhost:8080".into()
            }
        );
    }
}
