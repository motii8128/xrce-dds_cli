use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn call_command()
{
    let target_arg = std::env::args().nth(1).unwrap();

    let action_arg = std::env::args().nth(2).unwrap();

    if "gen" != target_arg
    {
        return;
    }
    else
    {
        if "set" == action_arg
        {
            let ws_arg = std::env::args().nth(3).unwrap();
            set_gen(ws_arg);
        }    
    }

}

fn set_gen(ws_name:String)
{
    {
        let mut child = Command::new("cd")
            .arg(ws_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to move workspace");

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
            eprintln!("Command Error")
        }
    }

    {
        let mut child = Command::new("git")
            .arg("clone")
            .arg("https://github.com/eProsima/Micro-XRCE-DDS-Gen.git")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed get Gen-tools");

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
            eprintln!("Command Error")
        }
    }

    {
        let mut child = Command::new("cd")
            .arg("Micro-XRCE-DDS-Gen")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to move dir");

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
            eprintln!("Command Error")
        }
    }

    {
        let mut child = Command::new("git")
            .arg("submodule")
            .arg("init")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to initialize submodule");

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
            eprintln!("Command Error")
        }
    }

    {
        let mut child = Command::new("git")
            .arg("submodule")
            .arg("update")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to update submodule");

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
            eprintln!("Command Error")
        }
    }

    {
        let mut child = Command::new("./gradlew")
            .arg("assemble")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed assemble");

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
            eprintln!("Command Error")
        }
    }
}