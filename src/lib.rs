pub mod agent;
pub mod client;
pub mod error;
pub mod gen;

use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn call_command()
{
    let target_arg = std::env::args().nth(1).unwrap();

    let action_arg = std::env::args().nth(2).unwrap();

    if "lib" != target_arg
    {
        return;
    }
    else
    {
        if "set" == action_arg
        {
            set_lib();
        }    
    }

}

fn set_lib()
{
    {
        let mut child = Command::new("sudo")
            .arg("ldconfig")
            .arg("/usr/local/lib")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to set lib");

        if let Some(ref mut stdout) = child.stdout
        {
            let reader = BufReader::new(stdout);

            for line in reader.lines()
            {
                println!("{}", line.unwrap());
            }
        }

        let status = child.wait().expect("Failed to wait");
        if !status.success()
        {
            eprintln!("Command Error");
        }
    }
}