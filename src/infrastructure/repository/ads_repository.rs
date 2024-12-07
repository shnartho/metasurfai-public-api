use crate::{
    domain::model::ads::{Ads, CreateAdResponse},
    infrastructure::repository::mongodb_repo::MongodbRepository,
};
use rand::Rng;

pub struct AdsRepository {
    db: MongodbRepository,
}

impl AdsRepository {
    pub fn new(db: MongodbRepository) -> Self {
        AdsRepository {db}
    }

    pub async fn fetch_ads_from_db(&self) -> Result<Vec<Ads>, Box<dyn std::error::Error>> {
        let ads: Vec<Ads> = self.db.get_ads_from_mongo_db().await?;
        Ok(ads)
    }

    pub async fn create_ads_in_db(
        &self,
        ad: Ads,
    ) -> Result<CreateAdResponse, Box<dyn std::error::Error>> {
        let ads: CreateAdResponse = self.db.create_ads_in_db(ad).await?;
        Ok(ads)
    }

    pub async fn delete_ads_in_db(&self, id: String) -> Result<(), Box<dyn std::error::Error>> {
        self.db.delete_ads_by_id(&id).await?;
        Ok(())
    }

    pub async fn generate_ads(&self) -> Vec<Ads> {
        let ads_links = vec![
            "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg",
            "https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F8d04466c-cba7-4c82-97c2-f079246825d8_2442x1194.png",
            "https://53.fs1.hubspotusercontent-na1.net/hub/53/hubfs/limited-time-offer.jpg?width=1190&height=800&name=limited-time-offer.jpg",
            "https://www.lebow.drexel.edu/sites/default/files/styles/landing_wide/public/story/2022-11/time_scarcity_node.jpg.webp?itok=u8fq-G-I",
            "https://i.postimg.cc/9M1wmpBj/tesla1.png",
            "https://i.postimg.cc/pVfq8h2h/car1.png",
            "https://i.postimg.cc/GhbzjL2M/car2.png",
            "https://i.postimg.cc/rFgjnBRq/house1.png",
            "https://i.postimg.cc/W13nQ50g/house2.png",
            "https://i.postimg.cc/wTr6w5GD/burger-ads.jpg",
            "https://substackcdn.com/image/fetch/f_auto,q_auto:good,fl_progressive:steep/https%3A%2F%2Fsubstack-post-media.s3.amazonaws.com%2Fpublic%2Fimages%2F8d04466c-cba7-4c82-97c2-f079246825d8_2442x1194.png",
            "https://53.fs1.hubspotusercontent-na1.net/hub/53/hubfs/limited-time-offer.jpg?width=1190&height=800&name=limited-time-offer.jpg",
            "https://i.postimg.cc/9M1wmpBj/tesla1.png",
            "https://i.postimg.cc/pVfq8h2h/car1.png",
            "https://i.postimg.cc/GhbzjL2M/car2.png",
            "https://i.postimg.cc/rFgjnBRq/house1.png",
            "https://i.postimg.cc/W13nQ50g/house2.png",
        ];

        let ad_titles = vec![
            "Burger Special Offer",
            "Latest Tech News",
            "Limited Time Offer!",
            "Time Scarcity Promotion",
            "Tesla Model S Ad",
            "Luxury Car Sale",
            "Sports Car Promotion",
            "House for Sale in Prime Location",
            "Luxury House for Sale",
            "Burger Special Offer",
            "Latest Tech News",
            "Limited Time Offer!",
            "Tesla Model S Ad",
            "Luxury Car Sale",
            "Sports Car Promotion",
            "House for Sale in Prime Location",
            "Luxury House for Sale",
        ];

        let ad_descriptions = vec![
            "Get our special discount on burgers for a limited time. Don't miss out!",
            "Stay updated with the latest tech news and trends in the industry.",
            "Grab this amazing limited-time offer before it’s too late!",
            "Take advantage of our time-limited promotions and save big!",
            "Advertise your Tesla Model S with a competitive price and premium features.",
            "Purchase this luxury car at an unbeatable price, with top-notch features.",
            "Check out this sports car with top speed and performance at a discounted rate.",
            "A beautiful house for sale in a prime location, perfect for families.",
            "Exclusive listing of a luxury house in a high-end neighborhood. Don't miss out!",
            "Get our special discount on burgers for a limited time. Don't miss out!",
            "Stay updated with the latest tech news and trends in the industry.",
            "Grab this amazing limited-time offer before it’s too late!",
            "Advertise your Tesla Model S with a competitive price and premium features.",
            "Purchase this luxury car at an unbeatable price, with top-notch features.",
            "Check out this sports car with top speed and performance at a discounted rate.",
            "A beautiful house for sale in a prime location, perfect for families.",
            "Exclusive listing of a luxury house in a high-end neighborhood. Don't miss out!",
        ];

        let mut rng = rand::thread_rng();
        let mut ads = Vec::with_capacity(ads_links.len());

        for (i, link) in ads_links.iter().enumerate() {
            let random_price = rng.gen_range(1..=100);
            let ad = Ads::new(
                "test_uuid".to_string(), 
                ad_titles[i].to_string(),
                link.to_string(),
                random_price,
                ad_descriptions[i].to_string(),
                "nayem".to_string(),
                true,
                1000 + i as i64 * 100,
                "Portugal".to_string(),
                0.1 * i as f64,
            );
            ads.push(ad);
        }

        ads
    }
}
