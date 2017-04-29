## Commander by tickbh

The complete solution for Rust command-line interfaces.

[![Build Status](https://api.travis-ci.org/tickbh/Commander.svg?branch=master)](https://travis-ci.org/tickbh/Commander)

### How to install it?

Add this to the `Cargo.toml` file of your project

```toml
[dependencies]
commander = "0.1"
```

### How to use it?

```rust
extern crate commander;
use td_rlua::Commander;
```

## Option parsing

 Options with commander are defined with the `.option()`,`.option_str()`,`.option_int()`,`.option_float()`,`.option_list()` method. The example below parses args and options from `std::env::args()` or `Vec<String>`, leaving remaining args can get by func `.get()`, `.get_str()`, `.get_int()`, `.get_float()`, `.get_list()`.

```rust
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
                .option_str("-c, --copy [value]", "copy content", Some("source".to_string()))
                .option("-r", "enable recursive", None)
                .parse_env()
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
        println!("arg list = {}", l);
    }

    if let Some(r) = command.get("r") {
        println!("arg r = {}", r);
    }
}
```

  The output info if you build bin is test. follow examples show.
1.  ./test -c xxxx -d
```
arg c = xxxx
arg copy = xxxx
arg d = 123
//arg c and copy is the same arg, and we has d param but we not set the value, it read from default:123
``` 

2. ./test -h #it will show help then exit the program
```
Usage:./test test
Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.

Options:
  -v, --version               Output the version
  -h, --help                  Output this help info
  -l, --list [value]          list		 default:a, b, c
      --enum [value]          enum
  -d, --debug [value]         debug		 default:123
  -c, --copy [value]          拷贝内容		 default:aaa
```

3. ./test -v #it will show version and show build time then exit the program
```
Version:0.0.1
Build Time:2017-04-29T14:11:24+08:00
``` 

4. ./test --enum aa -r --list aa bb cc #we provide enum and list arg
```
arg list = ["aa", "bb", "cc"]
arg r = true
// we has the arg enum, but the enum is not a vaild int, so convert failed, and we provide arg r, so r is true
```

### Contributing

Contributions are welcome!
