// struct CalculationRequest<'a> {
//     pub price: i32,
//     pub calculation_type: &'a str, 
//     pub margin_amount: i32,
//     pub margin_percentage: f32
// }

// fn calculation(req: CalculationRequest) -> i32 {
//     match req.calculation_type { 
//         "FIXED" => req.price + req.margin_amount,
//         "PERCENTAGE" => {
//             let margin = (req.price as f32 * req.margin_percentage / 100.0).round() as i32;
//             req.price + margin
//         }
//         "HYBRID" => {
//             let margin = (req.price as f32 * req.margin_percentage / 100.0).round() as i32;
//             req.price + req.margin_amount + margin
//         }
//         _ => req.price
//     }
// }