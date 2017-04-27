extern crate commander;

use commander::Commander;

fn main() {
    let command = Commander::new()
                .version("0.0.1")
                .usage("test")
                .usage("aaaaaa")
                .usage("bbbb")
                .usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.");
    command.print_help();
    println!("command = {:?}", command);
}