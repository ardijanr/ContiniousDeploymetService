use std::str::from_utf8;
use std::{env, process::exit};
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};


const CURRENT_BRANCH : &str = "rev-parse --abbrev-ref HEAD";
const LOCAL_COMMIT   : &str = "rev-parse HEAD";
const REMOTE_COMMIT  : &str = "ls-remote origin HEAD";
const GIT_RESET : &str = "reset --hard origin/";
const COMPOSE : &str = "docker-compose up --build --headless";

fn main() {

    if new_commit() {

        let rebase_command = format!("{}{}",GIT_RESET,run_capture("git",CURRENT_BRANCH));

        run_capture("git",rebase_command.as_str());

        let result = run_and_output("sudo", COMPOSE);

        if result.is_ok() {
            exit(0)
        } else {
            exit(1)
        }

    }





}


//Checks if remote is ahead of local.
fn new_commit()->bool{
    if run_capture("git",LOCAL_COMMIT) != run_capture("git",REMOTE_COMMIT){

        return true;
    }

    return false;
}

//runs a command, and captures stdout and converts it to string.
fn run_capture(command: &str,command_text : &str) -> String{
    let mut cmd = Command::new(command);
    cmd.args(command_text.split(" "));

    return from_utf8(&cmd.output().unwrap().stdout).unwrap().to_string();
}

//runs a command and continuously provides output from the running process
fn run_and_output(command: &str,args: &str) -> Result<(),std::io::Error> {
    let stdout = Command::new(command)
        .args(args.split(" "))
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| "Command failed").unwrap();

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

     Ok(())
}