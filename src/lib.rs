
include!(concat!(env!("OUT_DIR"), "/build_timer.rs"));

struct Commander {
    version : String,
    build_time : String,

}

impl Commander {
    pub fn new() -> Commander {
        Commander {
            version: String::new(),
            build_time: get_build_time().to_string(),
        }
    }
}