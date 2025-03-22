use rayon::prelude::*;
use svg::Document;

pub fn save_batch(
    svg_documents: &mut Vec<Document>,
    frames: &mut Vec<usize>,
    last_batch: bool,
    output_dir: &str,
) {
    if last_batch || frames.len() == 100 {
        frames
            .par_iter()
            .zip(svg_documents.par_iter())
            .for_each(|(frame_index, document)| {
                let output_path = format!("{}/frame_{:05}.svg", output_dir, frame_index);
                svg::save(&output_path, document).expect("Failed to save SVG frame");
            });
        svg_documents.clear();
        frames.clear();
    }
}
