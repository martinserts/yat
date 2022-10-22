use std::{marker::PhantomData, ops::RangeInclusive, str::FromStr};

use num_traits::{Num, One};

use super::{error::SettingsError, settings_reader::SettingsReader, Settings};

static API_URL: &str = "API_URL";
static YAT_API_KEY: &str = "YAT_API_KEY";
static MIN_YAT_LENGTH: &str = "MIN_YAT_LENGTH";
static MAX_YAT_LENGTH: &str = "MAX_YAT_LENGTH";

/// Settings loader.
/// Complication with 'a lifetime arises from the need to use `mockall` for testing
pub struct Loader<'a, T: SettingsReader<'a>> {
    settings_reader: T,
    marker: PhantomData<&'a T>,
}

impl<'a, T: SettingsReader<'a>> Loader<'a, T> {
    pub fn new(settings_reader: T) -> Self {
        Self {
            settings_reader,
            marker: Default::default(),
        }
    }

    // Loads config from environment (including .env file)
    pub fn load(&self) -> Result<Settings, SettingsError> {
        // We do not care, if .env file is missing. The user might have the environment variables
        // defined other way
        let _ = self.settings_reader.load_dot_env();

        let api_url = self.get_env_var(API_URL)?;
        let yat_api_key = self.get_env_var(YAT_API_KEY)?;
        let min_yat_length = self.get_number(MIN_YAT_LENGTH)?;
        let max_yat_length = self.get_number(MAX_YAT_LENGTH)?;
        if min_yat_length > max_yat_length {
            return Err(SettingsError::MustNotBeLarger {
                min: MIN_YAT_LENGTH.to_owned(),
                max: MAX_YAT_LENGTH.to_owned(),
            });
        }
        Ok(Settings {
            api_url,
            yat_api_key,
            required_yat_length: RangeInclusive::new(min_yat_length, max_yat_length),
        })
    }

    fn get_env_var(&self, name: &'a str) -> Result<String, SettingsError> {
        self.settings_reader
            .var(name)
            .map_err(|_| SettingsError::VariableMissing(name.to_owned()))
    }

    fn get_number<N>(&self, name: &'a str) -> Result<N, SettingsError>
    where
        N: FromStr + PartialOrd + Num,
    {
        let val = self.get_env_var(name)?;
        let n = val
            .parse::<N>()
            .map_err(|_| SettingsError::NotNumber(name.to_owned()))?;
        if n < One::one() {
            return Err(SettingsError::NotGreaterThanZero(name.to_owned()));
        }
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;
    use std::env::VarError;
    use std::ffi::OsStr;
    use std::path::PathBuf;

    use super::SettingsReader;
    use super::*;

    mock! {
        pub SettingsReader {
        }

        impl SettingsReader<'static> for SettingsReader {
            fn load_dot_env(&self) -> dotenv::Result<PathBuf>;
            fn var<K: AsRef<OsStr> + 'static>(&self, key: K) -> Result<String, VarError>;
        }
    }

    fn setup_settings_reader<S>(reader: &mut MockSettingsReader, env: Option<&[(S, S)]>)
    where
        S: Into<String> + Clone,
    {
        reader
            .expect_load_dot_env()
            .times(1)
            .returning(|| Ok(PathBuf::new()));
        if let Some(env) = env {
            for (k, v) in env {
                let key: String = k.clone().into();
                let val: String = v.clone().into();
                reader
                    .expect_var::<&str>()
                    .withf(move |k| *k == key)
                    .return_once(move |_| Ok(val));
            }
        }
        reader
            .expect_var::<&str>()
            .returning(|_| Err(VarError::NotPresent));
    }

    #[test]
    fn it_loads_dot_env() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(&mut reader, None);
        reader.expect_var::<&str>().returning(|_| Ok(String::new()));

        let loader = Loader::new(reader);
        let _ = loader.load();
    }

    #[test]
    fn it_fails_if_variable_is_missing() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(&mut reader, None);

        let loader = Loader::new(reader);
        let r = loader.load();

        assert_eq!(r, Err(SettingsError::VariableMissing("API_URL".to_owned())))
    }

    #[test]
    fn it_fails_if_min_is_not_a_number() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(
            &mut reader,
            Some(&[
                ("API_URL", "http://url"),
                ("YAT_API_KEY", "KEY"),
                ("MIN_YAT_LENGTH", "Nan"),
                ("MAX_YAT_LENGTH", "3"),
            ]),
        );

        let loader = Loader::new(reader);
        let r = loader.load();

        assert_eq!(
            r,
            Err(SettingsError::NotNumber("MIN_YAT_LENGTH".to_owned()))
        )
    }

    #[test]
    fn it_fails_if_min_is_zero() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(
            &mut reader,
            Some(&[
                ("API_URL", "http://url"),
                ("YAT_API_KEY", "KEY"),
                ("MIN_YAT_LENGTH", "0"),
                ("MAX_YAT_LENGTH", "3"),
            ]),
        );

        let loader = Loader::new(reader);
        let r = loader.load();

        assert_eq!(
            r,
            Err(SettingsError::NotGreaterThanZero(
                "MIN_YAT_LENGTH".to_owned()
            ))
        )
    }

    #[test]
    fn it_fails_if_max_is_less_than_min() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(
            &mut reader,
            Some(&[
                ("API_URL", "http://url"),
                ("YAT_API_KEY", "KEY"),
                ("MIN_YAT_LENGTH", "9"),
                ("MAX_YAT_LENGTH", "3"),
            ]),
        );

        let loader = Loader::new(reader);
        let r = loader.load();

        assert_eq!(
            r,
            Err(SettingsError::MustNotBeLarger {
                min: "MIN_YAT_LENGTH".to_owned(),
                max: "MAX_YAT_LENGTH".to_owned()
            })
        )
    }

    #[test]
    fn it_loads_data_from_environment() {
        let mut reader = MockSettingsReader::new();
        setup_settings_reader::<&str>(
            &mut reader,
            Some(&[
                ("API_URL", "http://url"),
                ("YAT_API_KEY", "KEY"),
                ("MIN_YAT_LENGTH", "1"),
                ("MAX_YAT_LENGTH", "5"),
            ]),
        );

        let loader = Loader::new(reader);
        let r = loader.load();

        assert_eq!(
            r,
            Ok(Settings {
                api_url: "http://url".to_owned(),
                yat_api_key: "KEY".to_owned(),
                required_yat_length: (1..=5),
            })
        )
    }
}
