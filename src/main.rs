use std::path::PathBuf;
use clap::{ Arg, Command };
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
			.help("recording directory to concatenate")
			.value_parser(clap::value_parser!(PathBuf))
			.required(true))

		.arg(Arg::new("output")
			.short('o')
			.value_name("OUTPUT FILE")
			.help("file to write")
			.help("the file to write the concatenated webm to")
			.value_parser(clap::value_parser!(PathBuf))
			.required(false))

		.arg(Arg::new("keep-second-track")
			.short('k')
			.long("keep-second-track")
			.help("keep the second game-only audio track")
			.long_help("keep the second audio track, which contains only game audio")
			.action(clap::ArgAction::SetTrue))

		.arg(Arg::new("force")
			.short('f')
			.long("force")
			.help("overwrite existing")
			.long_help("if output file exists, overwrite anyway")
			.action(clap::ArgAction::SetTrue))

	.get_matches();

	match do_main(&matches) {
		Ok(ExitStatus::OK) => std::process::exit(0),
		Ok(ExitStatus::WontForce(output)) => {
			println!("The output file {:?} already exists! Will not overwrite. Use `--force` if you want to overwrite this file.", output);
			std::process::exit(2)
		},
		Err(error) => {
			println!("Error:");
			println!("{}", error);
			std::process::exit(
				match error {
					CWCatError::DemuxError (_) => 3,
					CWCatError::SetColorError => 4,
					CWCatError::SetPrivateDataError (_) => 5,
					CWCatError::AddFrameError { .. } => 6,
					CWCatError::FinalizeError => 7,
					CWCatError::NoFiles => 8,
					CWCatError::ChannelChanges { .. } => 9,
					CWCatError::SampleRateChanges { .. } => 10,
					CWCatError::VideoResolutionChanges { .. } => 11,
					CWCatError::UnknownKeyframe => 12,
					CWCatError::IOError (_) => 1
				}
			)
		}
	}
}

/// Exit statuses. So we can have special codes.
enum ExitStatus {
	/// Nothing happened, yay!
	OK,
	/// Output file exists and force is not enabled.
	WontForce(PathBuf)
}

/// Do main. 🙂
fn do_main(matches: &clap::ArgMatches) -> Result<ExitStatus, CWCatError> {
	let input: &PathBuf = matches.get_one("input")
	.expect("clap did not return required argument 'input'");

	let output: &PathBuf = match matches.get_one("output") {
		Some(path) => path,
		None => {
			print_metadata(input)?;
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
pub fn print_metadata<P>(path: P) -> Result<(), CWCatError> where P: Into<PathBuf> {
	let dirs = cwcat::list_from_rec_path(path)?;

	println!("Total of {} clips:\n", dirs.len());
	print!("Name");
	for _ in 0..38 { print!(" ") }
	println!("Creation Date");

	for (path, metadata) in dirs {
		let name = path.file_name().expect("File should have a name").to_string_lossy();
		print!("{}", name);
		if name.len() < 42 {
			for _ in 0..42 - name.len() {
				print!(" ")
			}
		}

		#[cfg(not(feature = "pretty-time"))]
		println!("{:?}", metadata.created()?);
		#[cfg(feature = "pretty-time")]
		{
			let time = chrono::DateTime::<chrono::Local>::from(metadata.created()?);
			println!("{}", time.format("%d %b %Y %r"))
		}
	}

	Ok(())
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
