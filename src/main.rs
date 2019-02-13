use std::{
    env, fs,
    io::{self, stdin, stdout, Write},
    path::Path,
    process,
};

fn run(source: &str) {
    println!("{}", source);
}

fn get_user_input() -> io::Result<String> {
    let mut buffer = String::new();

    stdout().flush()?;

    stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}

fn run_prompt() -> io::Result<()> {
    loop {
        print!("> ");

        let user_input = get_user_input()?;

        run(&user_input);
    }
}

fn run_file(file_arg: &str) -> io::Result<()> {
    let file_path = Path::new(file_arg);

    let file_content = fs::read_to_string(&file_path)?;

    run(&file_content);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = env::args();

    match args.len() {
        1 => run_prompt()?,
        2 => run_file(&args.last().unwrap())?,
        _ => {
            println!("Usage: locl [script]");

            process::exit(64);
        }
    }

    Ok(())
}
