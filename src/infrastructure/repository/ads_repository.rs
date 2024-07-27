use crate::domain::model::ads::Ads;

pub fn get_ads_from_db() -> Vec<Ads> {
    let mut ads = Vec::new();

    for i in 1..=10 {
        ads.push(
            Ads::new(
                format!("Special Discount Burger {}", i),
                "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg".to_string(),
                i * 100,
                format!("Description for ad {}", i),
                "nayem".to_string(),
                true,
                1000 + i as i64 * 100,
                "Portugal".to_string(),
                0.1 * i as f64,
            )
        );
    }

    ads
}
