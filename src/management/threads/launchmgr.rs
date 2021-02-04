use crate::management::data::launches::Launch;
use tokio::sync::broadcast::*;

pub async fn spawn(mut s: Sender<Launch>, mut r: Receiver<Launch>) {
    tokio::spawn(async move {

    });
}