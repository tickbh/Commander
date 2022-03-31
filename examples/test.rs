extern crate commander;
use commander::Commander;

fn main() {
    let command = Commander::new()
                .version(&env!("CARGO_PKG_VERSION").to_string())
                .usage("test")
                .usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
                .option_list("-l, --list [value]", "list", Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]))
                .option_int("--enum value", "enum", None)
                .option_float("-f, --float value", "debug", Some(12.0))
                .option_int("-d, --debug value", "debug", Some(124))
                .option_str("-c, --copy value", "拷贝内容", Some("aaa".to_string()))
                .option("-r", "enable recursive", None)
                .after_desc("\n\nBy default, sparse SOURCE files are detected by a crude heuristic and the \n\
                            corresponding DEST file is made sparse as well.  That is the behavior      \n\
                            selected by --sparse=auto.  Specify --sparse=always to create a sparse DEST\n\
                            file whenever the SOURCE file contains a long enough sequence of zero bytes.\n\
                            Use --sparse=never to inhibit creation of sparse files.")
                .parse_env_or_exit()
                ;
    
    println!("current exec = {:?}", command.get_exec().unwrap());

    if let Some(s) = command.get_str("c") {
        println!("arg c = {}", s);
    }

    if let Some(s) = command.get_str("copy") {
        println!("arg copy = {}", s);
    }

    if let Some(f) = command.get_float("f") {
        println!("arg f = {}", f);
    }

    if let Some(d) = command.get_int("d") {
        println!("arg d = {}", d);
    }

    if let Some(e) = command.get_int("enum") {
        println!("arg enum = {}", e);
    }

    if let Some(l) = command.get_list("list") {
        println!("arg list = {:?}", l);
    }

    if let Some(r) = command.get("r") {
        println!("arg r = {}", r);
    }

}