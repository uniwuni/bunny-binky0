mod check;
mod data;
mod parse;
mod find;

use std::{
    fs, fs::File, path::Path,
    time,
    io::{self, prelude::*, BufReader},
};

use data::Direction;

fn eval<P>(path: P) -> (String,usize)
where
    P: AsRef<Path> + std::fmt::Debug, {
    println!("{:?}", path);
    let lines_: io::Result<Vec<_>> =
            BufReader::new(File::open(path).unwrap())
        .lines()
        .collect();
    let lines = lines_.unwrap();
    let task = parse::parse(lines);
    match task {
        data::Task::CheckTask {
            field,
            commands,
            player,
        } => {
            match check::potential_unvisited(&field, &commands, player) {
                check::VisitResult::AllVisitedIn(n) => {eprintln!("{} steps",n); ("GOOD PLAN\n".to_string(),0)},
                check::VisitResult::Missing(hash_set) => {
                    let mut s: String = "BAD PLAN\n".to_string();
                    for data::Pos(x,y) in hash_set {
                        s += &format!("{}, {}\n", x, y);
                    }
                    eprintln!("{}",s);
                    (s,0)
                }
            }
        }
        ,
        data::Task::FindTask { player, field } => {
            eprintln!("VISIT TEST");
            let commands = find::concatenate_strategies(&field, player, 1, 1, 10000);
            match check::potential_unvisited(&field, &commands, player) {
                check::VisitResult::AllVisitedIn(n) => {println!("{} steps",n); ("GOOD PLAN\n".to_string(),n)},
                check::VisitResult::Missing(hash_set) => {
                    let mut s: String = "BAD PLAN\n".to_string();
                    eprintln!("joa scheiße1");
                    for data::Pos(x,y) in hash_set {
                        s += &format!("{}, {}\n", x, y);
                    }
                    ("".to_string(),2032903)
                }
            }
        },
    }
}

fn main() {
/*    let mut dirs1 = vec![Direction::Left, Direction::Right, Direction::Right];
    let dirs2 = vec![Direction::Left, Direction::Right];
    eprintln!("{:?}\n", find::generate_commands(&dirs1, Direction::Right));

    dirs1.extend(dirs2);
    eprintln!("{:?}\n", find::generate_commands(&dirs1, Direction::Right));
    panic!();*/
    let paths = fs::read_dir("/home/uni/workspace/resources/ai1sysproj/assignment0/assignment/example-problems").unwrap();
    let now = time::Instant::now();
    let mut sum = 0;
    let mut max_ = 0;
    for filename in paths {
    let n = eval(filename.expect("path io error!").path()).1;
    max_ = max_.max(n);
    sum +=n;

    }
    eprintln!("{} steps total, {} max", sum, max_);
    eprintln!("Elapsed total: {:.2?}", now.elapsed());
}
