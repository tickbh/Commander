use std::collections::HashMap;
use std::fmt;
use std::env;
use std::str::FromStr;


include!(concat!(env!("OUT_DIR"), "/build_timer.rs"));

#[derive(Debug, Clone)]
enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(ref b) => write!(f, "{}", b),
            Value::Int(ref i) => write!(f, "{}", i),
            Value::Float(ref fl) => write!(f, "{}", fl),
            Value::Str(ref s) => write!(f, "{}", s),
            Value::List(ref list) => {
                for (i, v) in list.iter().enumerate() {
                    if i != 0 {
                        let _ = write!(f, ", ");
                    }
                    let _ = write!(f, "{}", v);
                }
                write!(f, "")
            },
        }
    }
}

#[derive(Debug)]
enum Type {
    Bool,
    Int,
    Float,
    Str,
    List,
}

#[derive(Debug)]
struct ArgInfo {
    short: String,
    long: String,
    arg_type : Type,
    default: Option<Value>,
    desc: String,
    arg: String,
}

impl ArgInfo {
    pub fn new(arg: String, desc: String, arg_type : Type, default: Option<Value>) -> ArgInfo {
        let new_arg = arg.clone();
        let mut chars = arg.chars();
        let mut cur_find = 0; //0表示无, 不作处理, 1表示寻找到短字符, 2表示寻找到长字符
        let mut cur_str = String::new();
        let mut short = String::new();
        let mut long = String::new();
        loop {
            if let Some(v) = chars.next() {
                if v == '-' {
                    if cur_find == 0 {
                        cur_find = 1;
                    } else if cur_find == 1 {
                        cur_find = 2;
                    }
                    cur_str = String::new();
                } else if v == ' ' || v == ',' {
                    if cur_find == 2 {
                        long = cur_str;
                    } else if cur_find == 1 {
                        short = cur_str;
                    }
                    cur_find = 0;
                    cur_str = String::new();
                } else {
                    cur_str += &v.to_string();
                }
            } else {
                if cur_str.len() != 0 {
                    if cur_find == 2 {
                        long = cur_str;
                    } else if cur_find == 1 {
                        short = cur_str;
                    }
                }
                break
            }
        }

        ArgInfo {
            short: short,
            long: long,
            arg_type : arg_type,
            default: default,
            desc: desc,
            arg: new_arg,
        }
    }
}

#[derive(Debug)]
pub struct Commander {
    version : String,
    build_time : String,
    usage: Vec<String>,
    usage_desc: String,
    after_desc: String,
    exec: Option<String>,
    args: Vec<ArgInfo>,
    helps: Vec<String>,
    versions: Vec<String>,
    all_args: Vec<String>,
    values: HashMap<String, Value>,
}

impl Commander {
    pub fn new() -> Commander {
        let commander = Commander {
            version: String::new(),
            build_time: get_build_time().to_string(),
            usage: vec![],
            usage_desc: String::new(),
            after_desc: String::new(),
            exec: None,
            args: vec![],
            helps: vec!["h".to_string(), "help".to_string()],
            versions: vec!["v".to_string(), "version".to_string()],
            all_args: vec![],
            values: HashMap::new(),
        };

        commander.option("-v, --version", "Show the bin version and build time", None)
                 .option("-h, --help", "Show this help message and exit", None)
    }

    pub fn version(mut self, version: &str) -> Commander {
        self.version = version.to_string();
        self
    }

    pub fn exec(mut self, exec: &str) -> Commander {
        self.exec = Some(exec.to_string());
        self
    }

    pub fn usage(mut self, usage: &str) -> Commander {
        self.usage.push(usage.to_string());
        self
    }
    
    pub fn helps(mut self, help: Vec<String>) -> Commander {
        self.helps = help;
        self
    }

    pub fn versions(mut self, version: Vec<String>) -> Commander {
        self.versions = version;
        self
    }

    pub fn usage_desc(mut self, usage_desc: &str) -> Commander {
        self.usage_desc = usage_desc.to_string();
        self
    }

    pub fn after_desc(mut self, after_desc: &str) -> Commander {
        self.after_desc = after_desc.to_string();
        self
    }
    
