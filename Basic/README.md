# Rust C2 Server

This repository contains a Rust implementation of a basic Command and Control (C2) server that can handle multiple asynchronous connections with clients and communicate with them simultaneously. The server periodically reads commands from a `commands.txt` file, sends these commands to clients, and receives and prints the output from each client. Each client is given a unique name and ID to interact with them individually.



## Features

- Asynchronous handling of multiple client connections.
- Simultaneous command execution and output collection from all connected clients.
- Unique names and IDs for each client to facilitate individual interaction.



## Prerequisites

Before running the server, you'll need the following prerequisites:

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.



## Getting Started

```bash
git clone https://github.com/your-username/your-repository.git
```
```
cd RustNecCommander/Basic
```
```
cargo build
```
```
cargo run
```


## Usage

1. Ensure that your clients are configured to connect to the server's IP address and port.

2. Create a `commands.txt` file with the commands you want to execute on the clients. Each line in the file represents a separate command.

3. Place the `commands.txt` file in the same directory as the server executable.

4. The server will periodically read the `commands.txt` file, send the commands to connected clients, and print the output received from each client.


## Configuration

You can configure the server by modifying the `config.toml` file. This file allows you to specify options such as the listening port, communication protocol, and other server settings.


## Client Interaction
Clients are identified by their unique names and IDs, allowing you to interact with specific clients individually. Use the provided client management features to select and interact with a specific client.
