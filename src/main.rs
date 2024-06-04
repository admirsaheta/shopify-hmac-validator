use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex::encode;
use std::io::Write;

fn main() {
    let mut secret = String::new();
    let mut query_params = String::new();
    let mut provided_hmac = String::new();

    print!("Enter your secret key from shopify: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut secret).unwrap();
    secret.pop(); 

    print!("Enter the query parameters: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut query_params).unwrap();
    query_params.pop(); 

    print!("Enter the provided HMAC: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut provided_hmac).unwrap();
    provided_hmac.pop(); 

    let computed_hmac = calculate_hmac(&secret, &query_params);

    if computed_hmac == provided_hmac {
        println!("HMAC is valid");
    } else {
        println!("HMAC is invalid");
    }
}

fn calculate_hmac(secret: &str, data: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    encode(code_bytes)
}
