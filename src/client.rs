use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead};

pub fn call_command()
{
    let target_arg = std::env::args().nth(1).unwrap();

    let action_arg = std::env::args().nth(2).unwrap();

    if "client" != target_arg
    {
        return;
    } 

    if "client" == target_arg
    {
        if "set" == action_arg
        {
            let ws_arg = std::env::args().nth(3).unwrap();
            set_client(ws_arg);
        }
        else if "start" == action_arg
        {
            let type_arg = std::env::args().nth(3).unwrap();
            if "demo_pub" == type_arg
            {
                let port_arg = std::env::args().nth(4).unwrap();
                start_demo_publisher(port_arg);
            }
            else if "demo_sub" == type_arg
            {
                let port_arg = std::env::args().nth(4).unwrap();
                start_demo_subscriber(port_arg);
            }
        }
        else if "create"  == action_arg
        {
            let file_arg = std::env::args().nth(3).unwrap();
            create_client(file_arg);
        }
    }
}

fn set_client(ws_name:String)
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
            .arg("https://github.com/eProsima/Micro-XRCE-DDS-Client.git")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed get client");

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
            .arg("Micro-XRCE-DDS-Client")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to move to Micro-XRCE-DDS-Client");

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
            .expect("Failed to make dir");

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
            .arg("-DUCLIENT_BUILD_EXAMPLES=ON")
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
}

fn start_demo_publisher(port:String)
{
    let mut child = Command::new("examples/PublishHelloWorld/PublishHelloWorldClient")
            .arg("127.0.0.1")
            .arg(port)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start demo publisher");

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

fn start_demo_subscriber(port:String)
{
    let mut child = Command::new("examples/SubscribeHelloWorld/SubscribeHelloWorldClient")
            .arg("127.0.0.1")
            .arg(port)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start demo subscriber");

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

fn create_client(file_name:String)
{
    let mut child = Command::new("~/Micro-XRCE-DDS-Gen/scripts/microxrceddsgen")
            .arg("-example")
            .arg(file_name)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to create client");

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