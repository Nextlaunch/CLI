#[derive(Debug, Clone)]
pub struct State {
    pub view_screen: i8,
    pub selected_article: i8,
    pub selected_update: i8,
    pub selected_side: i8,
    pub should_clear: bool,
    pub render_settings: bool,
    pub settings_pane: i8,
    pub launch_update_count: i8,
    pub open_selected: bool,
    pub news_article_count: i8,
    pub render_help: bool,
    pub settings_selected: i8,
    pub editing_settings: bool,
    pub stored_value: Vec<char>,
    pub save_stored: bool,
}

impl State {
    pub fn new() -> State {
        State {
            view_screen: 0,
            selected_article: 0,
            selected_update: 0,
            selected_side: 0,
            should_clear: true,
            render_help: false,
            settings_selected: 0,
            render_settings: false,
            settings_pane: 0,
            launch_update_count: 0,
            open_selected: false,
            news_article_count: 0,
            editing_settings: false,
            stored_value: vec![],
            save_stored: false
        }
    }
}


