use xrce_dds_cli::{agent, client, gen, error};

fn main()
{
    error::no_arg();
    agent::call_command();
    client::call_command();
    gen::call_command();
}