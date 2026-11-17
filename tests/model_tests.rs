use doku_sdk::models::*;

#[test]
fn test_checkout_request_serialization() {
    let req = CheckoutRequest {
        order: Order {
            invoice_number: "INV-20261117-001".to_string(),
            amount: Some(10500),
            currency: Some("IDR".to_string()),
            callback_url: Some("https://example.com/finish".to_string()),
            auto_redirect: Some(true),
            line_items: Some(vec![LineItem {
                name: "DOKU T-Shirt".to_string(),
                price: 10500,
                quantity: 1,
            }]),
            session_id: Some("session-abc".to_string()),
        },
        payment: Some(CheckoutPayment {
            payment_due_date: Some(60),
            payment_method_types: None,
        }),
        customer: Customer {
            name: "Anton Budiman".to_string(),
            email: "anton@doku.com".to_string(),
            phone: Some("+6285694566147".to_string()),
            address: Some("Plaza Asia Office Park Unit 3".to_string()),
            country: Some("ID".to_string()),
        },
        additional_info: None,
    };

    let json = serde_json::to_string(&req).expect("Failed to serialize CheckoutRequest");
    let val: serde_json::Value = serde_json::from_str(&json).expect("Failed to parse back to JSON");

    assert_eq!(val["order"]["invoice_number"], "INV-20261117-001");
    assert_eq!(val["order"]["amount"], 10500);
    assert_eq!(val["customer"]["name"], "Anton Budiman");
    assert_eq!(val["payment"]["payment_due_date"], 60);
}

#[test]
fn test_virtual_account_request_serialization() {
    let req = GeneratePaymentCodeRequest {
        order: Order {
            invoice_number: "INV-VA-001".to_string(),
            amount: Some(50000),
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
            email: "anton@doku.com".to_string(),
            ..Default::default()
        },
        additional_info: None,
    };

    let json = serde_json::to_string(&req).expect("Failed to serialize VA request");
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(val["virtual_account_info"]["reusable_status"], true);
    assert_eq!(val["virtual_account_info"]["billing_type"], "NO_BILL");
    assert_eq!(
        val["virtual_account_info"]["virtual_account_number"],
        "021862040000016"
    );
}

#[test]
fn test_snap_create_va_serialization() {
    let req = SnapCreateVaRequest {
        partner_service_id: "02186".to_string(),
        customer_no: Some("2040000016".to_string()),
        virtual_account_no: Some("021862040000016".to_string()),
        virtual_account_name: "Testing VA".to_string(),
        virtual_account_email: Some("testing@doku.com".to_string()),
        virtual_account_phone: Some("6281200001111".to_string()),
        trx_id: "INV-SNAP-001".to_string(),
        total_amount: SnapAmount::idr("12500.00"),
        virtual_account_trx_type: Some("1".to_string()),
        expired_date: Some("2026-11-17T12:00:00+07:00".to_string()),
        additional_info: None,
    };

    let json = serde_json::to_string(&req).expect("Failed to serialize SNAP create VA");
    let val: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(val["partnerServiceId"], "02186");
    assert_eq!(val["virtualAccountNo"], "021862040000016");
    assert_eq!(val["totalAmount"]["value"], "12500.00");
    assert_eq!(val["totalAmount"]["currency"], "IDR");
}

#[test]
fn test_snap_response_deserialization() {
    let raw_resp = r#"{
        "responseCode": "2002700",
        "responseMessage": "Successful",
        "virtualAccountData": {
            "partnerServiceId": "02186",
            "customerNo": "2040000016",
            "virtualAccountNo": "021862040000016",
            "virtualAccountName": "Testing VA",
            "trxId": "INV-SNAP-001",
            "totalAmount": {
                "value": "12500.00",
                "currency": "IDR"
            }
        }
    }"#;

    let parsed: SnapVaResponse =
        serde_json::from_str(raw_resp).expect("Failed to parse SnapVaResponse");
    assert_eq!(parsed.response_code, "2002700");
    assert_eq!(parsed.response_message, "Successful");
    assert_eq!(
        parsed
            .virtual_account_data
            .as_ref()
            .and_then(|d| d.virtual_account_no.as_deref()),
        Some("021862040000016")
    );
}
