extern crate commander;

use commander::Commander;

fn main() {
    let command = Commander::new()
                .version(&env!("CARGO_PKG_VERSION").to_string())
                .usage("test")
                .usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
                .option_list("-l, --list [value]", "list", Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]))
                .option_int("--enum [value]", "enum", None)
                .option_int("-d, --debug [value]", "debug", Some(123))
                .option_str("-c, --copy [value]", "拷贝内容", Some("aaa".to_string()))
                .option("-r", "enable recursive", None)
                // .parse_list(vec![
                //     "aatest".to_string(), "-c".to_string(),
                //     "-d".to_string(), "111111".to_string(), 
                //     "--enum".to_string(), "111".to_string(),
                //     // "-v".to_string(),
                //     // "-h".to_string(),
                // ])
                .parse_env_or_exit()
                ;

    if let Some(s) = command.get_str("c") {
        println!("arg c = {}", s);
    }

    if let Some(s) = command.get_str("copy") {
        println!("arg copy = {}", s);
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