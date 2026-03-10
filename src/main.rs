use ansi_strip::{process_stream, process_string};
use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ansi-strip")]
#[command(about = "Remove ANSI escape codes from text streams and files", long_about = None)]
struct Cli {
    #[arg(help = "Input files to process (omit for stdin)")]
    files: Vec<PathBuf>,

    #[arg(short, long, help = "Output file (default: stdout)")]
    output: Option<PathBuf>,

    #[arg(short, long, help = "Show processing statistics")]
    verbose: bool,

    #[arg(short, long, help = "Process files in-place (overwrite originals)")]
    in_place: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        process_stdin(&cli)?;
    } else {
        process_files(&cli)?;
    }

    Ok(())
}

fn process_stdin(cli: &Cli) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    let mut writer: Box<dyn Write> = if let Some(output) = &cli.output {
        Box::new(BufWriter::new(File::create(output)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    process_stream(reader, &mut writer, cli.verbose)?;
    writer.flush()?;

    Ok(())
}

fn process_files(cli: &Cli) -> io::Result<()> {
    for file_path in &cli.files {
        if !file_path.exists() {
            eprintln!("Warning: File not found: {}", file_path.display());
            continue;
        }

        let mut content = String::new();
        File::open(file_path)?.read_to_string(&mut content)?;

        let (cleaned, _) = process_string(&content, cli.verbose);

        if cli.in_place {
            fs::write(file_path, cleaned)?;
            if cli.verbose {
                eprintln!("Updated: {}", file_path.display());
            }
        } else if let Some(output) = &cli.output {
            fs::write(output, cleaned)?;
        } else {
            print!("{}", cleaned);
        }
    }

    Ok(())
}
