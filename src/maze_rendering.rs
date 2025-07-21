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
        print!("{}", match val {
            0 => "☐ ".red(),
            1 => "  ".white(),
            2 => "● ".green(),
            3 => "☐ ".yellow(),
            4 => "⌬ ".blue(),
            5 => "■ ".red(),
            6 => "■ ".bright_red(),
            7 => "■ ".yellow(),
            8 => "■ ".bright_yellow(),
            9 => "☐ ".bright_yellow(),
            _ => "  ".white(),

        });
    }
    println!("{}", "|".blue());
    let _ = crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveToColumn(0));
}

fn draw_life_bar(life: i8) {
    print!("{}", "| ".blue());

    if life > 0 {
        // Life display
        for _ in 0..(SIZE_X * life as usize / 3) {
            print!("{}", "█ ".blue());
        }
        for _ in 0..(SIZE_X - (SIZE_X * life as usize / 3)) {
            print!("{}", "  ".blue());
        }
    } else {
        for _ in 0..(SIZE_X - 26 / 2 - ((SIZE_X - 26 / 2) * life as usize / 3)) / 2 {
            print!("{}", "  ".blue());
        }
        print!("{}", "Created by PeachKingHunter".blue());
        for _ in 0..(SIZE_X - 26 / 2 - ((SIZE_X - 26 / 2) * life as usize / 3)) / 2 {
            print!("{}", "  ".blue());
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

pub fn render(tab: [[i8; SIZE_Y]; SIZE_X], life: i8) {
    clear_terminal();
    // Drawing map
    draw_empty_line();
    for i in 0..SIZE_Y {
        draw_line(tab, i);
    }
    draw_empty_line();

    draw_life_bar(life);

    draw_empty_line();
}
