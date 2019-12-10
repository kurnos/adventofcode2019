use num::rational::Rational32;
use std::cmp::PartialOrd;
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;

fn parse_field(contents: &str) -> Vec<Vec<bool>> {
    let mut res = Vec::new();
    for x in contents.lines() {
        let mut line = Vec::new();
        for b in x.chars() {
            line.push(if b == '#' { true } else { false });
        }
        res.push(line);
    }
    res
}

pub fn first(contents: &String) -> usize {
    let field = parse_field(contents);

    let mut counts: HashMap<(i32, i32), usize> = HashMap::new();
    for (from_x, from_y) in iproduct!(0..field.len() as i32, 0..field.len() as i32) {
        let mut directions: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let mut seen: HashSet<(i32, i32)> = HashSet::new();
        if field[from_x as usize][from_y as usize] == false {
            continue;
        }
        for (to_x, to_y) in iproduct!(0..field.len() as i32, 0..field.len() as i32) {
            if field[to_x as usize][to_y as usize] == false || to_y == from_y {
                continue;
            }
            let stride = Rational32::new(from_x - to_x, from_y - to_y);
            let (&dx, &dy) = (stride.numer(), stride.denom());

            if directions.contains(&(from_x, from_y, dx, dy)) {
                continue;
            }
            directions.insert((from_x, from_y, dx, dy));

            let mut x = from_x + dx;
            let mut y = from_y + dy;

            while 0 <= x && x < field.len() as i32 && 0 <= y && y < field.len() as i32 {
                if field[x as usize][y as usize] {
                    seen.insert((x, y));
                    break;
                }
                x += dx;
                y += dy;
            }
            x = from_x - dx;
            y = from_y - dy;
            while 0 <= x && x < field.len() as i32 && 0 <= y && y < field.len() as i32 {
                if field[x as usize][y as usize] {
                    seen.insert((x, y));
                    break;
                }
                x -= dx;
                y -= dy;
            }
        }
        let mut x = from_x - 1;
        let y = from_y;
        while 0 <= x && x < field.len() as i32 && 0 <= y && y < field.len() as i32 {
            if field[x as usize][y as usize] {
                seen.insert((x, y));
                break;
            }
            x -= 1;
        }

        let mut x = from_x + 1;
        while 0 <= x && x < field.len() as i32 && 0 <= y && y < field.len() as i32 {
            if field[x as usize][y as usize] {
                seen.insert((x, y));
                break;
            }
            x += 1;
        }
        counts.insert((from_x, from_y), seen.len());
    }
    let (_, &c) = counts.iter().max_by_key(|(_, &c)| c).unwrap();
    c
}

pub fn second(contents: &String) -> i32 {
    let mut field = parse_field(contents);

    let (from_x, from_y) = (19, 23);

    let mut directions: HashSet<(i32, i32)> = HashSet::new();
    for (to_x, to_y) in iproduct!(0..field.len() as i32, 0..field.len() as i32) {
        if field[to_x as usize][to_y as usize] == false {
            continue;
        }
        let (dx, dy) = if to_y == from_y {
            (1, 0)
        } else {
            let stride = Rational32::new(from_x - to_x, from_y - to_y);
            (*stride.numer(), *stride.denom())
        };

        directions.insert((dx, dy));
        directions.insert((-dx, -dy));
    }

    let mut asdf = Vec::new();
    asdf.extend(directions.into_iter());

    asdf.sort_by(|(ax, ay), (bx, by)| {
        (*ax as f32)
            .atan2(*ay as f32)
            .partial_cmp(&(*bx as f32).atan2(*by as f32))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let offset = asdf.iter().position(|&x| x == (-1, 0)).unwrap();
    asdf.rotate_left(offset);

    let mut blasted: Vec<(i32, i32)> = Vec::new();

    for (dx, dy) in Iterator::cycle(asdf.into_iter()) {
        let mut x = from_x + dx;
        let mut y = from_y + dy;

        while 0 <= x && x < field.len() as i32 && 0 <= y && y < field.len() as i32 {
            if field[x as usize][y as usize] {
                field[x as usize][y as usize] = false;
                blasted.push((x, y));
                break;
            }
            x += dx;
            y += dy;
        }
        if blasted.len() >= 200 {
            break;
        }
    }

    let (x, y) = blasted.last().unwrap();
    y * 100 + x
}
