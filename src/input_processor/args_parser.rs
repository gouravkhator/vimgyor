use std::{
    env::{self, Args},
    error::Error,
};

use crate::error_handler::AppError;

pub struct ArgParser {}

impl ArgParser {
    pub fn parse_env_args() -> Result<Option<String>, Box<dyn Error>> {
        let mut args = env::args();

        Self::validate_args_passed(&args)?;

        if let Some(filepath) = args.nth(1) {
            Ok(Some(filepath))
        } else {
            Ok(None)
        }
    }

    fn validate_args_passed(args: &Args) -> Result<(), Box<dyn Error>> {
        if args.len() > 2 {
            /*
             * we only support atmost 2 arguments,
             * the first one being the executable, and second one being the filename/path
             */
            Err(AppError::new(
                "This software does not support more than 2 arguments",
            ))
        } else {
            Ok(())
        }
    }
}
