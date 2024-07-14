use arikedb::ArikedbClient;
use arikedb::{Variable, VariableType, Epoch, VarEvent, Event};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    let mut client = ArikedbClient::connect(
        "127.0.0.1", // host
        6923,        // port
        false        // use_ssl
    ).await.unwrap();

    client.authenticate("admin", "admin").await.unwrap();

    client.create_collections(&vec!["collection1", "collection2", "collection3", "collection4"]).await.unwrap();

    client.delete_collections(&vec!["collection3", "collection4"]).await.unwrap();

    let collections = client.list_collections().await.unwrap();

    for collection in collections {
        println!("{:?}", collection);
    }

    let variables = vec![
        Variable { name: String::from("var1"), vtype: VariableType::I32, buffer_size: 10 },
        Variable { name: String::from("var2"), vtype: VariableType::I32, buffer_size: 5 },
        Variable { name: String::from("var3"), vtype: VariableType::I32, buffer_size: 4 },
        Variable { name: String::from("var4"), vtype: VariableType::I32, buffer_size: 4 },
    ];

    client.create_variables("collection1", variables).await.unwrap();

    client.delete_variables("collection1", &vec!["var3", "var4"]).await.unwrap();

    let variables = client.list_variables("collection1").await.unwrap();

    for variable in variables {
        println!("{:?}", variable);
    }

    client.set_variables(
        "collection1",
        vec!["var1", "var2"],
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(),
        vec!["-235", "48"],
        Epoch::Microsecond
    ).await.unwrap();

    let derived_order = 0;
    let data = client.get_variables("collection1", vec!["var1", "var2"], derived_order, Epoch::Nanosecond).await.unwrap();

    for point in data {
        println!("{:?}", point);
    }
    println!("--------------------------------------------------");

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
}
