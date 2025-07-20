use std::{sync::mpsc::Sender, thread, time, usize};

use crossterm::{
    event::{read, Event, KeyCode},
    *,
};

mod maze_maker;
mod maze_manager;
mod maze_pathfiding;
mod maze_rendering;

pub const SIZE_X: usize = 31;
pub const SIZE_Y: usize = 41;

//Warning no verif on the pos putted here (3x3 arround basePos)
const BASE_X: usize = 15;
const BASE_Y: usize = 15;

const FRAME_PER_SEC: i8 = 60;
const FRAME_TIME: f64 = 1. / (FRAME_PER_SEC as f64);

fn main() -> std::io::Result<()> {
    // Terminal mode
    crossterm::terminal::enable_raw_mode()?;

    // Create maze
    let mut tab: [[i8; SIZE_Y]; SIZE_X] = [[0; SIZE_Y]; SIZE_X];
    crate::maze_maker::create_maze(&mut tab);

    // Inputs receiving
    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(|| {
        let _ = inputs_receiver(tx);
    });

    // Game variables
    let mut nb_frame: i8 = 0;

    let mut dir_x: i8 = 0;
    let mut dir_y: i8 = 1;

    let mut enemies_pathfinding: Vec<(usize, usize, Vec<(i8, i8)>)> =
        Vec::new();

    crate::maze_manager::create_core(&mut tab);
    crate::maze_manager::spawn_player(&mut tab);

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
        if nb_frame % (FRAME_PER_SEC) == 0 {
            // Spawn enemies
            let (pos_x, pos_y): (usize, usize) = crate::maze_manager::spawn_enemie(&mut tab);
            // Calculate pathfinding for this enemie
            enemies_pathfinding.push((
                pos_x,
                pos_y,
                crate::maze_pathfiding::pathfinding(pos_x, pos_y, tab),
            ));
        }

        if nb_frame % (FRAME_PER_SEC / 4) == 0 {
            crate::maze_manager::move_player(&mut tab, dir_x, dir_y);

            // Move all enemies
            for (px, py, l) in &mut enemies_pathfinding {
                if crate::maze_pathfiding::interprete_pathfinding(l, &mut tab, px, py) == true {
                    break 'gameLoop;
                }
            }
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
