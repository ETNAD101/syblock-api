/* 
 *TODO
 *maybe add modes taht let you pick between profit, amount of teeth, or amount to spend
 *add auction house support
 *add easy swapping between different bazaar craft flips
 *add support for normal flips
*/

pub mod bazaar;

use std::io;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bazaar = bazaar::get_bazaar_info().await;

    println!("Enter the amount of teeth you want to make");
    let mut amount_of_gteeth = String::new();
    io::stdin()
        .read_line(&mut amount_of_gteeth)
        .expect("Failed to read line");

    let amount_of_gteeth: i64 = amount_of_gteeth.trim().parse().unwrap();

    let wolf_teeth = amount_of_gteeth * 32 * 4;
    let enchanted_gold = amount_of_gteeth * 32;
    
    let wolf_teeth_value = wolf_teeth as f64 * bazaar.products.WOLF_TOOTH.quick_status.buyPrice;
    let enchanted_gold_value = enchanted_gold as f64 * bazaar.products.ENCHANTED_GOLD.quick_status.buyPrice;
    let total_sell_value = amount_of_gteeth as f64 * bazaar.products.GOLDEN_TOOTH.quick_status.buyPrice;
    let total_cost = wolf_teeth_value + enchanted_gold_value;
    let profit = total_sell_value - total_cost;

    println!("Amount to buy");
    println!("Wolf Teeth: {}, cost: {}", wolf_teeth, wolf_teeth_value);
    println!("Enchanted Gold: {}, cost: {}", enchanted_gold, enchanted_gold_value);
    println!("Total cost: {}, Total sell value: {}", total_cost, total_sell_value);
    println!("Total profit: {}", profit);
    
    Ok(())
}
