use std::str::from_utf8;
use std::process::{Command, Stdio, exit};
use std::io::{BufRead, BufReader};


const CURRENT_BRANCH : &str = "rev-parse --abbrev-ref HEAD";
const LOCAL_COMMIT   : &str = "rev-parse HEAD";
const REMOTE_COMMIT  : &str = "ls-remote origin HEAD";
const GIT_RESET : &str = "reset --hard origin/";
const COMPOSE : &str = "docker-compose up --build --detach";


//TODO test


fn main() {

    if new_commit() {

        let rebase_command = format!("{}{}",GIT_RESET,run_capture("git",CURRENT_BRANCH).trim());

        let _ = run_and_output("git",rebase_command.as_str());
        let _ = run_and_output("git","pull");

        //TODO check if build was successful first with docker build, before we start docker-compose

        let result = run_and_output("sudo", COMPOSE);

        if result.is_ok() {
            println!("Successfully deployed new version");
            exit(0);
        } else {
            println!("Unable to deploy new changes, check logs");
            //log stdout
            exit(1);
        }
    }

    println!("No changes to pull!");
    exit(1);
}


//Checks if remote is ahead of local.
fn new_commit()->bool{
    println!("{:?}", run_capture("ls","./"));
    let tmp  = run_capture("git",LOCAL_COMMIT);
    let local : Vec<&str> = tmp.split_ascii_whitespace().collect();

    let tmp  = run_capture("git",REMOTE_COMMIT);
    let remote : Vec<&str> = tmp.split_ascii_whitespace().collect();

    if remote.len() == 0 {
        println!{"check repository or pull permissions for the user"}
        exit(1)
    }

    println!("{:?}\n{:?}",local[0],remote[0]    );

    if local[0] != remote[0] {
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