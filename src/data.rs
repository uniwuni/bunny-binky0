use std::collections::HashSet;
use std::ops::Index;
use rand::{distr::StandardUniform, prelude::*};

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord, Hash)]
pub enum FieldType {
    Wall,
    Free
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord, Hash)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Distribution<Direction> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let decider: u8 = rng.sample(StandardUniform);
        match decider % 4 {
            0 => Direction::Right,
            1 => Direction::Up,
            2 => Direction::Left,
            3 => Direction::Down,
            _ => unreachable!("mod 4")
        }
    }
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord, Hash)]
pub enum Command {
    TurnLeft, TurnRight, Move
}

impl Direction {
    pub fn apply_command(self, command: Command) -> Direction {
        match command {
            Command::TurnLeft => match self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up
            }
            Command::TurnRight => match self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down
            }
            Command::Move => self,
        }
    }
    pub fn to_commands(self, new_dir: Direction) -> Vec<Command> {
        use Command::*;
        match (self,new_dir) {
            (Direction::Up, Direction::Up) => vec![],
            (Direction::Up, Direction::Down) => vec![TurnLeft, TurnLeft],
            (Direction::Up, Direction::Left) => vec![TurnLeft],
            (Direction::Up, Direction::Right) => vec![TurnRight],
            (Direction::Down, Direction::Up) => vec![TurnLeft, TurnLeft],
            (Direction::Down, Direction::Down) => vec![],
            (Direction::Down, Direction::Left) => vec![TurnRight],
            (Direction::Down, Direction::Right) => vec![TurnLeft],
            (Direction::Left, Direction::Up) => vec![TurnRight],
            (Direction::Left, Direction::Down) => vec![TurnLeft],
            (Direction::Left, Direction::Left) => vec![],
            (Direction::Left, Direction::Right) => vec![TurnRight, TurnRight],
            (Direction::Right, Direction::Up) => vec![TurnLeft],
            (Direction::Right, Direction::Down) => vec![TurnRight],
            (Direction::Right, Direction::Left) => vec![TurnRight, TurnRight],
            (Direction::Right, Direction::Right) => vec![],
        }
    }
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Pos(pub i32, pub i32);

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Field {
    pub width: u16,
    pub height: u16,
    pub points: HashSet<Pos>
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Player {
    pub pos: Pos,
    pub dir: Direction
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct WeakPlayer {
    pub pos: Pos,
    pub dir: Option<Direction>
}


impl Field {
    pub fn normalize(&self, Pos(x,y): Pos) -> Pos {
        Pos(x.rem_euclid(self.width as i32), y.rem_euclid(self.height as i32))
    }
}

impl Index<Pos> for Field {
    type Output = FieldType;

    fn index(&self, index: Pos) -> &Self::Output {
        if self.points.contains(&self.normalize(index)) {
            &FieldType::Free
        } else {
            &FieldType::Wall
        }
    }
}

impl Field {
    pub fn move_dir(&self, Pos(x,y): Pos, dir: Direction) -> Pos {
        let newpos: Pos = match dir {
            Direction::Up => Pos(x,y-1),
            Direction::Down => Pos(x,y+1),
            Direction::Left => Pos(x-1,y),
            Direction::Right => Pos(x+1,y)
        };
        if self[newpos] == FieldType::Free { self.normalize(newpos) } else { Pos(x,y) }
    }
    pub fn command(&self, player: Player, command: Command) -> Player {
        let dir = player.dir.apply_command(command);
        let pos =
            if command == Command::Move {
                self.move_dir(player.pos, player.dir)
            } else {
                player.pos
            };
        Player {pos, dir}
    }
}

#[derive(Clone, Copy,Debug,PartialEq,Eq,PartialOrd,Ord, Hash)]
pub enum TaskType {
    Check,
    Find
}

#[derive(Clone, Debug,PartialEq,Eq)]
pub enum Task {
    CheckTask { field: Field, commands: Vec<Command>, player: WeakPlayer },
    FindTask { field: Field, player: WeakPlayer }
}
