use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/build_timer.rs"));

#[derive(Debug)]
enum Value {
    Int(i32),
    Float(f32),
    Str(String),
    List(Vec<String>),
}

#[derive(Debug)]
enum Type {
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
                        cur_find == 2;
                    }
                    cur_str = String::new();
                } else if v == ' ' {
                    if cur_find == 2 {
                        long = cur_str;
                    } else if cur_find == 1 {
                        short = cur_str;
                    }
                    cur_str = String::new();
                } else {
                    cur_str += &v.to_string();
                }
            } else {
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
    exec: Option<String>,
    args: Vec<ArgInfo>,
    values: HashMap<String, Value>,
}

impl Commander {
    pub fn new() -> Commander {
        Commander {
            version: String::new(),
            build_time: get_build_time().to_string(),
            usage: vec![],
            usage_desc: String::new(),
            exec: None,
            args: vec![],
            values: HashMap::new(),
        }
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

    pub fn usage_desc(mut self, usage_desc: &str) -> Commander {
        self.usage_desc = usage_desc.to_string();
        self
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

        println!("{}", help);
    }
}