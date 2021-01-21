use std::process::Command;
use std::path::{Path,PathBuf};
use inotify::{Inotify, WatchMask, EventMask};
use std::env::current_dir;
use std::{thread, time};
use structopt::StructOpt;
use anyhow::{Context, Result, bail};
use std::fs::metadata;
use open;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Name of the PBS script.
    #[structopt(short, long, parse(from_os_str))]
	pbs_name: PathBuf,
	
    /// Name of the job, from the PBS script.
    #[structopt(short, long, parse(from_os_str))]
    job_name: PathBuf,
	
	/// Name of the editor
    #[structopt(short, long, env="EDITOR")]
	editor: String,
	
	/// Opens the error file, if it isn't empty.
    #[structopt(short, long)]
    check_error: bool,
}

pub fn run_pbs_script(pbs_name: PathBuf) -> Result<String>
{
	let output = Command::new("qsub").arg(&pbs_name).output()
		.with_context(|| format!("Error while running the PBS script: {}", pbs_name.display()))?;
						 
	let file_id = String::from_utf8(output.stdout)
		.with_context(|| format!("Error while running the PBS script: {}", pbs_name.display()))?;
	
	let file_id = file_id.trim().strip_suffix(".argo")
		.with_context(|| format!("Error while running the PBS script: {}", pbs_name.display()))?;

	Ok(String::from(file_id))
}

pub fn run(config: Opt) -> Result <()>
{
	let mut file_id = run_pbs_script(config.pbs_name)?;

	// Use the output or the error file.
	match config.check_error
	{
		true => file_id.insert(0, 'e'),
		false => file_id.insert(0, 'o'),
	}

	let file_name = config.job_name.with_extension(file_id.as_str());

	let dir_path = current_dir().context("Error while trying to read current directory.")?;

	let mut file_path = PathBuf::new();

	if file_name.is_relative()
	{
		file_path.push(dir_path.clone());
	}

	file_path.push(file_name);

	wait_for_file(dir_path.as_path(), file_path.as_path())?;

	// Wait for the file contents to be written.
	thread::sleep(time::Duration::from_millis(250));

	if config.check_error && metadata(&file_path)?.len() == 0
	{
		file_path.set_extension(file_id.replace("e", "o"));
	}

	open::with(&file_path, config.editor)
		 .with_context(|| format!("Error in opening the file: {}", file_path.display()))?;

	Ok(())
}

fn wait_for_file(dir_path: &Path, file_path: &Path) -> Result<()>
{
	let mut inotify = Inotify::init().context("Failed to initialize inotify")?;

	// Notify when directory is deleted or file is created.
	inotify.add_watch(&dir_path, WatchMask::DELETE_SELF | WatchMask::CREATE)
		   .context("Failed to add inotify watch")?;

	if !file_path.exists() 
	{
		loop 
		{
			let mut buffer = [0; 1024];

			let events = inotify.read_events_blocking(&mut buffer)
								.context("Failed to read inotify events")?;

			for event in events 
			{
				if event.mask.contains(EventMask::CREATE) 
				{
					let event_name = event.name.context("Error in inotify event name")?;

					if dir_path.join(event_name) == file_path
					{
						return Ok(());
					}
				}

				else if event.mask.contains(EventMask::DELETE) 
				{
					bail!("The watched directory has been deleted.");
				}
			}
		}
	}

	// The file already existed.
	return Ok(());
}
