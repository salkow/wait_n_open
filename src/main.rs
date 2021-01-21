use std::process;
use structopt::StructOpt;
use wait_n_open::Opt;

fn main()
{
	// Get command line arguments.
	let config = Opt::from_args();

	if let Err(e) = wait_n_open::run(config)
	{
		eprintln!("Application error: {}", e);

		process::exit(1);
	}
}
