use crate::domain::model::ads::Ads;
use rand::Rng;

pub fn get_ads_from_db() -> Vec<Ads> {
    let ads_link = vec![
        "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg".to_string(),
        "https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F8d04466c-cba7-4c82-97c2-f079246825d8_2442x1194.png".to_string(),
        "https://53.fs1.hubspotusercontent-na1.net/hub/53/hubfs/limited-time-offer.jpg?width=1190&height=800&name=limited-time-offer.jpg".to_string(),
        "https://www.lebow.drexel.edu/sites/default/files/styles/landing_wide/public/story/2022-11/time_scarcity_node.jpg.webp?itok=u8fq-G-I".to_string(),
        "https://i.postimg.cc/9M1wmpBj/tesla1.png".to_string(),
        "https://i.postimg.cc/pVfq8h2h/car1.png".to_string(),
        "https://i.postimg.cc/GhbzjL2M/car2.png".to_string(),
        "https://i.postimg.cc/rFgjnBRq/house1.png".to_string(),
        "https://i.postimg.cc/W13nQ50g/house2.png".to_string(),
        "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg".to_string(),
        "https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F8d04466c-cba7-4c82-97c2-f079246825d8_2442x1194.png".to_string(),
        "https://53.fs1.hubspotusercontent-na1.net/hub/53/hubfs/limited-time-offer.jpg?width=1190&height=800&name=limited-time-offer.jpg".to_string(),
        "https://i.postimg.cc/9M1wmpBj/tesla1.png".to_string(),
        "https://i.postimg.cc/pVfq8h2h/car1.png".to_string(),
        "https://i.postimg.cc/GhbzjL2M/car2.png".to_string(),
        "https://i.postimg.cc/rFgjnBRq/house1.png".to_string(),
        "https://i.postimg.cc/W13nQ50g/house2.png".to_string(),
    ];

    let mut rng = rand::thread_rng();
    let mut ads = Vec::new();

    for (i, link) in ads_link.iter().enumerate() {
        let random_price = rng.gen_range(1..=100);

        ads.push(
            Ads::new(
                format!("Special Discount Burger {}", i),
                link.to_string(),
                random_price,
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
