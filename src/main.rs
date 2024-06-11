use std::path::PathBuf;
use clap::{ Arg, ArgGroup, Command };
use cwcat::Error as CWCatError;

fn main() {
	let matches =
	Command::new("cwcat")
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))

		.arg(Arg::new("input")
			.short('i')
			.value_name("INPUT DIRECTORY")
			.help("Recording directory to concatenate")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))

		.arg(Arg::new("input-default")
			.short('I')
			.value_name("RECORDING NAME")
			.help("Recording to concatenate from default rec path")
			.required(true))

		.arg(Arg::new("list")
			.short('l')
			.long("list")
			.help("Print clip details at path")
			.long_help("Print the clip details at the input directory and exit")
			.action(clap::ArgAction::SetTrue))

		.arg(Arg::new("list-default")
			.short('L')
			.long("list-default")
			.help("List clips at default rec path")
			.long_help("List clips at the default rec path and exit")
			.action(clap::ArgAction::SetTrue))

		.arg(Arg::new("output")
			.short('o')
			.value_name("OUTPUT FILE")
			.help("File to write")
			.long_help("The file to write the concatenated webm to")
			.value_parser(clap::value_parser!(PathBuf))
			.required(false))

		.arg(Arg::new("keep-second-track")
			.short('k')
			.long("keep-second-track")
			.help("Keep the second game-only audio track")
			.long_help("Keep the second audio track, which contains only game audio")
			.action(clap::ArgAction::SetTrue))

		.arg(Arg::new("force")
			.short('f')
			.long("force")
			.help("Overwrite existing")
			.long_help("If output file exists, overwrite anyway")
			.action(clap::ArgAction::SetTrue))

		.arg(Arg::new("rec-path")
			.short('r')
			.long("rec-path")
			.help("Print default rec path")
			.long_help("Print the default rec path and exit")
			.action(clap::ArgAction::SetTrue))

		.group(ArgGroup::new("modes").args(["input", "input-default", "rec-path", "list-default"]))
		.group(ArgGroup::new("print-modes").args(["list", "rec-path", "list-default", "output"]))

	.get_matches();

	match do_main(&matches) {
		Ok(ExitStatus::OK) => std::process::exit(0),
		Ok(ExitStatus::WontForce(output)) => {
			eprintln!("The output file {:?} already exists! Will not overwrite. Use `--force` if you want to overwrite this file.", output);
			std::process::exit(2)
		},
		Err(error) => {
			eprintln!("Error: {}", error);
			std::process::exit(error.to_code())
		}
	}
}

/// Normal exit statuses. So we can have special codes.
enum ExitStatus {
	/// Nothing happened, yay!
	OK,
	/// Output file exists and force is not enabled.
	WontForce(PathBuf)
}

/// Exit statuses that mean something went wrong.
enum ErrorStatus {
	CWCat (CWCatError),
	VarError (std::env::VarError)
}

impl From<CWCatError> for ErrorStatus {
	fn from(error: CWCatError) -> Self {
		Self::CWCat(error)
	}
}

impl From<std::io::Error> for ErrorStatus {
	fn from(error: std::io::Error) -> Self {
		Self::CWCat(error.into())
	}
}

impl From<std::env::VarError> for ErrorStatus {
	fn from(error: std::env::VarError) -> Self {
		Self::VarError(error)
	}
}

impl std::fmt::Display for ErrorStatus {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CWCat(error) => error.fmt(f),
			Self::VarError(error) => error.fmt(f)
		}
	}
}

impl ErrorStatus {
	/// Return an exit code from this ErrorStatus.
	pub fn to_code(&self) -> i32 {
		match self {
			Self::CWCat(CWCatError::DemuxError (_)) => 3,
			Self::CWCat(CWCatError::SetColorError) => 4,
			Self::CWCat(CWCatError::SetPrivateDataError (_)) => 5,
			Self::CWCat(CWCatError::AddFrameError { .. }) => 6,
			Self::CWCat(CWCatError::FinalizeError) => 7,
			Self::CWCat(CWCatError::NoFiles) => 8,
			Self::CWCat(CWCatError::ChannelChanges { .. }) => 9,
			Self::CWCat(CWCatError::SampleRateChanges { .. }) => 10,
			Self::CWCat(CWCatError::VideoResolutionChanges { .. }) => 11,
			Self::CWCat(CWCatError::UnknownKeyframe) => 12,
			Self::CWCat(CWCatError::MissingDuration) => 13,
			Self::CWCat(CWCatError::IOError (_)) => 1,
			Self::VarError(std::env::VarError::NotPresent) => 32,
			Self::VarError(std::env::VarError::NotUnicode(_)) => 33
		}
	}
}

/// Do main. 🙂
fn do_main(matches: &clap::ArgMatches) -> Result<ExitStatus, ErrorStatus> {
	let print_rec_path = matches.get_flag("rec-path");

	if print_rec_path {
		println!("{}", default_path()?.to_string_lossy());

		return Ok(ExitStatus::OK)
	}

	let list_default = matches.get_flag("list-default");

	if list_default {
		print_video_metadata(default_path()?)?;
		return Ok(ExitStatus::OK)
	}

	let input: PathBuf = match (matches.get_one::<&PathBuf>("input"), matches.get_one::<String>("input-default")) {
		(None, None) => panic!("clap did not return a required input argument"),
		(Some(_), Some(_)) => unreachable!(),
		(Some(path), _) => path.to_path_buf(),
		(None, Some(arg)) => default_path()?.join(arg)
	};

	let output: &PathBuf = match matches.get_one("output") {
		Some(path) => path,
		None => {
			print_clip_metadata(input)?;
			return Ok(ExitStatus::OK)
		}
	};

	let force = matches.get_flag("force");

	if !force && output.exists() { return Ok(ExitStatus::WontForce(output.to_owned())) }

	let bytes = cwcat::concatenate_from_rec_path(input, matches.get_flag("keep-second-track"))?;

	println!("Concatenated files (in {}), saving to {:?}", human_readable_size(bytes.len() as u64), output);

	std::fs::write(output, bytes)?;

	println!("File written successfully.");

	Ok(ExitStatus::OK)
}

