//! Simple example demonstrating the clean ApiResponse API with type-safe mutual exclusivity

use avinapi::transport::response::{ApiResponse, ResponseCode, data, empty, fail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    println!("=== Simple ApiResponse with Either ===\n");

    // 1. Success response with data
    let success = data(User {
        id: 1,
        name: "Alice".to_string(),
    });

    let json = serde_json::to_string_pretty(&success).unwrap();
    println!("Success Response:");
    println!("{}\n", json);

    // 2. Error response
    let error: ApiResponse<User> = fail(ResponseCode::NotFoundError, "User not found");

    let json = serde_json::to_string_pretty(&error).unwrap();
    println!("Error Response:");
    println!("{}\n", json);

    // 3. Empty success response
    let empty_response = empty();

    let json = serde_json::to_string_pretty(&empty_response).unwrap();
    println!("Empty Response:");
    println!("{}\n", json);

    // 4. The beauty: compile-time guarantee of mutual exclusivity
    println!("Key Benefits:");
    println!("✓ Payload enum ensures data XOR message at compile time");
    println!("✓ Clean functions: data(), fail(), empty()");
    println!("✓ No complex methods, just simple data structures");
    println!("✓ Type safety without runtime checks");
}
