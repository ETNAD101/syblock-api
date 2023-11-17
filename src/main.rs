/* 
 *TODO
 *maybe add modes taht let you pick between profit, amount of teeth, or amount to spend
 *add auction house support
 *add easy swapping between different bazaar craft flips
 *add support for normal flips
*/

pub mod bazaar;

use serde::{Serialize, Deserialize};
use bazaar::Product;
use std::fs::File;
use std::io::{self, Write};
use reqwest::Error;

#[derive(Serialize, Deserialize)]
pub struct Ingredients {
    products: Vec<Product>,
    quantity: Vec<i64>
}

impl Ingredients {
    pub fn from(products: Vec<Product>, quantity: Vec<i64>) -> Ingredients {
        Ingredients { 
            products, 
            quantity
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub product: Product,
    pub ingredients: Ingredients,
}

impl Recipe {
    pub fn calculate_profit(&self, amount: i64) -> f64 {
        let sell_value = self.product.quick_status.buyPrice;
        let mut total_cost = 0.0;
        let prod = &self.ingredients.products;
        let quantity = &self.ingredients.quantity;
        for i in 0..prod.len() {
            let cost = prod[i].quick_status.sellPrice * quantity[i] as f64;
            total_cost += cost;
        }
    
        let profit = sell_value - total_cost;
        profit * amount as f64
    }
}



#[tokio::main]
async fn main() -> Result<(), Error> {
    let bazaar = bazaar::refresh_bazaar().await;

    println!("Enter the amount of teeth you want to make");
    let mut amount_of_gteeth = String::new();
    io::stdin()
        .read_line(&mut amount_of_gteeth)
        .expect("Failed to read line");

    let amount_of_gteeth: i64 = amount_of_gteeth.trim().parse().unwrap();


    let golden_tooth = Recipe {
        product: bazaar.products.GOLDEN_TOOTH,
        ingredients: Ingredients::from(
            vec![bazaar.products.ENCHANTED_GOLD, bazaar.products.WOLF_TOOTH],
            vec![32, 128]
        )
    };

    let mut file = File::create("recipies.json").unwrap();
    file.write_all(serde_json::to_string(&golden_tooth).unwrap().as_bytes()).unwrap();

    println!("{}",golden_tooth.calculate_profit(amount_of_gteeth));
    
    Ok(())
}