/// Print the metadata of clips at path.
pub fn print_clip_metadata<P>(path: P) -> Result<(), CWCatError> where P: Into<PathBuf> {
	let dirs = cwcat::list_from_rec_path(path)?;

	// First, get the clip info.
	let mut meta: Vec<(String, String, chrono::DateTime<chrono::Local>)> = vec![];
	let mut total_duration = 0;

	for (path, metadata) in &dirs {
		let name = path.file_name().expect("File should have a name").to_string_lossy();
		let duration = match cwcat::clip_duration_from_path(&path) {
			Err(_) => "ERROR".to_owned(),
			Ok(d) => { total_duration += d; human_readable_time(d) }
		};
		let time = chrono::DateTime::<chrono::Local>::from(metadata.created()?);
		
		meta.push((name.to_string(), duration, time))
	}

	print!(
		"Total of {} clips, total duration is {}",
		dirs.len(),
		human_readable_time(total_duration)
	);
	if !meta.is_empty() {
		print!(", created at {}", meta[0].2.format("%d %b %Y %r"))
	}
	println!("\n");

	print!("Name");
	for _ in 0..38 { print!(" ") }
	print!("Duration");
	for _ in 0..9 { print!(" ") }
	println!("Recording Time");

	for (name, duration, time) in &meta {
		print!("{}", name);
		if name.len() < 42 {
			for _ in 0..42 - name.len() {
				print!(" ")
			}
		}

		print!("{}", duration);
		if duration.len() < 14 {
			for _ in 0..17 - duration.len() {
				print!(" ")
			}
		}

		#[cfg(not(feature = "pretty-time"))]
		println!("{:?}", metadata.created()?);
		#[cfg(feature = "pretty-time")]
		{
			let delta = time.signed_duration_since(meta[0].2);
			println!("+{}", human_readable_time(delta.num_milliseconds() as u64))
		}
	}

	Ok(())
}

/// Print the metadata of videos at the given rec path.
fn print_video_metadata<P>(path: P) -> Result<(), CWCatError> where P: Into<PathBuf> {
	let mut dirs = Vec::new();

	// Find all the directories that have the clips we want
	for entry in std::fs::read_dir(path.into())? {
		let entry = entry?;
		let metadata = entry.metadata()?;
		if metadata.is_dir() {
			if !cwcat::list_from_rec_path(entry.path())?.is_empty() {
				let created = metadata.created()?;
				dirs.push((entry.path(), created))
			}
		}
	}

	// Sort them by ascending date created
	dirs.sort_by(|a, b| {
		a.1.cmp(&b.1)
	});

	println!("Total of {} videos\n", dirs.len());
	print!("Name");
	for _ in 0..38 { print!(" ") }
	print!("Duration");
	for _ in 0..9 { print!(" ") }
	println!("Creation Date");

	for (path, created) in dirs {
		let name = path.file_name().expect("File should have a name").to_string_lossy();
		let duration = match cwcat::duration_from_rec_path(&path) {
			Err(_) => "ERROR".to_owned(),
			Ok(d) => human_readable_time(d)
		};

		print!("{}", name);
		if name.len() < 42 {
			for _ in 0..42 - name.len() {
				print!(" ")
			}
		}

		print!("{}", duration);
		if duration.len() < 14 {
			for _ in 0..17 - duration.len() {
				print!(" ")
			}
		}

		#[cfg(not(feature = "pretty-time"))]
		println!("{:?}", created);
		#[cfg(feature = "pretty-time")]
		{
			let time = chrono::DateTime::<chrono::Local>::from(created);
			println!("{}", time.format("%d %b %Y %r"))
		}
	}

	Ok(())
}

/// Return the default path for the game's rec directory.
pub fn default_path() -> Result<PathBuf, std::env::VarError> {
	#[cfg(target_os = "windows")]
	return Ok(PathBuf::from(std::env::var("LOCALAPPDATA")?)
		.join("Temp/rec/"));
	#[cfg(not(target_os = "windows"))]
	return Ok(PathBuf::from(std::env::var("HOME")?)
		.join(".local/share/Steam/steamapps/compatdata/2881650/pfx/drive_c/users/steamuser/Temp/rec/"))
}

/// Return a human-readable data size from a number of bytes.
pub fn human_readable_size(bytes: u64) -> String {
	if bytes < 1_000 {
		format!("{} bytes", bytes)
	} else {
		let kilobytes = bytes as f64 / 1_000.0;
		if kilobytes < 1_000.0 {
			format!("{} KB", kilobytes)
		} else {
			let megabytes = kilobytes / 1_000.0;
			// This is as high as we'll go for now.
			format!("{} MB", megabytes)
		}
	}
}

/// Return a human-readable time from a number of milliseconds.
pub fn human_readable_time(mut milliseconds: u64) -> String {
	let mut seconds = milliseconds / 1_000;
	milliseconds = milliseconds % 1_000;
	let minutes = seconds / 60;
	seconds = seconds % 60;
	format!("{minutes}m {seconds}s {milliseconds}ms")
}
