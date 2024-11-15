use clap::Parser;
use image::GenericImage;
use std::{fs, process::exit, sync::mpsc, thread};

mod cli;
mod compiler;
mod renderer;
mod text;

use compiler::parser;

fn main() {
    let opts = cli::Cli::parse();

    let source = match fs::read_to_string(&opts.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", opts.input.display(), e);
            exit(1);
        }
    };

    let lexer = compiler::lexer::Lexer::new(source);
    let source = lexer.source();
    let tokens: Vec<_> = lexer.collect();

    if opts.print_tokens {
        println!("Tokens:");
        for token in &tokens {
            println!("\t{:?}", token.kind);
        }
    }

    let on_retry = match opts.verbose {
        true => || println!("Expression returned constant, retrying ..."),
        false => || {},
    };
    let expr = parser::parse_tokens(tokens, source.clone(), on_retry);

    if opts.print_expr {
        println!("{}", expr);
    }

    if opts.print_ast {
        expr.print_ast(source);
    }

    if opts.dry_run {
        return;
    }

    const PARTS: u32 = 10;

    let mut image = image::RgbImage::new(opts.width, opts.height);

    let mut part = 0;
    let mut done_parts = 0;

    let (tx, rx) = mpsc::channel();

    while part < PARTS {
        let mut threads_running = 0;

        for _ in 0..opts.threads {
            if part >= PARTS {
                break;
            }
            threads_running += 1;
            let tx = tx.clone();
            let expr = expr.clone();
            thread::spawn(move || {
                let width = opts.width;
                let height = opts.height;
                let start_y = part * height / PARTS;
                let end_y = (part + 1) * height / PARTS;
                let sub_image =
                    renderer::render_subimage(&expr, (0, width), (start_y, end_y), width, height);
                tx.send((start_y, sub_image)).unwrap();
            });
            part += 1;
        }

        while threads_running > 0 {
            let (start_y, sub_image) = rx.recv().unwrap();
            image.copy_from(&sub_image, 0, start_y).unwrap();

            if opts.verbose {
                done_parts += 1;
                println!("Rendering {}% ...", done_parts * 100 / PARTS);
            }

            threads_running -= 1;
        }
    }

    let out_file = &opts.output;

    if opts.verbose {
        println!("Writing image to '{out_file}' ...");
    }

    let _ = image.save(out_file).map_err(|e| {
        eprintln!("Error saving image to '{}': {}", out_file, e);
        exit(1);
    });

    if opts.verbose {
        println!("Write successful!");
    }
}
