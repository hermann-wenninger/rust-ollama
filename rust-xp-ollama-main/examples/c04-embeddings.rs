// region:    --- Modules

use xp_ollama::Result;

use ollama_rs::Ollama;
use simple_fs::{ensure_dir, read_to_string, save_be_f32, save_be_f64, save_json};
use std::fs;
use std::path::Path;
use std::time::Instant;
use xp_ollama::splitter::{simple_char_splitter, simple_word_splitter};

// endregion: --- Modules

const MOCK_DIR: &str = "_mock-data";
const C04_DIR: &str = ".c04-data";

// IMPORTANT: Make sure to ollama run/pull `mixtral` and `nomic-embed-text` first.
//            You can run `cargo run --example c05-models` to install `nomic-embed-text`
// `mixtral` model (avg per embeddings ~1000ms)
// const EMBEDDINGS_MODEL: &str = xp_ollama::consts::MODEL_MIXTRAL;
// 10x faster (avg per embeddings ~30ms)
const EMBEDDINGS_MODEL: &str = xp_ollama::consts::MODEL_NOMIC;

const SPLIT_SIZE: u32 = 3000;

#[tokio::main]
async fn main() -> Result<()> {
	let ollama = Ollama::default();

	ensure_dir(C04_DIR)?;

	let txt = read_to_string(Path::new(MOCK_DIR).join("for-embeddings.txt"))?;
	// make the text bigger
	let txt = (txt.clone() + "\n\n").repeat(4);

	let splits = simple_word_splitter(&txt, SPLIT_SIZE)?;
	// let splits = simple_char_splitter(&txt, SPLIT_SIZE)?;

	let mut total_duration_us: u128 = 0;
	let mut splits_len = 0;
	let mut i = 0;
	for (i, seg) in splits.enumerate() {
		splits_len = i + 1;

		println!();
		let file_name = format!("c04-embeddings-{:0>2}.txt", i);
		let path = Path::new(C04_DIR).join(file_name);
		fs::write(path, &seg)?;

		println!("->>       text length: {}", seg.len());

		let start = Instant::now();
		let res = ollama
			.generate_embeddings(EMBEDDINGS_MODEL.to_string(), seg, None)
			.await?;
		total_duration_us += start.elapsed().as_micros();

		println!("->> embeddings length: {}", res.embeddings.len());

		let file_name = format!("c04-embeddings-{:0>2}.json", i);
		save_json(Path::new(C04_DIR).join(file_name), &res.embeddings)?;

		let file_name = format!("c04-embeddings-{:0>2}.be-f64.bin", i);
		let file_path = Path::new(C04_DIR).join(file_name);
		save_be_f64(&file_path, &res.embeddings)?;

		let file_name = format!("c04-embeddings-{:0>2}.be-f32.bin", i);
		let file_path = Path::new(C04_DIR).join(file_name);
		let f32s = res
			.embeddings
			.iter()
			.map(|&x| x as f32)
			.collect::<Vec<f32>>();
		save_be_f32(&file_path, &f32s)?;
	}

	let duration_ms = total_duration_us / splits_len as u128 / 1000;
	println!("\n->> average duration per embeddings: {duration_ms}ms");
	// >>>> WHAT NEXT <<<<

	Ok(())
}

// /// A SILLY text splitter on "char" num only.
// /// JUST FOR TESTING OLLAMA EMBEDDINGS APIs (not vector search)
// fn simple_char_splitter(txt: &str, num: u32) -> Result<Vec<String>> {
// 	let mut result = Vec::new();
// 	let mut last = 0;
// 	let mut count = 0;

// 	for (idx, _) in txt.char_indices() {
// 		count += 1;
// 		if count == num {
// 			result.push(&txt[last..idx + 1]);
// 			last = idx + 1;
// 			count = 0;
// 		}
// 	}

// 	// Handle any remaining characters
// 	if last < txt.len() {
// 		result.push(&txt[last..]);
// 	}

// 	Ok(result.into_iter().map(String::from).collect())
// }
