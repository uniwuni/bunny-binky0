use rand::prelude::*;

use crate::data::*;
use crate::check::*;


pub fn generate_random_directions(n: usize) -> Vec<Direction> {
    let mut rng = rand::rng();
    let mut dirs= vec![];
    for _ in 0..n {
        let new_dir: Direction = rng.random();
        dirs.push(new_dir);
    }
    dirs
}


pub fn generate_commands(dirs: &Vec<Direction>, dir: Direction) -> Vec<Command> {
    let mut commands = Vec::with_capacity(2*dirs.len());
    let mut dir = dir;
    for new_dir in dirs {
        commands.extend(dir.to_commands(*new_dir));
        commands.push(Command::Move);
        dir = *new_dir;
    }
    commands
}

pub fn concatenate_strategies(field: &Field, player: WeakPlayer, block_size: usize, minimum_new_added: usize, max: usize) -> (Vec<Command>,Vec<Command>){
    let mut players: Vec<Player> = match player.dir {
        Some(dir) => vec![Player {pos: player.pos, dir}],
        None => vec![Player {pos: player.pos, dir: Direction::Right},
                 Player {pos: player.pos, dir: Direction::Up},
                 Player {pos: player.pos, dir: Direction::Left},
                 Player {pos: player.pos, dir: Direction::Down}],
    };
    let mut new_players = players.clone();
    let old_dir = players[0].dir;
    let mut dir_strategy: Vec<Direction> = Vec::with_capacity(40*block_size);
    let mut strategy: Vec<Command> = Vec::with_capacity(80*block_size);
    let mut results: Vec<VisitResult> =  match player.dir {
        Some(_) => vec![VisitResult::Missing(field.points.clone())],
        None => vec![VisitResult::Missing(field.points.clone()),
                                                    VisitResult::Missing(field.points.clone()),
                                                    VisitResult::Missing(field.points.clone()),
                                                    VisitResult::Missing(field.points.clone())]
    };

    let mut new_results = results.clone();

    while (&results).iter().any(|x| !x.all_visited()) && dir_strategy.len() < max {
        let mut commands;
        let mut directions;
        let mut block_size = block_size;

        'outer: loop {
        let real_attempts_before_increase = 8<<((2*block_size).min(11));
        for _ in 0..real_attempts_before_increase {
            directions = generate_random_directions(block_size);
            commands = generate_commands(&directions, players[0].dir);
            let mut score = 0;
            for i in 0..players.len() {
                let (res, new_player) = unvisited(field, results[i].clone().to_hashset(), &commands, players[i]);
                new_players[i] = new_player;
                score += results[i].score(&res);
                new_results[i] = res;

            }
            if score >= minimum_new_added || new_results.iter().all(|x| x.all_visited()) {
                break 'outer;
            }
        }

            block_size = (block_size * 3) / 2 + 1; // experimental. * 2 would seem reasonable but yields worse results

    }
        dir_strategy.extend(directions);
        strategy.extend(commands);

        results = new_results.clone();
        players = new_players.clone();

    }
    (generate_commands(&minimize_dirs(dir_strategy), old_dir), strategy)

}

// doesnt always work because if there is a wall to your left, LRL adds another field to your visited list
pub fn minimize_dirs(dirs: Vec<Direction>) -> Vec<Direction> {
    let mut i = 0;
    let mut newdirs = vec![];
    while i < dirs.len() {
       match (dirs[i], dirs.get(i+1), dirs.get(i+2)) {
           (Direction::Left,Some(Direction::Right), Some(Direction::Left)) => {newdirs.push(dirs[i]); i+=3;},
           (Direction::Right,Some(Direction::Left), Some(Direction::Right)) => {newdirs.push(dirs[i]); i+=3;},
           (Direction::Up,Some(Direction::Down), Some(Direction::Up)) => {newdirs.push(dirs[i]); i+=3;},
           (Direction::Down,Some(Direction::Up), Some(Direction::Down)) => {newdirs.push(dirs[i]); i+=3;},
           _ => {newdirs.push(dirs[i]); i += 1}
       }
    }
    newdirs
}
