use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn call_command()
{
    let target_arg = std::env::args().nth(1).unwrap();

    let action_arg = std::env::args().nth(2).unwrap();

    if "agent" != target_arg
    {
        return;
    } 

    if "agent" == target_arg
    {
        if "set" == action_arg
        {
            let ws_arg = std::env::args().nth(3).unwrap();
            set_agent(ws_arg);
        }
        else if "start" == action_arg
        {
            let type_arg = std::env::args().nth(3).unwrap();

            if "udp" == type_arg
            {
                let port_arg = std::env::args().nth(4).unwrap();
                start_udp_agent(port_arg);
            }
            else if "serial" == type_arg
            {
                let port_arg = std::env::args().nth(4).unwrap();
                start_serial_agent(port_arg);
            }
        }
    }
}

fn set_agent(ws_name:String)
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
            .arg("https://github.com/eProsima/Micro-XRCE-DDS-Agent.git")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed get agent");

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
            .arg("Micro-XRCE-DDS-Agent")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed cd command");

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
        let mut child = Command::new("mkdir")
            .arg("build")
            .arg("&&")
            .arg("cd")
            .arg("build")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to make directory");

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
        let mut child = Command::new("cmake")
            .arg("..")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to cmake");

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
        let mut child = Command::new("make")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to make build");

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
        let mut child = Command::new("sudo")
            .arg("make")
            .arg("install")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to make install");

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

fn start_udp_agent(port:String)
{
    let mut child = Command::new("MicroXRCEAgent")
            .arg("udp4")
            .arg("-p")
            .arg(port)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to Start UDP Agent");

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

fn start_serial_agent(port:String)
{
    let port_name = "/dev/tty/ACM".to_owned() + &port;

    let mut child = Command::new("MicroXRCEAgent")
            .arg("serial")
            .arg("--dev")
            .arg(port_name)
            .arg("-b")
            .arg("115200")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to Start UDP Agent");

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