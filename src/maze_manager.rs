use std::usize;

use crate::BASE_X;
use crate::BASE_Y;
use crate::SIZE_X;
use crate::SIZE_Y;

pub fn get_player_pos(tab: [[i8; SIZE_Y]; SIZE_X]) -> (i8, i8) {
    for x in 0..SIZE_X - 1 {
        for y in 0..SIZE_Y - 1 {
            if tab[x][y] == 2 {
                return (x as i8, y as i8);
            }
        }
    }
    return (-1, -1);
}

pub fn verif_tab_limits(pos_x: i8, pos_y: i8) -> bool {
    // Tab limit x
    if pos_x < 0 || pos_x >= SIZE_X as i8 {
        return false;
    }

    // Tab limit y
    if pos_y < 0 || pos_y >= SIZE_Y as i8 {
        return false;
    }

    return true;
}

pub fn move_player(tab: &mut [[i8; SIZE_Y]; SIZE_X], move_x: i8, move_y: i8) {
    let (pos_x, pos_y) = get_player_pos(*tab);

    if verif_tab_limits(pos_x + move_x, pos_y + move_y) == false {
        return;
    }

    // Player valid pos
    if pos_x == -1 && pos_y == -1 {
        return;
    }

    // Move in direction if no wall
    if tab[(pos_x + move_x) as usize][(pos_y + move_y) as usize] == 1
        || tab[(pos_x + move_x) as usize][(pos_y + move_y) as usize] == 3
    {
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
        _ => eprintln!("ERROR spawn enemie"),
    }
    (0, 0)
}

pub fn delete_enemy(
    mut list: Vec<(usize, usize, Vec<(i8, i8)>)>,
    pos_x: i8,
    pos_y: i8,
) -> Vec<(usize, usize, Vec<(i8, i8)>)> {
    let mut i: usize = 0;
    let mut to_remove: bool = false;
    for (px, py, _) in &list {
        if *px as i8 == pos_x && *py as i8 == pos_y {
            to_remove = true;
            break;
        }
        i += 1;
    }
    if to_remove == true {
        let _ = list.remove(i);
    }
    list
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

pub fn core_explosion(tab: &mut [[i8; SIZE_Y]; SIZE_X]) {
    tab[BASE_X][BASE_Y] = 5;

    // No tab limit verif to do because start at 1 to size-1
    for x in 1..(SIZE_X - 1) {
        for y in 1..(SIZE_Y - 1) {
            let rdme:i8 = rand::random_range(0..2);
            if rdme == 0 {
                continue;
            }


            if tab[x][y] >= 5 && tab[x][y] < 40 {
                let mut new_val: i8 = tab[x][y];
                
                let mut rdm:i8 = rand::random_range(0..=8);
                if rdm == 0 {
                    if new_val < 30 {
                        new_val += 1;
                    }
                }

                rdm = rand::random_range(0..=2);
                if rdm == 0{
                    tab[x][y] += 1;
                }

                new_val += 50;
                if tab[x + 1][y] < 5 {
                    tab[x + 1][y] = new_val;
                }
                if tab[x - 1][y] < 5 {
                    tab[x - 1][y] = new_val;
                }
                if tab[x][y + 1] < 5 {
                    tab[x][y + 1] = new_val;
                }
                if tab[x][y - 1] < 5 {
                    tab[x][y - 1] = new_val;
                }
            }
        }
    }
    for x in 0..(SIZE_X - 0) {
        for y in 0..(SIZE_Y - 0) {
            if tab[x][y] > 40 {
                tab[x][y] -= 50;
            }
        }
    }

    tab[BASE_X][BASE_Y] = 5;
}
