use clap::Parser;
use std::fs;
use std::io;
use std::path::Path;

//Command-line arguments definition using 'clap'

#[derive(Parser)]
#[command(name="rtree", version="1.0", author = "areum", about = "project")]

struct Args{
	// Path to start traversing
	path: String,

	//limit the depth of directory traversal
	#[arg(short, long, default_value_t = usize::MAX)]
	depth: usize,

	// Show hidden files (starting with '.')
	#[arg(short='a', long="all", default_value_t = false)]
	all: bool,

	
}
// Recursively prints directory structure in tree format
fn print_tree(path: &Path, prefix: String, depth: usize, show_hidden:bool) -> io::Result<()> {
	if depth == 0 {
		return Ok(());
	}
	// Read directory entires and filter hidden files if needed
	let mut entries: Vec<_> = fs::read_dir(path)?
		.filter_map(Result::ok)
		.filter(|e| {
			show_hidden || !e.file_name().to_string_lossy().starts_with(".")
		})
		.collect();

	// Sort entries alphabetically
	entries.sort_by_key(|e| e.path());
	let last_index = entries.len().saturating_sub(1);

	for (i, entry) in entries.iter().enumerate(){
		let path = entry.path();
		let file_name = path.file_name().unwrap().to_string_lossy();
		let is_last = i == last_index;
		let connector = if is_last {"└──"} else {"├── "};
		// Print crrent entry with appropriate tree prefix
		println!("{}{}{}", prefix, connector, file_name);
		
		// Recurse into subdirectories
		if path.is_dir() {
			let new_prefix = if is_last{
				format!("{}      ", prefix)
			} else {
				format!("{}|     ", prefix)
			};
			print_tree(&path, new_prefix, depth -1, show_hidden)?;
		}
	}
	Ok(())
}

fn main() -> io::Result<()> {
	// Parse command-Line arguments
	let args = Args::parse();
	let path = Path::new(&args.path);

   	println!("{}", path.display());

	//Start tree traversal
	print_tree(path, String::new(), args.depth, args.all)?;
	Ok(())
}


