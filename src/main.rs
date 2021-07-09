#[warn(clippy::pedantic)]
mod app;
mod canvas;
mod io;
mod mode;
mod util;

use app::{edit, EditorOptions};
use clap::Clap;
use io::{ImageFormat, ImageIo};

#[derive(Clap)]
#[clap(version = "0.1", author = "Aldo Acevedo <aldo@aael.xyz>")]
struct Opts {
    /// Set input file (- is treated as stdin)
    input: ImageIo,

    /// Set output file (- is treated as stdout)
    #[clap(short, long)]
    output: Option<ImageIo>,

    /// Override output format. The default when outputting to stdout is PNG. When outputting
    /// to a file, it is guessed by the extension.
    #[clap(short = 'F', long = "format")]
    output_format: Option<ImageFormat>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();
    let canvas = opts.input.read()?;

    let options = EditorOptions::default();
    let output_canvas = edit(canvas, options);

    if let Some(output) = opts.output {
        output.write(&output_canvas, opts.output_format)
    } else {
        Ok(())
    }
}
