use std::env;

#[derive(Debug)]
enum Subcommand {
    /// Append contents of CSV column to the end of the filename
    FileVar(String),
    /// Append text to the end of the filename
    FileLit(String),
    /// Append contents of CSV column to the end of the directory name
    DirVar(String),
    /// Append text to the end of the directory name
    DirLit(String),
}

fn help() -> Result<(), ()> {
    println!("That's not how you use this command");
    println!("");
    println!("Look at the docs");

    Err(())
}

fn main() -> Result<(), ()> {
    let mut args = env::args().skip(1);
    let Some(csv_file) = args.next() else {
        return help();
    };

    println!("We goin’ with {csv_file}");

    loop {
        let Some(command) = args.next() else {
            break;
        };
        let Some(parameter) = args.next() else {
            return help();
        };

        let command = match command.as_str() {
            "fv" => Subcommand::FileVar,
            "fl" => Subcommand::FileLit,
            "dv" => Subcommand::DirVar,
            "dl" => Subcommand::DirLit,
            _ => return help(),
        }(parameter);

        println!("  Command: {command:?}");
    }

    Ok(())
}
