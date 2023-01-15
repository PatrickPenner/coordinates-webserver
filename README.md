# Rust/RDKit Coordinate Generation Server

PoC Rust RDKit server to explor how easy it is to implement Rust webservers of
cheminformatic functionality.

## Usage

Start the server with cargo:
```
cargo run
```

Test the server for example with the following command:
```
curl -vvv -H "Content-type: application/json" -d '{"smiles": "c1ccccc1"}' localhost:8080
```
