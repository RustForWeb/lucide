use std::{error::Error, fmt, num::NonZeroI32, process::ExitStatus};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExitStatusError(Option<NonZeroI32>);

impl fmt::Display for ExitStatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(code) = self.0 {
            write!(f, "process exited unsuccessfully: {code}")
        } else {
            write!(f, "process exited unsuccessfully: unknown")
        }
    }
}

impl Error for ExitStatusError {}

pub trait ExitStatusExt {
    fn stable_exit_ok(&self) -> Result<(), ExitStatusError>;
}

impl ExitStatusExt for ExitStatus {
    fn stable_exit_ok(&self) -> Result<(), ExitStatusError> {
        match self.code() {
            Some(code) => match NonZeroI32::try_from(code) {
                Ok(code) => Err(ExitStatusError(Some(code))),
                Err(_) => Ok(()),
            },
            None => Err(ExitStatusError(None)),
        }
    }
}
