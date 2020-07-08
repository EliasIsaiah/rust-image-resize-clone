extern crate aws_lambda_events;
extern crate image;
#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate log;
extern crate rayon;
extern crate s3;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;

use image::{ImageOutputFormat, GenericImageView, ImageError};

use rayon::prelude::*;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;

mod config;

use aws_lambda_events::event::s3::{S3Event, S3EventRecord};
use config::Config;
use lambda::error::HandlerError;
use serde_json::Value;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;

    lambda!(handle_event);

    Ok(())
}

fn handle_event(event: Value, ctx: lambda::Context) -> Result<(), HandlerError> {
    let config = Config::new();

    let s3_event: S3Event =
        serde_json::from_value(event).map_err(|e| ctx.new_error(e.to_string().as_str()))?;
}