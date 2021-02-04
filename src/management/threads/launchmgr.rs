use crate::management::data::launches::LaunchAPI;
use tokio::sync::broadcast::*;

pub async fn spawn(mut s: Sender<LaunchAPI>, mut r: Receiver<LaunchAPI>) {
    tokio::spawn(async move {
        loop {
            let inc_res = r.try_recv();

            if let Ok(payload) = inc_res {
                dbg!(payload)
            } else {
                println!("NO PAYLOAD AVAILABLE")
            }
        }
    });
}