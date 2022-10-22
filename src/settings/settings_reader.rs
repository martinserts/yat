use dotenv::dotenv;
use std::{env, env::VarError, ffi::OsStr, path::PathBuf};

pub trait SettingsReader<'a> {
    fn load_dot_env(&self) -> dotenv::Result<PathBuf>;
    fn var<K: AsRef<OsStr> + 'a>(&self, key: K) -> Result<String, VarError>;
}

pub struct EnvSettingsReader;

impl<'a> SettingsReader<'a> for EnvSettingsReader {
    fn load_dot_env(&self) -> dotenv::Result<PathBuf> {
        dotenv()
    }

    fn var<K: AsRef<OsStr> + 'a>(&self, key: K) -> Result<String, VarError> {
        env::var(key)
    }
}
