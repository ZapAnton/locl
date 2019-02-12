use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
    process,
};

fn run(source: &str) {
    println!("{}", source);
}

fn get_user_input() -> String {
    let mut buffer = String::new();

    stdout().flush().unwrap();

    stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_string()
}

fn run_prompt() {
    loop {
        print!("> ");

        let user_input = get_user_input();

        run(&user_input);
    }
}

fn run_file(file_arg: &str) {
    let file_path = Path::new(file_arg);

    let file_content = fs::read_to_string(&file_path).unwrap();

    run(&file_content);
}

fn main() {
    let args = env::args();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args.last().unwrap()),
        _ => {
            println!("Usage: locl [script]");

            process::exit(64);
        }
    }
}
