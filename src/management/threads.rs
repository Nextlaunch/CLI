use super::data::launches::LaunchAPI;
use super::data::telemetry::Snapshot;
use super::data::RenderFrame;
use tokio::sync::broadcast::*;
use crate::management::flags::Flags;


mod launch_mgr;
mod telemetry_mgr;
mod render_mgr;


pub async fn spawn_threads(f: Flags) -> ((Sender<LaunchAPI>, Receiver<LaunchAPI>), (Sender<Snapshot>, Receiver<Snapshot>), (Sender<RenderFrame>, Receiver<RenderFrame>)) {
    let (s_launch, mut r_launch): (Sender<LaunchAPI>, Receiver<LaunchAPI>) = channel(5);
    let (s_telem, mut r_telem): (Sender<Snapshot>, Receiver<Snapshot>) = channel(30);
    let (s_frame, mut r_frame): (Sender<RenderFrame>, Receiver<RenderFrame>) = channel(5);

    let (sc_launch, mut rc_launch): (Sender<LaunchAPI>, Receiver<LaunchAPI>) = (s_launch.clone(), s_launch.subscribe());
    let (sc_telem, mut rc_telem): (Sender<Snapshot>, Receiver<Snapshot>) = (s_telem.clone(), s_telem.subscribe());
    let (sc_frame, mut rc_frame): (Sender<RenderFrame>, Receiver<RenderFrame>) = (s_frame.clone(), s_frame.subscribe());

    launch_mgr::spawn(s_launch, r_launch, f.clone()).await;
    // telemetry_mgr::spawn(s_telem, r_telem).await;
    render_mgr::spawn(s_frame, r_frame, f.clone()).await;

    ((sc_launch, rc_launch), (sc_telem, rc_telem), (sc_frame, rc_frame))
}