use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to input image
    #[clap(short, long)]
    pub image: String,

    /// Number of stippling points
    #[clap(short, long)]
    pub points: usize,
    // /// Output file path [default: output.svg]
    // #[clap(short, long, default_value = "output.svg")]
    // output: String,
    /// Number of iterations [default: 10]
    #[clap(short, long, default_value_t = 50)]
    pub iterations: usize,

    /// Min radius [default: 1.0]
    #[clap(short, long, default_value_t = 1.0)]
    pub min_radius: f32,

    /// Max radius [default: 3.0]
    #[clap(short, long, default_value_t = 3.0)]
    pub max_radius: f32,

    /// Min stroke width [default: 0.5]
    #[clap(short, long, default_value_t = 0.5)]
    pub min_stroke_width: f32,

    /// Max stroke width [default: 3.0]
    #[clap(short, long, default_value_t = 3.0)]
    pub max_stroke_width: f32,
    // /// Random seed [default: 42]
    // #[clap(short, long, default_value_t = 42)]
    // seed: u64,
}
