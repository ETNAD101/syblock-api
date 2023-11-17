use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub productId: String,
    pub sellPrice: f64,
    pub sellVolume: i64,
    pub sellMovingWeek: i64,
    pub sellOrders: i32,
    pub buyPrice: f64,
    pub buyVolume: i64,
    pub buyMovingWeek: i64,
    pub buyOrders: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub quick_status: Status,
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Products {
   pub ENCHANTED_GOLD: Product,
   pub WOLF_TOOTH: Product,
   pub GOLDEN_TOOTH: Product,
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct BazzarInfo {
    pub success: bool,
    pub lastUpdated: i64,
    pub products: Products,
}

pub async fn refresh_bazaar() -> BazzarInfo {
    let request_url = "https://api.hypixel.net/v2/skyblock/bazaar";
    let http_response = reqwest::get(request_url).await.unwrap();
    http_response.json::<BazzarInfo>().await.unwrap()
}
