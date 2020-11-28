use std::process::exit;

pub fn build_matrix(mut num: i64, style: u8) -> Vec<Vec<String>> {
    if num < 0 {
        num = num.abs();
    }
    let num_str = num.to_string();
    let mut matrix = Vec::<Vec<String>>::new();

    // println!("{} {}", num_str, num_str.len());

    for number in num_str.chars() {
        match style {
            0 => {
                match number {
                    '0' => {
                        let mut elements = Vec::<String>::new();
                        elements.push("  ___  ".to_string());
                        elements.push(" / _  \\".to_string());
                        elements.push("| | | |".to_string());
                        elements.push("| | | |".to_string());
                        elements.push("| |_| |".to_string());
                        elements.push(" \\___/ ".to_string());
                        matrix.push(elements.clone());
                        if num_str.len() < 2 {
                            matrix.push(elements);
                        }
                    }
                    '1' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  __ ".to_string());
                        elements.push(" /_ |".to_string());
                        elements.push("  | |".to_string());
                        elements.push("  | |".to_string());
                        elements.push("  | |".to_string());
                        elements.push("  |_|".to_string());
                        matrix.push(elements);
                    }
                    '2' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  ___  ".to_string());
                        elements.push(" |__ \\ ".to_string());
                        elements.push("    ) |".to_string());
                        elements.push("   / / ".to_string());
                        elements.push("  / /_ ".to_string());
                        elements.push(" |____|".to_string());
                        matrix.push(elements);
                    }
                    '3' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  ____  ".to_string());
                        elements.push(" |___ \\ ".to_string());
                        elements.push("   __) |".to_string());
                        elements.push("  |__ < ".to_string());
                        elements.push("  ___) |".to_string());
                        elements.push(" |____/ ".to_string());
                        matrix.push(elements);
                    }
                    '4' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  _  _   ".to_string());
                        elements.push(" | || |  ".to_string());
                        elements.push(" | || |_ ".to_string());
                        elements.push(" |__   _|".to_string());
                        elements.push("    | |  ".to_string());
                        elements.push("    |_|  ".to_string());
                        matrix.push(elements);
                    }
                    '5' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  _____ ".to_string());
                        elements.push(" | ____|".to_string());
                        elements.push(" | |__  ".to_string());
                        elements.push(" |___ \\ ".to_string());
                        elements.push("  ___) |".to_string());
                        elements.push(" |____/ ".to_string());
                        matrix.push(elements);
                    }
                    '6' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("    __  ".to_string());
                        elements.push("   / /  ".to_string());
                        elements.push("  / /_  ".to_string());
                        elements.push(" | '_ \\ ".to_string());
                        elements.push(" | (_) |".to_string());
                        elements.push(" \\____/ ".to_string());
                        matrix.push(elements);
                    }
                    '7' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("  ______ ".to_string());
                        elements.push(" |____  |".to_string());
                        elements.push("     / / ".to_string());
                        elements.push("    / /  ".to_string());
                        elements.push("   / /   ".to_string());
                        elements.push("  /_/    ".to_string());
                        matrix.push(elements);
                    }
                    '8' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("   ___  ".to_string());
                        elements.push("  / _ \\ ".to_string());
                        elements.push(" | (_) |".to_string());
                        elements.push("  > _ < ".to_string());
                        elements.push(" | (_) |".to_string());
                        elements.push("  \\___/ ".to_string());
                        matrix.push(elements);
                    }
                    '9' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push("  ___  ".to_string());
                            em2.push(" / _  \\".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| | | |".to_string());
                            em2.push("| |_| |".to_string());
                            em2.push(" \\___/ ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("   ___  ".to_string());
                        elements.push("  / _ \\ ".to_string());
                        elements.push(" | (_) |".to_string());
                        elements.push("  \\__, |".to_string());
                        elements.push("    / / ".to_string());
                        elements.push("   /_/  ".to_string());
                        matrix.push(elements);
                    }
                    _ => {}
                }
            }
            1 => {
                match number {
                    '0' => {
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements.clone());
                        if num_str.len() < 2 {
                            matrix.push(elements);
                        }
                    }
                    '1' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push("   #   ".to_string());
                        elements.push(" ###   ".to_string());
                        elements.push("   #   ".to_string());
                        elements.push("   #   ".to_string());
                        elements.push("   #   ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '2' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push(" #     ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '3' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("   ### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '4' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        matrix.push(elements);
                    }
                    '5' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push(" #     ".to_string());
                        elements.push(" #     ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '6' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push(" #     ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '7' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        matrix.push(elements);
                    }
                    '8' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        matrix.push(elements);
                    }
                    '9' => {
                        if num_str.len() < 2 {
                            let mut em2 = Vec::<String>::new();
                            em2.push(" ##### ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" #   # ".to_string());
                            em2.push(" ##### ".to_string());
                            matrix.push(em2);
                        }
                        let mut elements = Vec::<String>::new();
                        elements.push(" ##### ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" #   # ".to_string());
                        elements.push(" ##### ".to_string());
                        elements.push("     # ".to_string());
                        elements.push("     # ".to_string());
                        matrix.push(elements);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    return matrix;
}

pub fn render(days: Vec<Vec<String>>, hours: Vec<Vec<String>>, minutes: Vec<Vec<String>>, seconds: Vec<Vec<String>>, num_secs: i64, style: i8) {
    let mut sep_lines = Vec::<&str>::new();
    let mut timespace = Vec::<&str>::new();
    let mut is_t_plus = false;
    match style {
        0 => {
            sep_lines.push(" _ ");
            sep_lines.push("(_)");
            sep_lines.push("   ");
            sep_lines.push(" _ ");
            sep_lines.push("(_)");
            sep_lines.push("   ");
            if num_secs.is_negative() || is_t_plus {
                is_t_plus = true;
                timespace.push(" --+--           ");
                timespace.push("   |        |    ");
                timespace.push("   |        |    ");
                timespace.push("   |     ---+--- ");
                timespace.push("   |        |    ");
                timespace.push("   |        |    ");
            } else {
                timespace.push(" --+--           ");
                timespace.push("   |             ");
                timespace.push("   |             ");
                timespace.push("   |     ---+--- ");
                timespace.push("   |             ");
                timespace.push("   |             ");
            }
        }
        1 => {
            sep_lines.push("   ");
            sep_lines.push("###");
            sep_lines.push("###");
            sep_lines.push("   ");
            sep_lines.push("###");
            sep_lines.push("###");
            sep_lines.push("   ");
            if num_secs.is_negative() || is_t_plus {
                is_t_plus = true;
                timespace.push(" #####           ");
                timespace.push("   #        #    ");
                timespace.push("   #        #    ");
                timespace.push("   #     ####### ");
                timespace.push("   #        #    ");
                timespace.push("   #        #    ");
            } else {
                timespace.push(" #####           ");
                timespace.push("   #             ");
                timespace.push("   #             ");
                timespace.push("   #     ####### ");
                timespace.push("   #             ");
                timespace.push("   #             ");
            }
        }
        _ => {}
    }

    let mut day_lines = Vec::<String>::new();
    let mut hour_lines = Vec::<String>::new();
    let mut minute_lines = Vec::<String>::new();
    let mut second_lines = Vec::<String>::new();

    let d_first = days.first().unwrap();
    if days.len() > 1 {
        let d_last = days.last().unwrap();
        for y in 0..6 {
            day_lines.push(format!("{}  {}", d_first[y], d_last[y]));
        }
    } else {
        for y in 0..6 {
            day_lines.push(d_first[y].clone());
        }
    }

    let h_first = hours.first().unwrap();
    if hours.len() > 1 {
        let h_last = hours.last().unwrap();
        for y in 0..6 {
            hour_lines.push(format!("{}  {}", h_first[y], h_last[y]));
        }
    } else {
        for y in 0..6 {
            hour_lines.push(h_first[y].clone());
        }
    }

    let m_first = minutes.first().unwrap();
    if minutes.len() > 1 {
        let m_last = minutes.last().unwrap();
        for y in 0..6 {
            minute_lines.push(format!("{}  {}", m_first[y], m_last[y]));
        }
    } else {
        for y in 0..6 {
            minute_lines.push(m_first[y].clone());
        }
    }

    let s_first = seconds.first().unwrap();
    if seconds.len() > 1 {
        let s_last = seconds.last().unwrap();
        for y in 0..6 {
            second_lines.push(format!("{}  {}", s_first[y], s_last[y]));
        }
    } else {
        for y in 0..6 {
            second_lines.push(s_first[y].clone());
        }
    }

    for y in 0..6 {
        if (num_secs % 2) == 0 {
            println!("{}   {}   {}   {}   {}   {}   {}   {}", timespace[y], day_lines[y], sep_lines[y], hour_lines[y], sep_lines[y], minute_lines[y], sep_lines[y], second_lines[y]);
        } else {
            println!("{}   {}         {}         {}         {}", timespace[y], day_lines[y], hour_lines[y], minute_lines[y], second_lines[y]);
        }
    }
}