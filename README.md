
# ArikeDB Rust Library

Welcome to the ArikeDB Rust library! This library provides an interface to interact with the Arike Database, an advanced real-time database solution. This documentation will guide you through the process of setting up and using the library effectively.

## Getting Started

### Installation

To use the ArikeDB Rust library, add it to your `Cargo.toml`:

```toml
[dependencies]
arikedb = "*"
tokio = {version = "*", features = ["full"]}
```

### Connecting to ArikeDB

To connect to an ArikeDB server instance, bring `ArikedbClient` into scope and instantiate it by calling the `connect` function.

#### Basic Connection

```rust
use arikedb::ArikedbClient;
use arikedb::{Variable, VariableType, Epoch, VarEvent, Event};

#[tokio::main]
async fn main() {
    let mut client = ArikedbClient::connect(
        "127.0.0.1", // host
        6923,        // port
        false,       // use_ssl
        None,        // ca_cert
        None,        // client_cert
        None         // client_key
    ).await.unwrap();
}
```

#### Connection with Authentication

If the server requires authentication, you need to authenticate after connecting.

```rust
use arikedb::ArikedbClient;
use arikedb::{Variable, VariableType, Epoch, VarEvent, Event};

#[tokio::main]
async fn main() {
    let mut client = ArikedbClient::connect(
        "127.0.0.1", // host
        6923,        // port
        false        // use_ssl
        None,        // ca_cert
        None,        // client_cert
        None         // client_key
    ).await.unwrap();

    client.authenticate("username", "password").await.unwrap();
}
```

### Creating Collections

ArikeDB organizes data into collections. Each collection has a name and a set of variables. To create multiple collections in a single call:

```rust
client.create_collections(&vec!["collection1", "collection2", "collection3"]).await.unwrap();
```

### Deleting Collections

```rust
client.delete_collections(&vec!["collection2", "collection3"]).await.unwrap();
```

### Listing Collections

```rust
let collections = client.list_collections().await.unwrap();

for collection in collections {
    println!("{:?}", collection);
}
```
Output:
```
Collection { name: "collection1" }
Collection { name: "collection2" }
```

### Creating Variables

```rust
let variables = vec![
    Variable { name: String::from("var1"), vtype: VariableType::I32, buffer_size: 10 },
    Variable { name: String::from("var2"), vtype: VariableType::I32, buffer_size: 5 },
    Variable { name: String::from("var3"), vtype: VariableType::I32, buffer_size: 4 },
    Variable { name: String::from("var4"), vtype: VariableType::I32, buffer_size: 4 },
];

client.create_variables("collection1", variables).await.unwrap();
```

### Deleting Variables

```rust
client.delete_variables("collection1", &vec!["var3", "var4"]).await.unwrap();
```

### Listing Variables

```rust
let variables = client.list_variables("collection1").await.unwrap();

for variable in variables {
    println!("{:?}", variable);
}
```
Output:
```
Variable { name: "var1", vtype: I32, buffer_size: 10 }
Variable { name: "var2", vtype: I32, buffer_size: 5 }
```

### Setting Variables Values

```rust
client.set_variables(
    "collection1",
    vec!["var1", "var2"],
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
    vec!["-235", "48"],
    Epoch::Microsecond
).await.unwrap();
```

### Getting Variables Values

```rust
let derived_order = 0;
let data = client.get_variables("collection1", vec!["var1", "var2"], derived_order, Epoch::Nanosecond).await.unwrap();

for point in data {
    println!("{:?}", point);
}
```
Output:
```
DataPoint { name: "var1", vtype: I32, timestamp: "1720927547279880000", epoch: Nanosecond, value: "-235" }
DataPoint { name: "var2", vtype: I32, timestamp: "1720927547279880000", epoch: Nanosecond, value: "48" }
```

### Subscribe to Variables Events
Events are generated over variables when they ar set and event condition happens.

```rust
tokio::spawn(
    client.subscribe_variables(
        "collection1",
        vec!["var1", "var2"],
        vec![
            VarEvent { event: Event::OnRise, ..Default::default() },
            VarEvent { event: Event::OnValueEqVal, value: String::from("56"), ..Default::default() },
        ],
        |point| {
            println!("{:?}", point);
        }
    ).await.unwrap()
);

for i in 0..10 {
    client.set_variables(
        "collection1",
        vec!["var1", "var2"],
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
        vec![(50 + i).to_string().as_str(), (60 - i).to_string().as_str()],
        Epoch::Microsecond
    ).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}
```
Output:
```
DataPoint { name: "var1", vtype: I32, timestamp: "1720927881619292000", epoch: Nanosecond, value: "50" }
DataPoint { name: "var2", vtype: I32, timestamp: "1720927881619292000", epoch: Nanosecond, value: "60" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927882629171000", epoch: Nanosecond, value: "51" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927883638571000", epoch: Nanosecond, value: "52" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927884652619000", epoch: Nanosecond, value: "53" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927885664589000", epoch: Nanosecond, value: "54" }
DataPoint { name: "var2", vtype: I32, timestamp: "1720927885664589000", epoch: Nanosecond, value: "56" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927886673866000", epoch: Nanosecond, value: "55" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927887693254000", epoch: Nanosecond, value: "56" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927888703026000", epoch: Nanosecond, value: "57" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927889717038000", epoch: Nanosecond, value: "58" }
DataPoint { name: "var1", vtype: I32, timestamp: "1720927890728150000", epoch: Nanosecond, value: "59" }
```