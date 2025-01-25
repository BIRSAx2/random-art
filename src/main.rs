use clap::Parser;
use random_art::grammar::{ArtGrammar, PerrigSongGrammar, RandomArtGrammar};
use random_art::operations::Operation;
use random_art::renderer::*;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum RenderMode {
    File,
    Window,
}
#[derive(clap::Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(
        short,
        long,
        value_name = "SEED_STRING",
        help = "Sets a custom seed (string or integer)"
    )]
    seed: Option<String>,

    #[clap(
        short,
        long,
        value_name = "DEPTH",
        default_value = "5",
        help = "Depth of the expression tree to generate"
    )]
    depth: usize,

    #[clap(
        short,
        long,
        value_name = "OUTPUT_FILE",
        default_value = "generated/random_art.png",
        help = "Sets the output file name"
    )]
    output: String,

    #[clap(
        short,
        long,
        value_name = "RENDER_MODE",
        default_value = "file",
        help = "Render mode"
    )]
    render_mode: RenderMode,

    #[clap(
        short,
        long,
        value_name = "USE_ALTERNATIVE_GRAMMAR",
        default_value = "false",
        help = "Use alternative grammar"
    )]
    use_alternative_grammar: bool,
}

#[macroquad::main("Random Art")]
async fn main() {
    let args = Args::parse();

    // Create the output directory if it doesn't exist
    let output_dir = Path::new(&args.output)
        .parent()
        .expect("Failed to get output directory");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }

    let x_res = 800;
    let y_res = 800;

    let seed = match args.seed {
        Some(seed_str) => {
            let mut hasher = DefaultHasher::new();
            seed_str.hash(&mut hasher);
            hasher.finish()
        }
        None => {
            let now = SystemTime::now();
            let since_epoch = now
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards");
            let mut hasher = DefaultHasher::new();
            since_epoch.as_millis().hash(&mut hasher);
            println!(
                "No seed provided, using current time as seed: {}",
                since_epoch.as_millis()
            );

            hasher.finish()
        }
    };

    let root: Operation;
    if args.use_alternative_grammar {
        root = RandomArtGrammar::new(seed).generate_tree(args.depth);
    } else {
        root = PerrigSongGrammar::new(seed).generate_tree(args.depth);
    }

    if let RenderMode::Window = args.render_mode {
        println!("Rendering to window");
        WindowRenderer::new().render(x_res, y_res, &root).await;
    } else {
        println!("Rendering to file");
        FileRenderer::new(args.output)
            .render(x_res, y_res, &root)
            .expect("Failed to render image");
    }
}
