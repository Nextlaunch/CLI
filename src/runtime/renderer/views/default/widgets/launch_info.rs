use tui::widgets::{Borders, Block, Paragraph, Table, Row};
use tui::text::Text;
use tui::style::{Style, Modifier, Color};
use tui::layout::Constraint;
use crate::runtime::data::launches::structures::{Launch, PadLocation, LaunchPad, LSP, RocketConfiguration, Rocket};

pub fn render_missing() -> Paragraph<'static> {
    Paragraph::new(
        Text::raw(
            " Unfortunately, there is not a launch currently available.\n Please check the logs."
        )
    )
        .block(
            Block::default()
                .title(" Launch Info ")
                .borders(Borders::ALL)
        )
}

pub fn render_dynamic(launch: Launch) -> Table<'static> {
    let suc = Text::styled("Launch Successful", Style::default().fg(Color::LightGreen));
    let tbd = Text::styled("To Be Determined", Style::default().fg(Color::Yellow));
    let tbc = Text::styled("To Be Confirmed", Style::default().fg(Color::LightYellow));
    let paf = Text::styled("Partial Failure", Style::default().fg(Color::LightYellow));
    let fal = Text::styled("Launch Failure", Style::default().fg(Color::Red));
    let g4l = Text::styled("Go For Launch", Style::default().fg(Color::Green));
    let inf = Text::styled("In Flight", Style::default().fg(Color::LightGreen));
    let hol = Text::styled("On Hold", Style::default().fg(Color::Gray));
    let fetching = Text::raw("Fetching...");

    let raw_name = launch.name.clone().unwrap_or("Unknown Launch | Unknown Mission".to_string());
    let pieces: Vec<&str> = raw_name.split(" | ").collect();
    let mission = pieces.last().unwrap_or(&"Unknown Mission").to_string();

    let vehicle = launch.rocket.clone().unwrap_or(Rocket {
        id: None,
        configuration: None,
    }).configuration
        .unwrap_or(RocketConfiguration {
            id: None,
            name: None,
            description: None,
            family: None,
            full_name: None,
            manufacturer: None,
            variant: None,
            alias: None,
            min_stage: None,
            max_stage: None,
            length: None,
            diameter: None,
            maiden_flight: None,
            launch_mass: None,
            leo_capacity: None,
            gto_capacity: None,
            to_thrust: None,
            apogee: None,
            vehicle_range: None,
            total_launch_count: None,
            consecutive_successful_launches: None,
            successful_launches: None,
            failed_launches: None,
            pending_launches: None,
        });

    let lsp = launch.launch_service_provider.clone().unwrap_or(LSP {
        id: None,
        name: None,
        features: None,
        org: None,
        country_code: None,
        abbrev: None,
        description: None,
        administrator: None,
        founding_year: None,
        launchers: None,
        spacecraft: None,
        launch_library_url: None,
        total_launch_count: None,
        consecutive_successful_launches: None,
        successful_launches: None,
        failed_launches: None,
        pending_launches: None,
        consecutive_successful_landings: None,
        successful_landings: None,
        failed_landings: None,
        attempted_landings: None,
        logo: None
    });

    let launchpad = launch.pad.unwrap_or(LaunchPad {
        id: None,
        agency_id: None,
        name: None,
        latitude: None,
        longitude: None,
        location: PadLocation {
            id: None,
            name: None,
            country_code: None,
            total_launch_count: None,
            total_landing_count: None,
        },
        total_launch_count: None,
    });

    let status = match launch.status.id.clone() {
        None => fetching,
        Some(status) => {
            match status {
                1 => g4l,
                2 => tbd,
                3 => suc,
                4 => fal,
                5 => hol,
                6 => inf,
                7 => paf,
                8 => tbc,
                _ => fetching
            }
        }
    };

    Table::new(vec![
        Row::new(vec![Text::from(" Name"), Text::styled(launch.name.unwrap_or("Unknown Launch | Unknown Mission".to_string()), Style::default().add_modifier(Modifier::UNDERLINED))]),
        Row::new(vec![" Provider".to_string(), lsp.name.unwrap_or("Unknown Provider".to_string())]),
        Row::new(vec![" Vehicle".to_string(), vehicle.name.unwrap_or("Unknown Launch Vehicle".to_string())]),
        Row::new(vec![" Mission".to_string(), mission.clone()]),
        Row::new(vec![" Pad".to_string(), launchpad.name.unwrap_or("Unkown Launchpad".to_string())]),
        Row::new(vec![" Location".to_string(), launchpad.location.name.unwrap_or("Unkown Location".to_string())]),
        Row::new(vec![Text::from(" Status"), status]),
    ])
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(45)
        ])
        .block(Block::default().title(" Launch Info ").borders(Borders::ALL))
}