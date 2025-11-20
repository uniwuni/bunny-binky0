use std::collections::HashSet;

use crate::data::*;


fn parse_commands(cmdline: &str) -> Vec<Command> {
    cmdline
        .chars()
        .map(|c| match c {
            'L' => Command::TurnLeft,
            'R' => Command::TurnRight,
            'M' => Command::Move,
            _ => panic!("invalid command line"),
        })
        .collect()
}

fn parse_field(field_lines: &[String]) -> (Field, WeakPlayer) {
    let height = field_lines.len();
    if height == 0 {
        panic!("empty field");
    }
    let width = field_lines[0].len();
    let mut points: HashSet<Pos> = HashSet::new();
    let mut player: Option<WeakPlayer> = None;
    for iy in 0..height {
        for ix in 0..width {
            let x = ix as i32;
            let y = iy as i32;
            match field_lines[iy].as_bytes()[ix] as char {
                'X' => (), // noop
                ' ' => {
                    points.insert(Pos(x, y));
                }
                '<' => {
                    points.insert(Pos(x, y));
                    player = Some(WeakPlayer {
                        pos: Pos(x, y),
                        dir: Some(Direction::Left),
                    })
                }
                '>' => {
                    points.insert(Pos(x, y));
                    player = Some(WeakPlayer {
                        pos: Pos(x, y),
                        dir: Some(Direction::Right),
                    })
                }
                '^' => {
                    points.insert(Pos(x, y));
                    player = Some(WeakPlayer {
                        pos: Pos(x, y),
                        dir: Some(Direction::Up),
                    })
                }
                'v' => {
                    points.insert(Pos(x, y));
                    player = Some(WeakPlayer {
                        pos: Pos(x, y),
                        dir: Some(Direction::Down),
                    })
                }
                'S' => {
                    points.insert(Pos(x, y));
                    player = Some(WeakPlayer {
                        pos: Pos(x, y),
                        dir: None,
                    })
                }
                _ => panic!("invalid code point in map"),
            }
        }
    }
    (
        Field {
            width: width.try_into().expect("width too high"),
            height: height.try_into().expect("height too high"),
            points,
        },
        player.expect("no player position"),
    )
}

fn get_task_type(first_line: &str) -> TaskType {
    if first_line == "CHECK PLAN" {
        TaskType::Check
    } else if first_line == "FIND PLAN" {
        TaskType::Find
    } else {
        panic!("invalid task type")
    }
}

pub fn parse(lines: Vec<String>) -> Task {
    match get_task_type(&lines[0]) {
        TaskType::Check => {
            let commands = parse_commands(&lines[1]);
            let (field, player) = parse_field(&lines[2..]);
            Task::CheckTask { field, commands, player }
        }
        TaskType::Find => {
            let (field, player) = parse_field(&lines[1..]);
            Task::FindTask { field, player }
        },
    }

}
