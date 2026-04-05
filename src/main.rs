use std::io::{self, BufRead, Write};
use std::process;

use xcat::cli::Cli;
use xcat::colorizer::Colorizer;
use xcat::config::Config;
use xcat::display::DisplayOptions;
use xcat::error::XcatResult;
use xcat::reader::{FileReader, process_line};

pub fn run() {
    let cli = Cli::parse_args();
    let config = match Config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let opts = DisplayOptions::from_cli_and_config(&cli, &config);
    let colorizer = Colorizer::new(opts.color_enabled);

    let exit_code = match execute(&cli, &opts, &colorizer) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    };

    process::exit(exit_code);
}

fn execute(cli: &Cli, opts: &DisplayOptions, colorizer: &Colorizer) -> XcatResult<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let total_line_count = if opts.count_lines {
        let mut total = 0usize;
        let files = if cli.files.is_empty() {
            vec![String::from("-")]
        } else {
            cli.files.clone()
        };

        for path in &files {
            let reader = FileReader::open(path)?;
            let content = reader.read_to_string()?;
            let count = content.lines().count();
            total += count;
        }
        Some(total)
    } else {
        None
    };

    let files = if cli.files.is_empty() {
        vec![String::from("-")]
    } else {
        cli.files.clone()
    };

    let multiple_files = files.len() > 1;

    for path in &files {
        if multiple_files {
            writeln!(out, "{}", colorizer.colorize_header(path))?;
        }

        let reader = FileReader::open(path)?;
        let buf_reader = reader.reader()?;
        let mut line_num: usize = 0;
        let mut blank_lines: usize = 0;

        for line_result in buf_reader.lines() {
            let line = line_result.map_err(|e| {
                xcat::error::XcatError::Io(e, path.clone())
            })?;
            process_line(
                &line,
                &mut line_num,
                &mut blank_lines,
                opts,
                colorizer,
                &mut out,
            )?;
        }
    }

    if opts.count_lines {
        if let Some(count) = total_line_count {
            writeln!(
                out,
                "{}",
                colorizer.colorize_header(&format!("Total lines: {count}"))
            )?;
        }
    }

    out.flush()?;
    Ok(())
}

fn main() {
    run();
}
