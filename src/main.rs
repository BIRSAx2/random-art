use clap::{Arg, ArgAction, Command};
use random_art::grammar::Grammar;
use random_art::renderer::cpu::CpuRenderer;
use random_art::renderer::Renderer;
use random_art::utils::write_image;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug)]
struct Args {
    seed: u64,
    output_file: String,
    tree_depth: usize,
}

fn parse_args() -> Args {
    let matches = Command::new("random_art")
        .version("0.1.0")
        .author("Mouhieddine Sabir <me@mouhieddine.dev>")
        .about("Generates random art based on a grammar")
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .value_name("SEED_STRING")
                .help("Sets a custom seed (string or integer)")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .value_name("DEPTH")
                .help("Depth of the expression tree to generate")
                .default_value("5")
                .value_parser(clap::value_parser!(usize))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("output_file")
                .short('o')
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("Sets the output file name")
                .default_value("generated/random_art.png")
                .action(ArgAction::Set),
        )
        .get_matches();

    let seed = match matches.get_one::<String>("seed") {
        Some(seed_str) => {
            let mut hasher = DefaultHasher::new();
            seed_str.hash(&mut hasher);
            hasher.finish()
        }
        None => {
            println!("No seed provided, using current time as seed");
            let now = SystemTime::now();
            let since_epoch = now
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            let mut hasher = DefaultHasher::new();
            since_epoch.as_millis().hash(&mut hasher);
            hasher.finish()
        }
    };

    let depth = *matches.get_one::<usize>("depth").unwrap();

    let output_file = matches
        .get_one::<String>("output_file")
        .unwrap()
        .to_string();

    Args {
        seed,
        output_file,
        tree_depth: depth,
    }
}

fn main() {
    let args = parse_args();

    // Create the output directory if it doesn't exist
    let output_dir = Path::new(&args.output_file)
        .parent()
        .expect("Failed to get output directory");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }

    let x_res = 1000;
    let y_res = 1000;
    let min_x = 0.0;
    let max_x = 1.0;
    let min_y = 0.0;
    let max_y = 1.0;

    let mut grammar = match Grammar::new(args.seed) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error creating grammar: {}", e);
            return;
        }
    };
    let root = match grammar.build_tree(args.tree_depth) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error building tree: {}", e);
            return;
        }
    };

    let mut renderer = CpuRenderer {};
    let values = match renderer.render(x_res, y_res, min_x, max_x, min_y, max_y, &root) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error rendering image: {:?}", e);
            return;
        }
    };

    if let Err(e) = write_image(&args.output_file, x_res, y_res, &values) {
        eprintln!("Error writing image: {}", e);
        return;
    }

    println!("Image saved to {}", args.output_file);
}
