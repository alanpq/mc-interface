/*
let mut p = Popen::create(&["ps", "x"], PopenConfig {
    stdout: Redirection::Pipe, ..Default::default()
})?;

// Obtain the output from the standard streams.
let (out, err) = p.communicate(None)?;

if let Some(exit_status) = p.poll() {
    // the process has finished
} else {
    // it is still running, terminate it
    p.terminate()?;
}
*/

use subprocess::{Popen, PopenConfig, Redirection};

use std::env;
use std::fs;
use std::process::Command;

fn remove_chars(s: &str, n: usize) -> String {
    let mut iter = s.chars();
    iter.by_ref().nth(n);
    String::from(iter.as_str())
}

fn main() -> std::io::Result<()> {
    // TODO: error handling
    // TODO: version handling
    let mut path = fs::canonicalize(env::var("SERVER_JAR").expect("No server JAR specified!"))?;

    let path_str = remove_chars(path.to_str().expect("could not trim path string!"), 3);
    path.pop();
    let pwd_str = remove_chars(path.to_str().expect("could not trim path string!"), 3);

    println!(
        "Server jar path: {}",
        path.file_name()
            .expect("failed to extract jar file name!")
            .to_str()
            .expect("failed to convert path to string!")
    );
    println!("Server working directory: {}", path_str);
    println!("Server working directory: {}", pwd_str);

    let mut child = Command::new("java")
        .current_dir(pwd_str)
        .arg("-jar")
        .arg(&env::var("SERVER_JAR").expect("No server JAR specified!"))
        .arg("-nogui")
        .spawn()
        .expect("failed to execute child");

    child.wait().expect("failed to wait for server process");
    Ok(())
}
