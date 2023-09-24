use std::error::Error;
use tokio::sync::oneshot::Sender;
use common_lib::model::Zipcode;
use crate::app_error::AppError;

#[derive(Debug)]
pub enum Command {
    GetZipDetails {
        key: String,
        resp_tx: Sender<Result<Zipcode, AppError>>
    },

    UpdateZipDetails {
        key: String,
        zipcode: Zipcode,
        resp_tx: Sender<Result<Zipcode, AppError>>
    },
}