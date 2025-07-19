use colored::Colorize;
use crossterm::*;
use std;

use crate::SIZE_X;
use crate::SIZE_Y;

fn draw_empty_line() {
    print!("{}", "*".blue());
    for _ in 0..SIZE_X {
        print!("{}", "==".bright_blue());
    }
    print!("{}", "=".bright_blue());
    println!("{}", "*".blue());
    let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveToColumn(0));
}

fn draw_line(tab: [[i8; SIZE_Y]; SIZE_X], line_y: usize) {
    print!("{}", "| ".blue());
    for i in 0..SIZE_X {
        let val: i8 = tab[i][line_y];
        if val == 0 {
            print!("{}", "☐ ".red());
        } else if val == 1 {
            print!("  ");
        } else if val == 2 {
            print!("{}", "● ".green());
        } else if val == 3 {
            print!("{}", "☐ ".yellow());
        } else if val == 4 {
            print!("{}", "⌬ ".blue());
        }
    }
    println!("{}", "|".blue());
    let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveToColumn(0));
}

fn clear_terminal() {
    let _ = std::io::stdout().queue(crossterm::terminal::Clear(terminal::ClearType::All));
    std::io::stdout()
        .queue(crossterm::cursor::MoveTo(0, 0))
        .unwrap();
}

pub fn render(tab: [[i8; SIZE_Y]; SIZE_X]) {
    clear_terminal();
    // Drawing map
    draw_empty_line();
    for i in 0..SIZE_Y {
        draw_line(tab, i);
    }
    draw_empty_line();
}
