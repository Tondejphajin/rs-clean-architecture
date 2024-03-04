pub mod ecommerce {
    pub struct Product {
        pub name: String,
        pub description: String,
        pub base_price: f64,
        pub inventory: u32,
    }

    pub trait PricingStrategy {
        fn final_price(&self, base_price: f64) -> f64;
    }

    pub struct StandardPricing;

    impl PricingStrategy for StandardPricing {
        fn final_price(&self, base_price: f64) -> f64 {
            base_price // No discount applied
        }
    }

    pub struct SeasonalDiscount {
        pub discount_percentage: f64,
    }

    impl PricingStrategy for SeasonalDiscount {
        fn final_price(&self, base_price: f64) -> f64 {
            base_price * (1.0 - self.discount_percentage / 100.0)
        }
    }

    // Function to apply pricing strategy to a product
    pub fn apply_pricing_strategy(strategy: &dyn PricingStrategy, base_price: f64) -> f64 {
        strategy.final_price(base_price)
    }
}
