impl super::LaunchAPI {
    pub fn new(op: super::LaunchAPIop) -> super::LaunchAPI {
        super::LaunchAPI {
            op,
            launch: None,
        }
    }

    pub fn new_launch(op: super::LaunchAPIop, launch: super::structures::Launch) -> super::LaunchAPI {
        super::LaunchAPI {
            op,
            launch: Some(launch),
        }
    }
}