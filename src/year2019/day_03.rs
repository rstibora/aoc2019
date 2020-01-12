use std::collections::HashMap;

pub fn first_star(input: &Vec<String>) -> String {
    let wire_a = path_to_grid(parse_path_input(&input[0]));
    let wire_b = path_to_grid(parse_path_input(&input[1]));
    let intersection_grid = intersect_grids(wire_a, wire_b);

    let mut closest_intersection = std::i32::MAX;
    for key in intersection_grid.keys() {
        if key == &(0, 0) {
            continue;
        }
        closest_intersection = std::cmp::min(closest_intersection, manhattan_distance(&(0, 0), key));
    }
    return closest_intersection.to_string();
}

pub fn second_star(input: &Vec<String>) -> String {
    let wire_a = path_to_grid(parse_path_input(&input[0]));
    let wire_b = path_to_grid(parse_path_input(&input[1]));
    let intersection_grid = intersect_grids(wire_a, wire_b);

    let mut closest_intersection: Option<((i32, i32), u32)> = None;
    for (position, (steps_a, steps_b)) in intersection_grid {
        if position == (0, 0) {
            continue;
        }

        let steps_combined = steps_a + steps_b;
        if let Some((_, best_steps)) = closest_intersection {
            if steps_combined < best_steps {
                closest_intersection = Some((position, steps_combined));
            }
        } else {
            closest_intersection = Some((position, steps_combined));
        }
    }

    if let Some((_, num_steps)) = closest_intersection {
        return num_steps.to_string();
    } else {
        return String::from("N/A");
    }
}

fn path_to_grid(path: Vec<(char, i32)>) -> HashMap<(i32, i32), u32> {
    let mut grid = HashMap::new();
    let mut current_position = (0, 0);
    let mut total_distance = 0;

    grid.insert(current_position, total_distance);
    for (instruction, distance) in path {
        for _ in 0..distance {
            match instruction {
                'R' => current_position.0 += 1,
                'L' => current_position.0 -= 1,
                'U' => current_position.1 += 1,
                'D' => current_position.1 -= 1,
                _ => panic!()
            }
            total_distance +=1;
            grid.entry(current_position).or_insert(total_distance);
        }
    }
    grid
}

fn intersect_grids(grid_a: HashMap<(i32, i32), u32>, grid_b: HashMap<(i32, i32), u32>) -> HashMap<(i32, i32), (u32, u32)> {
    let mut intersection_grid = HashMap::new();
    for key in grid_a.keys() {
        if grid_b.contains_key(key) {
            intersection_grid.insert(*key, (grid_a[key], grid_b[key]));
        }
    }
    intersection_grid
}

fn manhattan_distance(point_a: &(i32, i32), point_b: &(i32, i32)) -> i32 {
    i32::abs(point_b.0 - point_a.0) + i32::abs(point_b.1 - point_a.1)
}

fn parse_path_input(input: &String) -> Vec<(char, i32)> {
    let splits = input.split(",");
    let mut output: Vec<(char, i32)> = Vec::new();
    for split in splits {
        let mut chars = split.chars();
        let direction = chars.next().unwrap();
        let distance: String = chars.collect();
        let distance: i32 = distance.parse().unwrap();
        output.push((direction, distance));
    }
    output
}
