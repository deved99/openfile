use std::collections::HashMap;
use std::env::{args,var};
use std::fmt::Debug;
use std::fs::File;
use std::process::{Command, Stdio};

// json parsing
use serde::Deserialize;
use serde_json as json;

const CONF_PATH: &str = "/.config/openfile/openfile.json";

fn main() {
    let config = get_config();
    let argv = args();
    for i in argv.skip(1) {
	open(&i, &config)
    }
}

fn get_config() -> Config {
    let f_path = format!("{}{}", var("HOME").unwrap(), CONF_PATH);
    let f = File::open(f_path).unwrap();
    match json::from_reader(f) {
	Err(why) => panic!("Error parsing {} as json: {}", CONF_PATH, why),
	Ok(j) => j
    }
}

fn open(f: &str, conf: &Config) {
    let ext = match f.rfind(".") {
	None => "",
	Some(i) => f.split_at(i+1).1
    };
    let kind = &conf.filetypes[ext];
    let mut cmd = conf.commands[kind].clone();
    cmd.push(f.to_string());
    let program = &cmd[0];
    let args = &cmd[1..];
    println!("EXEC: {}\nARGS: {:?}", program, args);
    match Command::new(program)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() {
	    Err(why) => panic!("Error executing {:?}: {}", &cmd, why),
	    Ok(_) => ()
	};
}

#[derive(Deserialize, Debug, Clone)]
struct Config {
    filetypes: HashMap<String, String>,
    commands: HashMap<String, Vec<String>>
}