    pub fn get_exec(&self) -> Option<String> {
        self.exec.clone()
    }

    pub fn get_all_args(&self) -> Vec<String> {
        self.all_args.clone()
    }

    pub fn get(&self, arg: &str) -> Option<bool> {
        if let Some(v) = self.values.get(&arg.to_string()) {
            match *v {
                Value::Bool(ref b) => return Some(b.clone()),
                _ => return None,
            };
        }
        None
    }

    pub fn get_str(&self, arg: &str) -> Option<String> {
        if let Some(v) = self.values.get(&arg.to_string()) {
            match *v {
                Value::Str(ref s) => return Some(s.clone()),
                _ => return None,
            };
        }
        None
    }

    pub fn get_int(&self, arg: &str) -> Option<i64> {
        if let Some(v) = self.values.get(&arg.to_string()) {
            match *v {
                Value::Int(ref i) => return Some(i.clone()),
                _ => return None,
            };
        }
        None
    }

    pub fn get_float(&self, arg: &str) -> Option<f64> {
        if let Some(v) = self.values.get(&arg.to_string()) {
            match *v {
                Value::Float(ref f) => return Some(f.clone()),
                _ => return None,
            };
        }
        None
    }

    pub fn get_list(&self, arg: &str) -> Option<Vec<String>> {
        if let Some(v) = self.values.get(&arg.to_string()) {
            match *v {
                Value::List(ref l) => return Some(l.clone()),
                _ => return None,
            };
        }
        None
    }

    pub fn option(mut self, arg: &str, desc: &str, default: Option<bool>) -> Commander {
        let new_default = default.map(|val| Value::Bool(val.clone()));
        let args = ArgInfo::new(arg.to_string(), desc.to_string(), Type::Bool, new_default.clone());
        if let Some(s) = new_default {
            if args.short.len() > 0 {
                self.values.insert(args.short.clone(), s.clone());
            }
            if args.long.len() > 0 {
                self.values.insert(args.long.clone(), s);
            }
        }
        self.args.push(args);
        self
    }

    pub fn option_str(mut self, arg: &str, desc: &str, default: Option<String>) -> Commander {
        let new_default = default.map(|val| Value::Str(val.clone()));
        let args = ArgInfo::new(arg.to_string(), desc.to_string(), Type::Str, new_default.clone());
        if let Some(s) = new_default {
            if args.short.len() > 0 {
                self.values.insert(args.short.clone(), s.clone());
            }
            if args.long.len() > 0 {
                self.values.insert(args.long.clone(), s);
            }
        }
        self.args.push(args);
        self
    }

    pub fn option_int(mut self, arg: &str, desc: &str, default: Option<i64>) -> Commander {
        let new_default = default.map(|val| Value::Int(val.clone()));
        let args = ArgInfo::new(arg.to_string(), desc.to_string(), Type::Int, new_default.clone());
        if let Some(s) = new_default {
            if args.short.len() > 0 {
                self.values.insert(args.short.clone(), s.clone());
            }
            if args.long.len() > 0 {
                self.values.insert(args.long.clone(), s);
            }
        }
        self.args.push(args);
        self
    }

    pub fn option_float(mut self, arg: &str, desc: &str, default: Option<f64>) -> Commander {
        let new_default = default.map(|val| Value::Float(val.clone()));
        let args = ArgInfo::new(arg.to_string(), desc.to_string(), Type::Float, new_default.clone());
        if let Some(s) = new_default {
            if args.short.len() > 0 {
                self.values.insert(args.short.clone(), s.clone());
            }
            if args.long.len() > 0 {
                self.values.insert(args.long.clone(), s);
            }
        }
        self.args.push(args);
        self
    }

    pub fn option_list(mut self, arg: &str, desc: &str, default: Option<Vec<String>>) -> Commander {
        let new_default = default.map(|val| Value::List(val.clone()));
        let args = ArgInfo::new(arg.to_string(), desc.to_string(), Type::List, new_default.clone());
        if let Some(s) = new_default {
            if args.short.len() > 0 {
                self.values.insert(args.short.clone(), s.clone());
            }
            if args.long.len() > 0 {
                self.values.insert(args.long.clone(), s);
            }
        }
        self.args.push(args);
        self
    }

