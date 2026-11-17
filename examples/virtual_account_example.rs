use doku_sdk::{
    Customer, DokuClient, GeneratePaymentCodeRequest, Order, VaChannel, VirtualAccountInfo,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = DokuClient::builder()
        .client_id("MCH-0001-DEMO")
        .secret_key("SK-DEMO-SECRET-KEY")
        .sandbox()
        .build()?;

    let request = GeneratePaymentCodeRequest {
        order: Order {
            invoice_number: format!("INV-VA-{}", chrono::Utc::now().timestamp()),
            amount: Some(50000),
            currency: Some("IDR".to_string()),
            ..Default::default()
        },
        virtual_account_info: Some(VirtualAccountInfo {
            reusable_status: Some(true),
            billing_type: Some("NO_BILL".to_string()),
            virtual_account_number: Some("021862040000016".to_string()),
            ..Default::default()
        }),
        customer: Customer {
            name: "Anton Budiman".to_string(),
            email: "anton@example.com".to_string(),
            ..Default::default()
        },
        additional_info: None,
    };

    println!("Generating BCA Virtual Account payment code...");
    match client
        .virtual_account()
        .generate_payment_code(VaChannel::Bca, &request)
        .await
    {
        Ok(res) => {
            if let Some(va_info) = res.virtual_account_info {
                println!("VA Number: {:?}", va_info.virtual_account_number);
                println!("How to pay: {:?}", va_info.how_to_pay_page);
            } else {
                println!("Response: {:?}", res.raw);
            }
        }
        Err(e) => {
            eprintln!("Error generating VA: {}", e);
        }
    }

    Ok(())
}
