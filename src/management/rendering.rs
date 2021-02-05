use terminal_size::{Width, Height, terminal_size};


pub mod structures;

pub async fn render(frame: structures::Frame, previous: Option<structures::Frame>) {
    let size = terminal_size();
    let mut width = 0;
    let mut height = 0;
    if let Some((w,h)) = size {
        width = w.0;
        height = h.0;
    } else {
        width = 100;
        height = 35
    }
    println!("w: {} chars - h: {} chars", width, height);

    let lines: Vec<String> = vec!();
    for y in 0..height-2 {
        if y == 0 || y == height-3 {
            println!("+");
            for x in 0..(width-2) {
                println!("\x1b[1A\x1b[{}C=", x);
            }
            println!("\x1b[1A\x1b[{}C+", width-1);
        } else {
            println!("|");
            for x in 0..(width-2) {
                println!("\x1b[1A\x1b[{}C ", x);
            }
            println!("\x1b[1A\x1b[{}C|", width-1);
        }

    }

}