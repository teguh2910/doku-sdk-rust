use doku_sdk::{
    CheckoutPayment, CheckoutRequest, Customer, DokuClient, LineItem, Order,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client
    let client = DokuClient::builder()
        .client_id("MCH-0001-DEMO")
        .secret_key("SK-DEMO-SECRET-KEY")
        .sandbox()
        .build()?;

    // 2. Prepare checkout payload
    let request = CheckoutRequest {
        order: Order {
            invoice_number: format!("INV-{}", chrono::Utc::now().timestamp()),
            amount: Some(90000),
            currency: Some("IDR".to_string()),
            callback_url: Some("https://myshop.com/payment/callback".to_string()),
            auto_redirect: Some(true),
            line_items: Some(vec![
                LineItem {
                    name: "Rust T-Shirt Red".to_string(),
                    price: 45000,
                    quantity: 1,
                },
                LineItem {
                    name: "Rust T-Shirt Blue".to_string(),
                    price: 45000,
                    quantity: 1,
                },
            ]),
            session_id: Some("session-12345".to_string()),
        },
        payment: Some(CheckoutPayment {
            payment_due_date: Some(60), // 60 minutes
            payment_method_types: None,
        }),
        customer: Customer {
            name: "Anton Budiman".to_string(),
            email: "anton@example.com".to_string(),
            phone: Some("+628123456789".to_string()),
            address: Some("Sudirman Central Business District".to_string()),
            country: Some("ID".to_string()),
        },
        additional_info: None,
    };

    println!("Initiating DOKU Checkout payment...");
    // 3. Call Checkout Service
    match client.checkout().initiate_payment(&request).await {
        Ok(resp) => {
            if let Some(payment) = resp.response.and_then(|r| r.payment) {
                println!("Success! Payment URL: {:?}", payment.url);
                println!("Token: {:?}", payment.token);
            } else {
                println!("Response: {:?}", resp.raw);
            }
        }
        Err(e) => {
            eprintln!("Checkout request error: {}", e);
        }
    }

    Ok(())
}
