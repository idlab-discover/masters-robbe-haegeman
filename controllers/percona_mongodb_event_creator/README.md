# Event creator for Percona MongoDB operator
Based on [mongodbSpammer](https://github.com/idlab-discover/wasm-operator/tree/main/controllers/mongodbSpammer)

## Setup
See `cargo run -- -h` for the help command.  
Run `kubectl port-forward svc/minimal-cluster-mongos 27017:27017 -n default` to access the mongodb
Run `cargo run -- --db-uri "mongodb://databaseAdmin:databaseAdminPassword@localhost:27017/admin?ssl=false"`
