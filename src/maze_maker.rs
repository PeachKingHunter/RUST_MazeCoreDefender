use crate::SIZE_X;
use crate::SIZE_Y;

fn is_accessible_ok(tab: &mut [[i8; SIZE_Y]; SIZE_X], pos_x: usize, pos_y: usize) -> bool {
    // Is this case in the maze have an way to go to
    if pos_x + 1 < SIZE_X {
        if tab[pos_x + 1][pos_y] == 1 {
            return false;
        }
    }

    if pos_x - 1 < SIZE_X {
        if tab[pos_x - 1][pos_y] == 1 {
            return false;
        }
    }

    if pos_y + 1 < SIZE_Y {
        if tab[pos_x][pos_y + 1] == 1 {
            return false;
        }
    }

    if pos_y - 1 < SIZE_Y {
        if tab[pos_x][pos_y - 1] == 1 {
            return false;
        }
    }

    return true;
}

fn is_direction_ok(tab: &mut [[i8; SIZE_Y]; SIZE_X], pos_x: usize, pos_y: usize, rdm: i8) -> bool {
    // If can go to this direction
    match rdm {
        0 => {
            if pos_x + 2 >= SIZE_X {
                return false;
            }
            if tab[pos_x + 1][pos_y] == 1 {
                return false;
            }
            if is_accessible_ok(tab, pos_x + 2, pos_y) == false {
                return false;
            }

            return true;
        }
        1 => {
            if pos_y + 2 >= SIZE_Y {
                return false;
            }
            if tab[pos_x][pos_y + 1] == 1 {
                return false;
            }
            if is_accessible_ok(tab, pos_x, pos_y + 2) == false {
                return false;
            }
            return true;
        }
        2 => {
            if (pos_x as i8 - 2) <= 0 {
                return false;
            }
            if tab[pos_x - 1][pos_y] == 1 {
                return false;
            }
            if is_accessible_ok(tab, pos_x - 2, pos_y) == false {
                return false;
            }
            return true;
        }
        3 => {
            if (pos_y as i8 - 2) <= 0 {
                return false;
            }
            if tab[pos_x][pos_y - 1] == 1 {
                return false;
            }
            if is_accessible_ok(tab, pos_x, pos_y - 2) == false {
                return false;
            }
            return true;
        }
        _ => eprintln!("Error is direction ok"),
    }
    return false;
}

fn create_ways(tab: &mut [[i8; SIZE_Y]; SIZE_X], pos_x: usize, pos_y: usize) {
    // Create the way recursively
    let mut rdm: i8 = rand::random_range(0..4);
    for i in 0..4 {
        if is_direction_ok(tab, pos_x, pos_y, (rdm + i) % 4) == true {
            rdm = (rdm + i) % 4;
            match rdm {
                0 => {
                    tab[pos_x + 1][pos_y] = 1;
                    create_ways(tab, pos_x + 2, pos_y);
                }
                1 => {
                    tab[pos_x][pos_y + 1] = 1;
                    create_ways(tab, pos_x, pos_y + 2);
                }
                2 => {
                    tab[pos_x - 1][pos_y] = 1;
                    create_ways(tab, pos_x - 2, pos_y);
                }
                3 => {
                    tab[pos_x][pos_y - 1] = 1;
                    create_ways(tab, pos_x, pos_y - 2);
                }
                _ => eprintln!("Error create ways"),
            }
        }
    }
}

pub fn create_maze(tab: &mut [[i8; SIZE_Y]; SIZE_X]) {
    // Create empty maze
    for y in 0..SIZE_Y {
        if (y % 2) == 0 {
            continue;
        }
        for x in 0..SIZE_X {
            if (x % 2) == 0 {
                continue;
            }
            tab[x][y] = 1;
        }
    }

    // Start at random pos
    let mut rdm: usize = rand::random_range(0..SIZE_X - 1);
    let mut pos_x: usize = rdm;
    rdm = rand::random_range(0..SIZE_Y - 1);
    let mut pos_y: usize = rdm;

    if (pos_x % 2) == 0 {
        pos_x += 1;
    }
    if (pos_y % 2) == 0 {
        pos_y += 1;
    }

    // Make the way
    create_ways(tab, pos_x, pos_y);

    // Create imperfections
    let imperfection: i8 = 20;
    for y in 1..SIZE_Y - 1 {
        for x in 1..SIZE_X - 1 {
            if tab[x][y] == 0 {
                if imperfection > rand::random_range(0..100) {
                    tab[x][y] = 1;
                }
            }
        }
    }
}
