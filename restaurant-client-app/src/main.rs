use tokio::task::{JoinHandle};
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use serde_json::json;
use reqwest::Client;
use futures::future::join_all;
use rand::Rng;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderItem {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<ObjectId>,
    pub order_id: String,
    pub item_id: String,
    pub quantity: i32,
    pub price: i32,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: Option<ObjectId>,
    pub order_id: isize,
    pub ordered_items: Vec<OrderItem>,
    pub table_no: i32,
    pub order_status: String,
    pub total_amount: f64,
    pub waiting_time: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut rng = rand::thread_rng();
  let paths = vec![
    "http://localhost:8000/menu".to_string(),
    "http://localhost:8000/menu/603876d72c19a8cde68a6a13".to_string(),
    "http://localhost:8000/menu/603db0e200b2d4a400c41e6e".to_string(),
    "http://localhost:8000/menu/60387eef2c19a8cde68a6a15".to_string(),
    "http://localhost:8000/menu".to_string(),
    "http://localhost:8000/menu/60387fad2c19a8cde68a6a17".to_string(),
    "http://localhost:8000/menu".to_string(),
];


let create_order = json!({"table_no":rng.gen_range(0..100),
    "order_status": "Order Placed",
    "ordered_items":[{
        "item_id":"603882eb2c19a8cde68a6a1e",
        "quantity": 1,
        "price": 300 
    }, 
    {
        "item_id":"603881c62c19a8cde68a6a1b",
        "quantity": 1,
        "price": 480
    }, 
    {
        "item_id":"603db0e200b2d4a400c41e6e",
        "quantity": 1,
        "price":920 
    }] });
    let request_url = "http://localhost:8000/order/";
    let response = Client::new()
        .post(request_url)
        .json(&create_order)
        .send().await?;
    
    let order: Order = response.json().await?;

    println!("Created order id: {}",order.order_id);



// Iterate over the paths.
let mut tasks: Vec<JoinHandle<Result<(), ()>>>= vec![];
for path in paths {
    let path = path.clone();
    // Create a Tokio task for each path
    tasks.push(tokio::spawn(async move {
        match reqwest::get(&path).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => {
                        println!("RESPONSE: {} bytes from {}", text.len(), path);
                    }
                    Err(_) => println!("ERROR reading {}", path),
                }
            }
            Err(_) => println!("ERROR downloading {}", path),
        }
        Ok(())
    }));
}

// Wait for them all to finish
println!("Started {} tasks. Waiting...", tasks.len());
join_all(tasks).await;

    let update_item = json!({
        "_id": {
            "$oid": "603876d72c19a8cde68a6a13"
        },
        "title": "PIZZA",
        "itemType": "Vegetarian",
        "timeToPrepare": 15,
        "price": 800,
        "name": "Five Pepper",
        "description": "Mozarella Cheese, Jalapenos,Tomato Sauce, Red Paprika, Capsicum, Bell Pepper"
    });
    let base_path = "http://localhost:8000/menu/";
    let request_url = format!("{}/603876d72c19a8cde68a6a13", base_path);
    let response = Client::new()
        .put(&request_url) 
        .json(&update_item)
        .send().await?;
   if response.status().is_success() {
            println!("Item updated in Menu");
    }  
    
    let create_order = json!({"table_no":rng.gen_range(0..100),
    "order_status": "Order Placed",
    "ordered_items":[{
        "item_id":"603882eb2c19a8cde68a6a1e",
        "quantity": 1,
        "price": 300 
    }, 
    {
        "item_id":"603881c62c19a8cde68a6a1b",
        "quantity": 1,
        "price": 480
    }, 
    {
        "item_id":"603db0e200b2d4a400c41e6e",
        "quantity": 1,
        "price":920 
    }] });
    let request_url = "http://localhost:8000/order/";
    let response = Client::new()
        .post(request_url)
        .json(&create_order)
        .send().await?;
    
    let order: Order = response.json().await?;

    println!("Created order id: {}",order.order_id);

  Ok(())
}

