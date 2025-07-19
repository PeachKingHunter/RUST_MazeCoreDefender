use std::usize;

use crate::BASE_X;
use crate::BASE_Y;
use crate::SIZE_X;
use crate::SIZE_Y;

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

pub fn move_player(tab: &mut [[i8; SIZE_Y]; SIZE_X], move_x: i8, move_y: i8) {
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

pub fn spawn_enemie(tab: &mut [[i8; SIZE_Y]; SIZE_X]) -> (usize, usize) {
    let mut rdm: usize = rand::random_range(0..=3);
    match rdm {
        0 => {
            rdm = rand::random_range(0..(SIZE_X as i8)) as usize;
            tab[rdm][0] = 3;
            return (rdm, 0);
        }
        1 => {
            rdm = rand::random_range(0..(SIZE_X as i8)) as usize;
            tab[rdm][SIZE_Y - 1] = 3;
            return (rdm, SIZE_Y - 1);
        }
        2 => {
            rdm = rand::random_range(0..(SIZE_Y as i8)) as usize;
            tab[0][rdm] = 3;
            return (0, rdm);
        }
        3 => {
            rdm = rand::random_range(0..(SIZE_Y as i8)) as usize;
            tab[SIZE_X - 1][rdm] = 3;
            return (SIZE_X - 1, rdm);
        }
        _ => println!("ERROR spawn enemie"),
    }
    (0, 0)
}

pub fn create_core(tab: &mut [[i8; SIZE_Y]; SIZE_X]) {
    tab[BASE_X][BASE_Y] = 4;

    tab[BASE_X - 1][BASE_Y - 1] = 1;
    tab[BASE_X][BASE_Y - 1] = 1;
    tab[BASE_X + 1][BASE_Y - 1] = 1;

    tab[BASE_X - 1][BASE_Y] = 1;
    tab[BASE_X + 1][BASE_Y] = 1;

    tab[BASE_X - 1][BASE_Y + 1] = 1;
    tab[BASE_X][BASE_Y + 1] = 1;
    tab[BASE_X + 1][BASE_Y + 1] = 1;
}

pub fn spawn_player(tab: &mut [[i8; SIZE_Y]; SIZE_X]) {
    tab[BASE_X][BASE_Y - 1] = 2;
}
