use std::fs;
use std::path;
use std::time::Instant;

use structopt::StructOpt;

const MB_FACTOR: u64 = 1024 * 1024;

#[derive(StructOpt, Debug)]
struct Options {
	#[structopt(short="f", long, default_value=".")]
	folder: String,
	#[structopt(short="s", long)]
	folder_size: bool,
	#[structopt(default_value="4")]
	threads: u8,
	#[structopt(short="i", long, default_value="10")]
	track: u8,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Entry {
	item_name: String,
	size: u64,
}

struct Tracker {
	list: Vec<Entry>,
	min: u64,
	min_pos: usize,
}

impl Tracker {
	fn new(max_items: u8) -> Tracker {
		Tracker {
			list: vec![Entry{item_name: "".to_string(), size: 0}; max_items as usize],
			min: 0,
			min_pos: 0,
		}
	}

	fn report(&mut self, file_name: String, size: u64) {
		if size < self.min {
			return
		}
		
		self.list[self.min_pos] = Entry{
			item_name: file_name,
			size: size,
		};
		let mut new_min : u64 = size;
		let mut new_pos : usize = self.min_pos;
		for i in 0..self.list.len() {
			if self.list[i].size <= new_min {
				new_min = self.list[i].size;
				new_pos = i;
			}
		}
		self.min = new_min;
		self.min_pos = new_pos;
	}

	fn print(mut self) {
		self.list.sort_by(|a,b| b.size.cmp(&a.size));
		for item in self.list {
			println!("{:65} = {} Mb", item.item_name, to_mb(item.size));
		}
	}
}

fn to_mb(bytes_source: u64) -> u64 {
	bytes_source / MB_FACTOR
}

fn list(original_path: String, only_folders: bool, tracker: &mut Tracker) {
	let paths = fs::read_dir(&original_path);
	match paths {
		Ok(p) => {
			let mut folder_size: u64 = 0;
			for path in p {
				let dir_entry = &path.unwrap();
				if let Ok(metadata) = dir_entry.metadata() {
					if metadata.is_dir() {
						let fpath = format!("{}{}{}", original_path, path::MAIN_SEPARATOR, dir_entry.file_name().into_string().unwrap());
						list(fpath, only_folders, tracker);
					} else {
						if !only_folders {
							let fpath = format!("{}{}{}", original_path, path::MAIN_SEPARATOR, dir_entry.file_name().into_string().unwrap());
							tracker.report(fpath, metadata.len());
						}
						folder_size += metadata.len();
					}
					
				}
			}
			if only_folders{
				tracker.report(original_path, folder_size);
			}
		},
		Err(e) => {
			println!("Error for path {}: {}", original_path, e);
		}
	}
}

fn main() {
	let opt = Options::from_args();
	let mut tracker = Tracker::new(opt.track);
	println!("Will look for the top {} files from {} using {} threads\n", opt.track, opt.folder, opt.threads);
	let start = Instant::now();
	list(opt.folder, opt.folder_size, &mut tracker);
	println!("\nResults:");
	tracker.print();
	println!("\n--- Total process took: {:?}", start.elapsed());
}