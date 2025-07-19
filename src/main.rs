use std::{sync::mpsc::Sender, thread, time};

use crossterm::{
    event::{read, Event, KeyCode},
    *,
};

mod maze_maker;
mod maze_manager;
mod maze_rendering;

pub const SIZE_X: usize = 31;
pub const SIZE_Y: usize = 41;

const BASE_X: usize = 15;
const BASE_Y: usize = 15;

const FRAME_PER_SEC: i8 = 60;
const FRAME_TIME: f64 = 1. / (FRAME_PER_SEC as f64);

fn get_player_pos(tab: [[i8; SIZE_Y]; SIZE_X]) -> (i8, i8) {
    for x in 0..SIZE_X - 1 {
        for y in 0..SIZE_Y - 1 {
            if tab[x][y] == 2 {
                return (x as i8, y as i8);
            }
        }
    }
    return (-1, -1);
}

fn move_player(tab: &mut [[i8; SIZE_Y]; SIZE_X], move_x: i8, move_y: i8) {
    let (pos_x, pos_y) = get_player_pos(*tab);

    // Tab limit x
    if pos_x + move_x < 0 || (pos_x + move_x) as usize >= SIZE_X {
        return;
    }

    // Tab limit y
    if pos_y + move_y < 0 || (pos_y + move_y) as usize >= SIZE_Y {
        return;
    }

    // Player valid pos
    if pos_x == -1 && pos_y == -1 {
        return;
    }

    // Move in direction if no wall
    if tab[(pos_x + move_x) as usize][(pos_y + move_y) as usize] == 1 {
        tab[pos_x as usize][pos_y as usize] = 1;
        tab[(pos_x + move_x) as usize][(pos_y + move_y) as usize] = 2;
    }
}

fn spawn_enemie(tab: &mut [[i8; SIZE_Y]; SIZE_X]) {}

fn main() -> std::io::Result<()> {
    // Terminal mode
    crossterm::terminal::enable_raw_mode()?;

    // Create maze
    let mut tab: [[i8; SIZE_Y]; SIZE_X] = [[0; SIZE_Y]; SIZE_X];
    crate::maze_maker::create_maze(&mut tab);
    // Temp pacman
    tab[10][10] = 2;

    // Inputs receiving
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(|| {
        let _ = inputs_receiver(tx);
    });

    // Game variables
    let mut nb_frame: i8 = 0;

    let mut dir_x: i8 = 0;
    let mut dir_y: i8 = 1;

    // Game loop
    'gameLoop: loop {
        nb_frame = (nb_frame + 1) % FRAME_PER_SEC;

        // Managing inputs
        if let Ok(input) = rx.try_recv() {
            if event::KeyCode::is_esc(&input) == true {
                break 'gameLoop;
            }
            if input == event::KeyCode::Char('q') {
                dir_x = -1;
                dir_y = 0;
            }
            if input == event::KeyCode::Char('d') {
                dir_x = 1;
                dir_y = 0;
            }
            if input == event::KeyCode::Char('z') {
                dir_x = 0;
                dir_y = -1;
            }
            if input == event::KeyCode::Char('s') {
                dir_x = 0;
                dir_y = 1;
            }
        }

        // Update & Render
        if nb_frame % (FRAME_PER_SEC / 4) == 0 {
            move_player(&mut tab, dir_x, dir_y);
            // Temp spawn speed for testing
            spawn_enemie(&mut tab);
        }
        crate::maze_rendering::render(tab);

        thread::sleep(time::Duration::from_millis((FRAME_TIME * 1000.) as u64));
        // Not Exact -> should be remake
    }
    let _ = crossterm::terminal::disable_raw_mode();
    Ok(())
}

fn inputs_receiver(tx: Sender<KeyCode>) -> std::io::Result<()> {
    loop {
        if let Ok(_) = crossterm::event::poll(time::Duration::from_millis(20)) {
            let event = read()?;
            match event {
                Event::Key(event) => {
                    let _ = Event::Key(event);
                    let _ = tx.send(event.code);
                }
                _ => {}
            }
        }
    }
}
