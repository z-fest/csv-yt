use std::{fs, env, path::PathBuf, str::FromStr, process::Command};

type Result<T = (), E = ()> = std::result::Result<T, E>;

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

const FOLDER: &str = "csv-yt";

fn help() -> Result {
    println!("That's not how you use this command");
    println!();
    println!("Look at the docs");

    Err(())
}

fn main() -> Result {
    let mut commands = Vec::new();
    let mut args = env::args().skip(1);
    let Some(csv_file) = args.next() else {
        return help();
    };

    println!("We goin’ with {csv_file}");

    let Some(link_column) = args.next() else {
        return help();
    };
    
    println!("Link column: {link_column}");

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

        commands.push(command);
    }

    println!("Commands: {commands:?}");

    let Ok(mut csv_reader) = csv::Reader::from_path(&csv_file) else {
        eprintln!("Failed to open file: {csv_file}");

        return Err(());
    };
    let Ok(headers) = csv_reader.headers() else {
        eprintln!("File {csv_file} is not valid CSV");

        return Err(());
    };
    let headers: Vec<_> = headers.iter().map(String::from).collect();
    let Some(link_column) = headers.iter().position(|x| x == &link_column) else {
        eprintln!("Link column not found: {link_column}");

        return Err(());
    };

    for result in csv_reader.records() {
        let Ok(record) = result else {
            eprintln!("File {csv_file} is not valid CSV");

            return Err(());
        };

        let mut dirname = String::new();
        let mut filename = String::new();

        for cmd in commands.iter() {
            use Subcommand::*;

            match cmd {
                FileVar(var) => {
                    let mut lit = None;

                    for (header, value) in headers.iter().zip(record.iter()) {
                        if header == var {
                            lit = Some(value);
                        }
                    }

                    let Some(lit) = lit else {
                        eprintln!("No CSV column of name {var}");

                        return Err(());
                    };

                    filename.push_str(lit);
                },
                FileLit(lit) => filename.push_str(lit),
                DirVar(var) => {
                    let mut lit = None;

                    for (header, value) in headers.iter().zip(record.iter()) {
                        if header == var {
                            lit = Some(value);
                        }
                    }

                    let Some(lit) = lit else {
                        eprintln!("No CSV column of name {var}");

                        return Err(());
                    };

                    dirname.push_str(lit);
                },
                DirLit(lit) => dirname.push_str(lit),
            }
        }

        {
            let mut dir = PathBuf::from_str(FOLDER).unwrap();

            dir.push(&dirname);
            
            fs::create_dir_all(dir).unwrap();
        }

        let args = [
            &record[link_column],
            "-o",
            &format!("{FOLDER}/{dirname}/{filename}.%(ext)s"),
        ];
        let Ok(mut child) = Command::new("yt-dlp").args(args).spawn() else {
            eprintln!("Failed to spawn yt-dlp");

            return Err(());
        };
        let Ok(status) = child.wait() else {
            eprintln!("Failed to wait for yt-dlp to complete");

            return Err(());
        };

        if !status.success() {
            eprintln!("warning: Failed to download {}", &record[link_column]);
        }
    }

    Ok(())
}
