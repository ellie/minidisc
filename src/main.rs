// Ok cool so let's figure out how this should work
// One important fact we should remember:

// ====================================================
// A process cannot change the directory of its parent.
// ====================================================

// So, seeing as minidisc cannot chdir its parent (ie, the shell), then
// we need to do something else.
// We can wrap md with a shell function. The shell function can pass the
// input, md can figure out what directory to use, and then the shell
// can switch to that.

use clap::Parser;

mod cd;
mod init;

#[derive(clap::Subcommand, Debug)]
enum Command {
    Cd(cd::Cmd),
    Init,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub action: Command,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Command::Cd(cmd) => cmd.run(),
        Command::Init => init::run(),
    };
}
