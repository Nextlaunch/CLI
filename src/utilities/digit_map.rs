pub fn map_str(content: &str) -> [String; 9] {
    let mut raw_map = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        "    Years                Weeks                Days                 Hours               Minutes              Seconds   \u{200b}".to_string()
    ];

    let mut x = 0;
    for char in content.chars() {
        let incoming = map_digit(char);
        if x == 0 {
            raw_map[1] = format!("{}{}", raw_map[1], incoming[1]);
            raw_map[2] = format!("{}{}", raw_map[2], incoming[2]);
            raw_map[3] = format!("{}{}", raw_map[3], incoming[3]);
            raw_map[4] = format!("{}{}", raw_map[4], incoming[4]);
            raw_map[5] = format!("{}{}", raw_map[5], incoming[5]);
            raw_map[6] = format!("{}{}", raw_map[6], incoming[6]);
            raw_map[7] = format!("{}{}", raw_map[7], incoming[7]);
            x += 1;
        } else {
            raw_map[1] = format!("{}   {}", raw_map[1], incoming[1]);
            raw_map[2] = format!("{}   {}", raw_map[2], incoming[2]);
            raw_map[3] = format!("{}   {}", raw_map[3], incoming[3]);
            raw_map[4] = format!("{}   {}", raw_map[4], incoming[4]);
            raw_map[5] = format!("{}   {}", raw_map[5], incoming[5]);
            raw_map[6] = format!("{}   {}", raw_map[6], incoming[6]);
            raw_map[7] = format!("{}   {}", raw_map[7], incoming[7]);
        }
    }

    raw_map[1] = format!("{}\u{200b}", raw_map[1]);
    raw_map[2] = format!("{}\u{200b}", raw_map[2]);
    raw_map[3] = format!("{}\u{200b}", raw_map[3]);
    raw_map[4] = format!("{}\u{200b}", raw_map[4]);
    raw_map[5] = format!("{}\u{200b}", raw_map[5]);
    raw_map[6] = format!("{}\u{200b}", raw_map[6]);
    raw_map[7] = format!("{}\u{200b}", raw_map[7]);
    raw_map
}


fn map_digit(c: char) -> [String; 9] {
    let mut raw_map = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new()
    ];

    match c {
        '0' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "#   #".to_string();
            raw_map[3] = "#   #".to_string();
            raw_map[4] = "#   #".to_string();
            raw_map[5] = "#   #".to_string();
            raw_map[6] = "#   #".to_string();
            raw_map[7] = "#####".to_string();
        }
        '1' => {
            raw_map[1] = "    #".to_string();
            raw_map[2] = "    #".to_string();
            raw_map[3] = "    #".to_string();
            raw_map[4] = "    #".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "    #".to_string();
        }
        '2' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "    #".to_string();
            raw_map[3] = "    #".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "#    ".to_string();
            raw_map[6] = "#    ".to_string();
            raw_map[7] = "#####".to_string();
        }
        '3' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "    #".to_string();
            raw_map[3] = "    #".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "#####".to_string();
        }
        '4' => {
            raw_map[1] = "#   #".to_string();
            raw_map[2] = "#   #".to_string();
            raw_map[3] = "#   #".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "    #".to_string();
        }
        '5' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "#    ".to_string();
            raw_map[3] = "#    ".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "#####".to_string();
        }
        '6' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "#    ".to_string();
            raw_map[3] = "#    ".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "#   #".to_string();
            raw_map[6] = "#   #".to_string();
            raw_map[7] = "#####".to_string();
        }
        '7' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "    #".to_string();
            raw_map[3] = "    #".to_string();
            raw_map[4] = "    #".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "    #".to_string();
        }
        '8' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "#   #".to_string();
            raw_map[3] = "#   #".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "#   #".to_string();
            raw_map[6] = "#   #".to_string();
            raw_map[7] = "#####".to_string();
        }
        '9' => {
            raw_map[1] = "#####".to_string();
            raw_map[2] = "#   #".to_string();
            raw_map[3] = "#   #".to_string();
            raw_map[4] = "#####".to_string();
            raw_map[5] = "    #".to_string();
            raw_map[6] = "    #".to_string();
            raw_map[7] = "    #".to_string();
        }
        ':' => {
            raw_map[1] = "  ".to_string();
            raw_map[2] = "##".to_string();
            raw_map[3] = "##".to_string();
            raw_map[4] = "  ".to_string();
            raw_map[5] = "##".to_string();
            raw_map[6] = "##".to_string();
            raw_map[7] = "  ".to_string();
        }
        _ => {}
    }

    raw_map
}