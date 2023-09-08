# Basic C2 Server

The Basic C2 Server is a Rust-based implementation of a basic Command and Control (C2) server that allows you to manage multiple clients, generate unique names and IDs for each client, and execute commands remotely. It periodically reads commands from a file and sends them to all connected clients, collects the output from clients, and prints it.

## Features

- Asynchronous handling of multiple client connections.
- Unique name and ID generation for each client.
- Periodic command execution given the commands in a file and output collection from all connected clients.


# Intermediate C2 Server

The Intermediate C2 Server is a command-line tool implemented in Rust that provides a comprehensive set of features for managing multiple clients, including starting the server, listing connected clients, stopping the manager, and interacting with specific clients.

## Features

- Start the server on port 8080, with each client connection handled in a separate background thread.
- List all connected clients using a HashMap that maps unique names to client IP addresses and TcpStreams.
- Stop the server manager gracefully.
- Interact with specific clients by sending custom commands and receiving output in real-time.