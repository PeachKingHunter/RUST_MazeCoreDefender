use std::{collections::LinkedList, i8, process::exit, usize};

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

    let new_case = Case {
        pos_x: (parent.pos_x as i8 + dir_x) as usize,
        pos_y: (parent.pos_y as i8 + dir_y) as usize,
        price: parent.price + 1 + distance,
        parent_x: parent.pos_x,
        parent_y: parent.pos_y,
    };
    return new_case;
}

fn is_in_list(list: &LinkedList<Case>, pos_x: usize, pos_y: usize) -> bool {
    for case in list.iter() {
        if pos_x == case.pos_x && pos_y == case.pos_y {
            return true;
        }
    }
    return false;
}

fn get_parent(list: &LinkedList<Case>, pos_x: usize, pos_y: usize) -> Case {
    for case in list.iter() {
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

pub fn pathfinding(
    pos_x: usize,
    pos_y: usize,
    tab: [[i8; SIZE_Y]; SIZE_X],
) -> LinkedList<(i8, i8)> {
    // Needed vars
    let mut closed: LinkedList<Case> = LinkedList::new();
    let mut opened: LinkedList<Case> = LinkedList::new();

    let distance = 0;

    let new_case = Case {
        pos_x: pos_x,
        pos_y: pos_y,
        price: distance,
        parent_x: pos_x,
        parent_y: pos_y,
    };
    opened.push_front(new_case);

    loop {
        if let Some(case) = opened.pop_front() { // Temp line should take the lower price
            // Found a way to the core
            if case.pos_x == BASE_X && case.pos_y == BASE_Y {
                let mut movs: LinkedList<(i8, i8)> = LinkedList::new();
                let mut case = case;
                while case.pos_x != case.parent_x && case.pos_y != case.parent_y {
                    let dir_x = case.pos_x as i8 - case.parent_x as i8;
                    let dir_y = case.pos_y as i8 - case.parent_y as i8;
                    movs.push_front((dir_x, dir_y));

                    case = get_parent(&closed, case.parent_x, case.parent_y);
                }
                return movs;
            }

            // Searching
            let npos_x = case.pos_x + 1;
            let npos_y = case.pos_y;
            if available_pos(npos_x as i8, npos_y as i8) {
                if tab[npos_x][npos_y] != 0 {
                    if is_in_list(&closed, npos_x, npos_y) == false {
                        let new = create_case(1, 0, &case);
                        opened.push_front(new);
                    }
                }
            }

            let npos_x:i8 = case.pos_x as i8 - 1;
            let npos_y:i8 = case.pos_y as i8;
            if available_pos(npos_x, npos_y) {
                if tab[npos_x as usize][npos_y as usize] != 0 {
                    if is_in_list(&closed, npos_x as usize, npos_y as usize) == false {
                        let new = create_case(-1, 0, &case);
                        opened.push_front(new);
                    }
                }
            }

            let npos_x = case.pos_x;
            let npos_y = case.pos_y + 1;
            if available_pos(npos_x as i8, npos_y as i8) {
                if tab[npos_x][npos_y] != 0 {
                    if is_in_list(&closed, npos_x, npos_y) == false {
                        let new = create_case(0, 1, &case);
                        opened.push_front(new);
                    }
                }
            }

            let npos_x:i8 = case.pos_x as i8;
            let npos_y:i8 = case.pos_y as i8 - 1;
            if available_pos(npos_x, npos_y) {
                if tab[npos_x as usize][npos_y as usize] != 0 {
                    if is_in_list(&closed, npos_x as usize, npos_y as usize) == false {
                        let new = create_case(0, -1, &case);
                        opened.push_front(new);
                    }
                }
            }

            closed.push_front(case);
        } else {
            break;
        }
    }

    // No Move
    let mut movs: LinkedList<(i8, i8)> = LinkedList::new();
    return movs;
}

pub fn interprete_pathfinding(
    list: &mut LinkedList<(i8, i8)>,
    tab: &mut [[i8; SIZE_Y]; SIZE_X],
    pos_x: &mut usize,
    pos_y: &mut usize,
) {
    let mov = (*list).pop_front();
    match mov {
        Some((dir_x, dir_y)) => {
            let new_pos_x: i8 = *pos_x as i8 + dir_x;
            let new_pos_y: i8 = *pos_y as i8 + dir_y;
            if available_pos(new_pos_x, new_pos_y) {
                if tab[new_pos_x as usize][new_pos_y as usize] == 1 {
                    tab[*pos_x as usize][*pos_y as usize] = 1;
                    tab[new_pos_x as usize][new_pos_y as usize] = 3; // TODO put an higher number
                                                                     // but change it by three at the end before render for not move multiple time
                                                                     // an enemie

                    *pos_x = new_pos_x as usize;
                    *pos_y = new_pos_y as usize;
                }
            }
        }
        _ => println!("Empty pathfinding"),
    }
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
