use crate::app_config;
use std::error;
use std::fmt::Display;
use std::fmt::Formatter;

pub fn list() -> Result<(), Error> {
    let accounts = app_config::list_accounts().map_err(Error::AppConfig)?;
    err_if_no_accounts(&accounts)?;

    for account in accounts {
        println!("{}", account);
    }

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    AppConfig(app_config::Error),
    NoAccounts,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AppConfig(e) => write!(f, "{}", e),
            Error::NoAccounts => {
                writeln!(f, "No accounts found")?;
                write!(f, "Use `ggldrive account add` to add an account.")
            }
        }
    }
}

fn err_if_no_accounts(accounts: &[String]) -> Result<(), Error> {
    if accounts.is_empty() {
        Err(Error::NoAccounts)
    } else {
        Ok(())
    }
}