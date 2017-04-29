extern crate commander;

use commander::Commander;

fn main() {
    let command = Commander::new()
                .version("0.0.1")
                .usage("test")
                .usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
                .option_list("-l, --list [value]", "list", Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]))
                .option_int("--enum [value]", "enum", None)
                .option_int("-d, --debug [value]", "debug", Some(123))
                .option_str("-c, --copy [value]", "拷贝内容", Some("aaa".to_string()))
                // .parse_list(vec![
                //     "aatest".to_string(), "-c".to_string(),
                //     "-d".to_string(), "111111".to_string(), 
                //     "--enum".to_string(), "111".to_string(),
                //     // "-v".to_string(),
                //     // "-h".to_string(),
                // ])
                .parse_env()
                ;

    println!("c = {:?}", command.get_str(&"c".to_string()));
    println!("d = {:?}", command.get_int(&"d".to_string()));
    
    command.print_help();
}