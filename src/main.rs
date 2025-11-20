mod check;
mod data;
mod find;
mod parse;

use std::{
    fs,
    fs::File,
    io::{self, prelude::*, BufReader},
    path::Path,
    time,
};

use data::display_commands;

fn eval<P>(path: P) -> (String, usize)
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let lines_: io::Result<Vec<_>> = BufReader::new(File::open(path).unwrap()).lines().collect();
    let lines = lines_.unwrap();
    let task = parse::parse(lines);
    match task {
        data::Task::CheckTask {
            field,
            commands,
            player,
        } => match check::potential_unvisited(&field, &commands, player) {
            check::VisitResult::AllVisitedIn(n) => {
                ("GOOD PLAN\n".to_string(), 0)
            }
            check::VisitResult::Missing(hash_set) => {
                let mut s: String = "BAD PLAN\n".to_string();
                for data::Pos(x, y) in hash_set {
                    s += &format!("{}, {}\n", x, y);
                }
                (s, 0)
            }
        },
        data::Task::FindTask { player, field } => {
            let (commands_shortened, commands_full) =
                find::concatenate_strategies(&field, player, 1, 1, 10000);
            let (cmds, n) = match check::potential_unvisited(&field, &commands_shortened, player) {
                check::VisitResult::AllVisitedIn(n) => (commands_shortened, n),
                check::VisitResult::Missing(_) => {
                    match check::potential_unvisited(&field, &commands_full, player) {
                        check::VisitResult::AllVisitedIn(n) => (commands_full, n),
                        check::VisitResult::Missing(_) => panic!("did not find path???"),
                    }
                }
            };
            (display_commands(cmds), n)
        }
    }
}

fn main() {
    /*    let mut dirs1 = vec![Direction::Left, Direction::Right, Direction::Right];
    let dirs2 = vec![Direction::Left, Direction::Right];
    eprintln!("{:?}\n", find::generate_commands(&dirs1, Direction::Right));

    dirs1.extend(dirs2);
    eprintln!("{:?}\n", find::generate_commands(&dirs1, Direction::Right));
    panic!();*/
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 3 {
        panic!("call as: assignment0 <folder with input problems> <folder for solution outputs>");
    }
    let input_folder = &args[1];
    let output_folder = &args[2];
    let paths = fs::read_dir(input_folder).expect("Input path does not exist/is unreadable");
    let now = time::Instant::now();
    let mut sum = 0;
    let mut max_ = 0;
    for entry in paths {
        let filename : String = entry.as_ref().map(|n| n.file_name().into_string()).expect("path error").expect("path error");
        let stripped = filename.strip_prefix("problem").expect("non problem file in folder");
        let mut output_path = Path::new(output_folder).to_owned();
        let input_path = entry.expect("dir issue").path();
        output_path.push("solution".to_owned() + stripped);
        println!("input {:?}", input_path);
        println!("output {:?}", output_path);
        let (res, n) = eval(input_path);
        max_ = max_.max(n);
        sum += n;
        fs::write(output_path, res).expect("cannot write to solution file");
    }
    eprintln!("{} steps total, {} max", sum, max_);
    eprintln!("Elapsed total: {:.2?}", now.elapsed());
}
