use terminal_size::{Width, Height, terminal_size};


pub mod structures;

pub async fn render(frame: structures::Frame, previous: Option<structures::Frame>) {
    let size = terminal_size();
    let mut width = 0;
    let mut height = 0;
    if let Some((w, h)) = size {
        width = w.0 as usize;
        height = h.0 as usize;
    } else {
        width = 100;
        height = 35
    }
    println!("w: {} chars - h: {} chars", width, height);

    if width > 300 {
        width = 300;
    }
    if height > 100 {
        height = 100;
    }


    // Build border of the system
    let mut lines: Vec<Vec<&str>> = vec!();
    for y in 0..height - 2 {
        let mut ln: Vec<&str> = vec!();
        if y == 0 || y == height - 3 {
            ln.push("+");
            for _ in 0..(width - 2) {
                ln.push("=");
            }
            ln.push("+");
        } else {
            ln.push("|");
            for _ in 0..(width - 2) {
                ln.push(" ");
            }
            ln.push("|");
        }
        lines.push(ln);
    }

    let mut h: usize = 0;

    let ln_width = lines[0].len()-1;

    for row in frame.rows.clone() {
        let pos = 0;
        for mut cell in row {
            if cell.height == 0 {
                cell.height = height;
            }
            if cell.width == 0 {
                cell.width = width;
            }


            if cell.height < height && pos == 0 {
                lines[h + cell.height + 1][0] = "+";
                h += (cell.height + 1);


            } else if cell.height < height && pos == frame.rows.len() - 1 {
                lines[h + cell.height + 1][ln_width] = "+";
                h += (cell.height + 1);
            }
        }
    }

    for line in lines {
        println!("{}", line.join(""));
    }
}