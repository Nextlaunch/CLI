use tui::widgets::{Borders, Block, Paragraph, Table, Row};
use tui::text::Text;
use tui::style::{Style, Modifier, Color};
use tui::layout::Constraint;
use crate::languages::LanguagePack;
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

pub fn render_dynamic(language: &LanguagePack, launch: Launch) -> Table<'static> {
    let suc = Text::styled(language.launch.status.success, Style::default().fg(Color::LightGreen));
    let tbd = Text::styled(language.launch.status.to_be_determined, Style::default().fg(Color::Yellow));
    let tbc = Text::styled(language.launch.status.to_be_confirmed, Style::default().fg(Color::LightYellow));
    let paf = Text::styled(language.launch.status.partial_failure, Style::default().fg(Color::LightYellow));
    let fal = Text::styled(language.launch.status.failure, Style::default().fg(Color::Red));
    let g4l = Text::styled(language.launch.status.go_for_liftoff, Style::default().fg(Color::Green));
    let inf = Text::styled(language.launch.status.in_flight, Style::default().fg(Color::LightGreen));
    let hol = Text::styled(language.launch.status.on_hold, Style::default().fg(Color::Gray));
    let fetching = Text::raw(format!("{}...", language.launch.status.fetching));

    let raw_name = launch.name.clone().unwrap_or(format!("{} | {}", language.launch.unknown_launch, language.launch.unknown_mission));
    let pieces: Vec<&str> = raw_name.split(" | ").collect();
    let mission = pieces.last().unwrap_or(&format!("{}", language.launch.unknown_mission).as_str()).to_string();

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
        Row::new(vec![Text::from(format!(" {}", language.launch.name)), Text::styled(launch.name.unwrap_or(format!("{} | {}", language.titles.unknown_vehicle, language.titles.unknown_mission)), Style::default().add_modifier(Modifier::UNDERLINED))]),
        Row::new(vec![format!(" {}", language.launch.provider), lsp.name.unwrap_or("Unknown Provider".to_string())]),
        Row::new(vec![format!(" {}", language.launch.vehicle), vehicle.name.unwrap_or(language.title.unknown_vehicle)]),
        Row::new(vec![format!(" {}", language.launch.mission), mission.clone()]),
        Row::new(vec![format!(" {}", language.launch.pad), launchpad.name.unwrap_or(language.titles.unknown_launchpad)]),
        Row::new(vec![format!(" {}", language.launch.location), launchpad.location.name.unwrap_or(language.titles.unknown_location)]),
        Row::new(vec![Text::from(format!(" {}", language.launch.status.name)), status]),
    ])
        .widths(&[
            Constraint::Min(10),
            Constraint::Min(45)
        ])
        .block(Block::default().title(format!(" {} ", language.titles.launch)).borders(Borders::ALL))
}