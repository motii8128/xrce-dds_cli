# xrce-dds_cli
CLI tools to build Micro-XRCE-DDS (Agent, Client, Gen)

# How to use
cargo version is 1.72.1

[This](https://micro-xrce-dds.docs.eprosima.com/en/latest/introduction.html) site is details about Micro-XRCE-DDS.

make workspace before launch these commands. 
```
mkdir my_ws
```
build
```
cargo build
```

## Agent tools
### Set
```
cargo run agent set <workspace name>
```
### Start UDP Agent
#### UDP version
For example port... 2023
```
cargo run agent start udp <port>
```
#### Serial version
For example... 0
```
cargo run agent start serial <port number>
```

## Client tools
### Set
```
cargo run client set <workspace name>
```
### Create Client
Create client from *.idl file
```
cargo run client create <*.idl file>
```
### Start demo client
For example port... 2023
#### Publisher
```
cargo run client start demo_pub <port>
```
#### Subscriber
```
cargo run client start demo_sub <port>
```