    fn try_analyse_commnad(&mut self, command: &String, args: &Vec<String>) {
        if command.len() == 0 {
            return;
        }

        for v in &self.versions {
            if command == v {
                self.print_version();
                ::std::process::exit(0);
            }
        }
        
        for h in &self.helps {
            if command == h {
                self.print_help();
                ::std::process::exit(0);
            }
        }

        for arg in &self.args {
            if arg.short == *command || arg.long == *command {
                let mut value = arg.default.clone();
                match arg.arg_type {
                    Type::Bool => {
                        if args.len() > 0 {
                            if let Some(i) = bool::from_str(&args[0]).ok() {
                                value = Some(Value::Bool(i));
                            }
                        } else {
                            value = Some(Value::Bool(true));
                        }
                    },
                    Type::Int => {
                        if args.len() > 0 {
                            if let Some(i) = i64::from_str(&args[0]).ok() {
                                value = Some(Value::Int(i));
                            } else {
                                eprintln!("Convert {} to int failed", args[0]);
                            }
                        }
                    },
                    Type::Float => {
                        if args.len() > 0 {
                            if let Some(f) = f64::from_str(&args[0]).ok() {
                                value = Some(Value::Float(f));
                            } else {
                                eprintln!("Convert {} to float failed", args[0]);
                            }
                        }
                    },
                    Type::Str => {
                        if args.len() > 0 {
                            value = Some(Value::Str(args[0].clone()));
                        }
                    },
                    Type::List => {
                        if args.len() > 0 {
                            value = Some(Value::List(args.clone()));
                        }
                    }
                }

                if value.is_some() {
                    if arg.short.len() > 0 {
                        self.values.insert(arg.short.clone(), value.clone().unwrap());
                    }

                    if arg.long.len() > 0 {
                        self.values.insert(arg.long.clone(), value.clone().unwrap());
                    }
                }
            }

        }
        
    }

    pub fn parse_list_or_exit(mut self, mut list: Vec<String>) -> Commander {
        if list.len() > 0 {
            self.exec = Some(list.remove(0));
        }
        self.all_args = list.clone();

        let mut command = String::new();
        let mut new_commnad = None;
        let mut args : Vec<String> = vec![];
        for v in list {
            if v.starts_with("--") || v.starts_with("-") {
                new_commnad = Some(v.trim_start_matches('-').to_string())
            } else {
                args.push(v);
            }

            if new_commnad.is_some() {
                self.try_analyse_commnad(&command, &args);
                command = new_commnad.unwrap();
                args = vec![];
                new_commnad = None;
            }
        }
        if command.len() > 0 {
            self.try_analyse_commnad(&command, &args);
        }

        self
    }

    pub fn parse_env_or_exit(self) -> Commander {
        let args = env::args();
        let mut list = vec![];
        for arg in args {
            list.push(arg.to_string());
        }
        self.parse_list_or_exit(list)
    }


    pub fn print_version(&self) {
        let mut version = String::new();
        version += "Version:";
        version += &self.version;

        version += "\nBuild Time:";
        version += &self.build_time;
        println!("{}", version);
    }

    pub fn print_help(&self) {
        let mut help = String::new();
        help += "Usage:";
        for (i, usage) in self.usage.iter().enumerate() {
            if i != 0 {
                help += "\n      ";
            }
            if self.exec.is_some() {
                help += &format!("{} {}", self.exec.clone().unwrap(), usage);
            } else {
                help += &format!("{}", usage);
            }
        }
        help += &format!("\n{}\n", self.usage_desc);

        help += &format!("\nOptions:\n");
        for (_, arg) in self.args.iter().enumerate() {
            let mut line = String::new();
            if arg.short.len() == 0 {
                line += "      ";
            } else {
                line += "  ";
            }

            line += &arg.arg;
            for _ in line.len() .. 30 {
                line += " ";
            }

            line += &arg.desc;

            if arg.default.is_some() {
                line += "\t\t default:";
                line += &format!("{}", arg.default.as_ref().unwrap());
            }

            line += "\n";

            help += &line;
        }

        help += &self.after_desc;

        println!("{}", help);
    }
}