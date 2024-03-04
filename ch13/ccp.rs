use rust_ecommerce_ccp::ecommerce::{
    apply_pricing_strategy, Product, SeasonalDiscount, StandardPricing,
};

fn main() {
    let product = Product {
        name: "Rust Programming Book".to_string(),
        description: "Learn Rust with examples".to_string(),
        base_price: 50.0,
        inventory: 100,
    };

    let standard_pricing = StandardPricing;
    let seasonal_discount = SeasonalDiscount {
        discount_percentage: 10.0,
    };

    println!(
        "Standard Price: ${}",
        apply_pricing_strategy(&standard_pricing, product.base_price)
    );
    println!(
        "Discounted Price: ${}",
        apply_pricing_strategy(&seasonal_discount, product.base_price)
    );
}
