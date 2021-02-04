use super::data::launches::LaunchAPI;
use super::data::telemetry::Snapshot;
use super::data::RenderFrame;
use tokio::sync::broadcast::*;
use crate::management::flags::Flags;


mod launchmgr;
mod telemetrymgr;
mod rendermgr;


pub async fn spawn_threads(f: Flags) -> ((Sender<LaunchAPI>, Receiver<LaunchAPI>), (Sender<Snapshot>, Receiver<Snapshot>), (Sender<RenderFrame>, Receiver<RenderFrame>)) {

    let (mut s_launch, mut r_launch): (Sender<LaunchAPI>, Receiver<LaunchAPI>) = channel(5);
    let (mut s_telem, mut r_telem): (Sender<Snapshot>, Receiver<Snapshot>) = channel(30);
    let (mut s_frame, mut r_frame): (Sender<RenderFrame>, Receiver<RenderFrame>) = channel(5);

    let (mut sc_launch, mut rc_launch): (Sender<LaunchAPI>, Receiver<LaunchAPI>) = (s_launch.clone(), s_launch.subscribe());
    let (mut sc_telem, mut rc_telem): (Sender<Snapshot>, Receiver<Snapshot>) = (s_telem.clone(), s_telem.subscribe());
    let (mut sc_frame, mut rc_frame): (Sender<RenderFrame>, Receiver<RenderFrame>) = (s_frame.clone(), s_frame.subscribe());

    launchmgr::spawn(s_launch, r_launch, f.clone()).await;
    // telemetrymgr::spawn(s_telem, r_telem).await;
    // rendermgr::spawn(s_frame, r_frame).await;

    ((sc_launch, rc_launch), (sc_telem, rc_telem), (sc_frame, rc_frame))
}