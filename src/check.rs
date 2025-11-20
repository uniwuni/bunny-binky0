use std::collections::HashSet;

use crate::data::*;

#[derive(Debug, Clone)]
pub enum VisitResult {
    AllVisitedIn(usize),
    Missing(HashSet<Pos>)
}

impl VisitResult {
    pub fn all_visited(&self) -> bool {
        match self {
            VisitResult::AllVisitedIn(_) => true,
            VisitResult::Missing(_) => false,
        }
    }
    pub fn to_hashset(self) -> Option<HashSet<Pos>> {
        match self {
            VisitResult::AllVisitedIn(_) => None,
            VisitResult::Missing(hash_set) => Some(hash_set),
        }
    }
    pub fn score(&self, new: &VisitResult) -> usize {
        match (self,new) {
            (VisitResult::AllVisitedIn(_), VisitResult::AllVisitedIn(_)) => 0,
            (VisitResult::AllVisitedIn(x), VisitResult::Missing(y)) => {println!("{:?}, {:?}",x,y); panic!("what")},
            (VisitResult::Missing(hash_set), VisitResult::AllVisitedIn(_)) => hash_set.len(),
            (VisitResult::Missing(hash_set), VisitResult::Missing(hash_set_new)) => (hash_set.difference(hash_set_new)).count(),
        }
    }
    fn merge(self, res: VisitResult) -> VisitResult {
        match (self,res) {
            (VisitResult::AllVisitedIn(n), VisitResult::AllVisitedIn(m)) => VisitResult::AllVisitedIn(n.max(m)),
            (VisitResult::AllVisitedIn(_), VisitResult::Missing(hash_set)) => VisitResult::Missing(hash_set),
            (VisitResult::Missing(hash_set), VisitResult::AllVisitedIn(_)) => VisitResult::Missing(hash_set),
            (VisitResult::Missing(mut hash_set), VisitResult::Missing(hash_set2)) => {
                hash_set.extend(hash_set2);
                VisitResult::Missing(hash_set)
            }
        }
    }
}

pub fn unvisited(field: &Field, relative: Option<HashSet<Pos>>, commands: &[Command], player: Player) -> (VisitResult, Player) {
    let mut unv = relative.unwrap_or(HashSet::new());
    let mut player = player;
    let mut steps = 0;
    let mut finalsteps = 0;
    for command in commands {
        if unv.is_empty() {
            finalsteps = steps;
        } else {
            unv.remove(&player.pos);
        }
        player = field.command(player, *command);
        steps += 1;

    }
    unv.remove(&player.pos);
    if unv.is_empty() {
            if finalsteps == 0 { finalsteps = steps; }
            return (VisitResult::AllVisitedIn(finalsteps), player);
    }
    (VisitResult::Missing(unv), player)
}

pub fn potential_unvisited(field: &Field, commands: &[Command], player: WeakPlayer) -> VisitResult {
    match player.dir {
        Some(dir) => unvisited(field, Some(field.clone().points), commands, Player {pos: player.pos, dir}).0,
        None => {
            let mut res = VisitResult::AllVisitedIn(0);
            for dir in [Direction::Down, Direction::Up, Direction::Left, Direction::Right] {
                res = res.merge(unvisited(field, Some(field.clone().points), commands, Player {pos: player.pos, dir}).0);
            }
            res
        }
    }
}
