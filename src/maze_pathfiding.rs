use std::{i8, usize};

use crate::{BASE_X, BASE_Y, SIZE_X, SIZE_Y};

#[derive(Clone)]
struct Case {
    pos_x: usize,
    pos_y: usize,
    price: i64,
    parent_x: usize,
    parent_y: usize,
}

fn create_case(dir_x: i8, dir_y: i8, parent: &Case) -> Case {
    let disx1 = BASE_X as i64 - parent.pos_x as i64;
    let disx2 = BASE_X as i64 - parent.pos_x as i64 - dir_x as i64;
    let disy1 = BASE_Y as i64 - parent.pos_y as i64;
    let disy2 = BASE_Y as i64 - parent.pos_y as i64 - dir_y as i64;
    let distance = disy2.abs() - disy1.abs() + disx2.abs() - disx1.abs();

    let npos_x = parent.pos_x as i8 + dir_x;
    let npos_y = parent.pos_y as i8 + dir_y;

    let new_case = Case {
        pos_x: npos_x as usize,
        pos_y: npos_y as usize,
        price: parent.price + 1 + distance,
        parent_x: parent.pos_x,
        parent_y: parent.pos_y,
    };
    return new_case;
}

fn is_in_list(list: &Vec<Case>, pos_x: usize, pos_y: usize) -> bool {
    for case in list {
        if pos_x == case.pos_x && pos_y == case.pos_y {
            return true;
        }
    }
    return false;
}

fn get_parent(list: &Vec<Case>, pos_x: usize, pos_y: usize) -> Case {
    for case in list {
        if pos_x == case.pos_x && pos_y == case.pos_y {
            return case.clone();
        }
    }
    Case {
        pos_x: pos_x,
        pos_y: pos_y,
        price: 0,
        parent_x: pos_x,
        parent_y: pos_y,
    }
}

fn get_lower_price(list: &mut Vec<Case>) -> Option<Case> {
    let mut smallest_price: i64 = i64::MAX;
    let mut res_index: i8 = -1;
    for i in 0..list.len() {
        let case: Option<&Case> = list.get(i);
        match case {
            Some(case) => {
                if case.price <= smallest_price {
                    smallest_price = case.price;
                    res_index = i as i8;
                }
            }

            None => {}
        }
    }

    if res_index == -1 {
        return None;
    }

    return Some(list.remove(res_index as usize));
}

pub fn pathfinding(
    default_pos_x: usize,
    default_pos_y: usize,
    tab: [[i8; SIZE_Y]; SIZE_X],
) -> Vec<(i8, i8)> {
    // Needed vars
    let mut closed: Vec<Case> = Vec::new();
    let mut opened: Vec<Case> = Vec::new();

    let distance = 0;

    let new_case = Case {
        pos_x: default_pos_x,
        pos_y: default_pos_y,
        price: distance,
        parent_x: default_pos_x,
        parent_y: default_pos_y,
    };
    opened.push(new_case);

    loop {
        let case: Case = match get_lower_price(&mut opened) {
            Some(c) => c,
            _ => break,
        };

        // Temp line should take the lower price
        // Found a way to the core
        if case.pos_x == BASE_X && case.pos_y == BASE_Y {
            let mut movs: Vec<(i8, i8)> = Vec::new();
            let mut case = case;
            while case.pos_x != default_pos_x || case.pos_y != default_pos_y {
                let dir_x = case.pos_x as i8 - case.parent_x as i8;
                let dir_y = case.pos_y as i8 - case.parent_y as i8;
                movs.push((dir_x, dir_y));

                case = get_parent(&closed, case.parent_x, case.parent_y);
            }
            return movs;
        }

        // Searching
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        for (dir_x, dir_y) in directions {
            let npos_x = case.pos_x as i8 + dir_x;
            let npos_y = case.pos_y as i8 + dir_y;

            if available_pos(npos_x, npos_y) {
                if tab[npos_x as usize][npos_y as usize] != 0
                    && !is_in_list(&closed, npos_x as usize, npos_y as usize)
                    && !is_in_list(&opened, npos_x as usize, npos_y as usize)
                {
                    let new_case = create_case(dir_x as i8, dir_y as i8, &case);
                    opened.push(new_case);
                }
            }
        }

        closed.push(case);
    }

    // No Move
    let movs: Vec<(i8, i8)> = Vec::new();
    return movs;
}

pub fn interprete_pathfinding(
    list: &mut Vec<(i8, i8)>,
    tab: &mut [[i8; SIZE_Y]; SIZE_X],
    pos_x: &mut usize,
    pos_y: &mut usize,
) -> bool {
    let mov = (*list).pop();
    match mov {
        Some((dir_x, dir_y)) => {
            let new_pos_x: i8 = *pos_x as i8 + dir_x;
            let new_pos_y: i8 = *pos_y as i8 + dir_y;
            if available_pos(new_pos_x, new_pos_y) {
                if tab[new_pos_x as usize][new_pos_y as usize] != 0 {

                    //if tab[*pos_x as usize][*pos_y as usize] != 2 {
                        tab[*pos_x as usize][*pos_y as usize] = 1;
                    //}
                    if tab[new_pos_x as usize][new_pos_y as usize] != 2 {
                        tab[new_pos_x as usize][new_pos_y as usize] = 3;
                    }

                    *pos_x = new_pos_x as usize;
                    *pos_y = new_pos_y as usize;
                } else if tab[new_pos_x as usize][new_pos_y as usize] == 4 {
                    // Lose
                    return true;
                }
            }
        }
        _ => {}, //println!("Empty pathfinding"),
    }
    return false;
}

fn available_pos(pos_x: i8, pos_y: i8) -> bool {
    if pos_x < 0 || pos_x >= SIZE_X as i8 {
        return false;
    }

    // Tab limit y
    if pos_y < 0 || pos_y >= SIZE_Y as i8 {
        return false;
    }

    return true;
}
