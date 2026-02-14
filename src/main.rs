use std::time::Instant;
use looping::calculation;
use crate::looping::CalculationRequest;

mod looping;

fn main() {
    let iterations = 1_000_000;
    
    let mut sum = 0i64;
    
    let start = Instant::now();
    for _ in 0..iterations {
        let req = CalculationRequest {
            price: 1000,
            calculation_type: "PERCENTAGE",
            margin_amount: 0,
            margin_percentage: 0.7
        };
        sum += calculation(req) as i64;  
    }
    let duration = start.elapsed();
    
    println!("Rust 1M iterations: {:?}", duration);
    println!("Per operation: {:?}", duration / iterations);
    println!("Sum (prevent optimization): {}", sum);  
}