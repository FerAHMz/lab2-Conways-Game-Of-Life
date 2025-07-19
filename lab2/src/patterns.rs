use crate::game_of_life::GameOfLife;

fn place_pattern(game: &mut GameOfLife, coords: &[(usize, usize)], offset_x: usize, offset_y: usize) {
    for &(x, y) in coords {
        game.set_alive(x + offset_x, y + offset_y);
    }
}

fn place_scaled(game: &mut GameOfLife, pattern: &[(usize, usize)], offset_x: usize, offset_y: usize, scale: usize) {
    for &(x, y) in pattern {
        let sx = x * scale;
        let sy = y * scale;
        for dx in 0..scale {
            for dy in 0..scale {
                game.set_alive(offset_x + sx + dx, offset_y + sy + dy);
            }
        }
    }
}

pub fn load_all_patterns(game: &mut GameOfLife) {
    let glider = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let block = [(0, 0), (1, 0), (0, 1), (1, 1)];
    let beacon = [(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)];
    let blinker = [(1, 0), (1, 1), (1, 2)];
    let toad = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    let lwss = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 2),
    ];
    let beehive = [(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)];
    let pulsar = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    let penta_decathlon = [
        (0, 2),
        (1, 0), (1, 1), (1, 2), (1, 3), (1, 4),
        (2, 2),
        (3, 0), (3, 1), (3, 2), (3, 3), (3, 4),
        (4, 2),
    ];
    let mwss = [
        (1,0), (2,0), (3,0), (4,0),
        (0,1), (4,1),
        (4,2),
        (0,3), (1,3), (2,3), (3,3), (4,3), (3,2),
    ];
    let glider_gun = [
        (24, 0),
        (22, 1), (24, 1),
        (12, 2), (13, 2), (20, 2), (21, 2), (34, 2), (35, 2),
        (11, 3), (15, 3), (20, 3), (21, 3), (34, 3), (35, 3),
        (0, 4), (1, 4), (10, 4), (16, 4), (20, 4), (21, 4),
        (0, 5), (1, 5), (10, 5), (14, 5), (16, 5), (17, 5), (22, 5), (24, 5),
        (10, 6), (16, 6), (24, 6),
        (11, 7), (15, 7),
        (12, 8), (13, 8),
    ];

    let patterns: Vec<&[(usize, usize)]> = vec![
        &glider, &block, &beacon, &blinker,
        &toad, &lwss, &beehive, &pulsar,
        &penta_decathlon, &mwss,
    ];

    let spacing = 14;
    let scale = 6;
    let rows = 7;
    let cols = 7;

    for row in 0..rows {
        for col in 0..cols {
            let pattern = patterns[(row * cols + col) % patterns.len()];
            let x = col * spacing * scale;
            let y = row * spacing * scale;
            place_scaled(game, pattern, x, y, scale);
        }
    }

    place_scaled(game, &glider_gun, 70, 150, scale);
}


pub fn load_glider(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    place_pattern(game, &pattern, x, y);
}

pub fn load_blinker(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(1, 0), (1, 1), (1, 2)];
    place_pattern(game, &pattern, x, y);
}

pub fn load_beacon(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)];
    place_pattern(game, &pattern, x, y);
}

pub fn load_toad(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    place_pattern(game, &pattern, x, y);
}

pub fn load_block(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];
    place_pattern(game, &pattern, x, y);
}

pub fn load_lwss(game: &mut GameOfLife, x: usize, y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 2),
    ];
    place_pattern(game, &pattern, x, y);
}
