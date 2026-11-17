use doku_sdk::{DokuClient, SnapAmount, SnapBank, SnapCreateVaRequest};

const SAMPLE_RSA_PRIVATE_KEY_PEM: &str = r#"-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCx1sG5T84ty8oA
W4IgQIOsR2hmWoI4xSn6TDrr/WsF5Z53vm35Hbj4lqvsRGEvqSzIExapGKe64TP3
dLeMvCfpcixHsm05Dzdxfv2Wd9XWSE3XsexNH8HGpSE56aNsmU1se4l96zkujJOs
iBfwwo4wDQcDp57o9T4tJOXdk827DA2zsQXBBTcaDZ6iqsixYUvJXa76SFUtyXC+
Sm9rk7n6bomSCoSuEs6FiKQ6rNSQSU4UQD4PpNPv+g6Aeem0MIBD+2kAOncXSzbv
eMxd4BLooypOJaMI/bZxfqkiLXUo3ll0kopjmvzN6KAxJgTEOok64EMjREhm9l9J
loBdXpUXAgMBAAECggEAYzoXl2VO/kCB8lccsRAuJ/cj6lA/7pscQ/s6uSgKCh1H
oTqQs/oAljvDq62vqF81jGESOMF3/Q7g5wapz43Am0HIMSz+QmMaJWqc5dLxboGj
ZWFyHspEXakPc/fH5rVW/NulKKwRmMZ5M0AwU3bKzUbHjiN9yC/mRFwdIdfxZ5Zx
BfKL8X9d2OmzGnR/76PFJrjFKnPVKLSQZyBvMJgk/INrJ5F2t7Twcmf9EnHScDWI
BtCd4EmIuejLMB4kC7lCcWNCOL9DBNL2ka9YeLZ34tjE4Itvowtz1hgdNcAkoFAt
wMRxUf9vVqkukqt/0c37UAsrwjXFIAfRkSKatV0WGQKBgQDWxuDPKwsCeY6VAY0f
aa/kKbSBgau2Sgsgx0yNvtCx2/gK1eKuHScXbCer8sXYVPO9nwNkynMEO5pec4vV
fhmleDrKH18kYXafNeTuXV4gotnm51OWOaQZkwIBnUV8BxnMlL9gicIn4bFNrtaW
P+MmHafacjScJr3li+DTAApWawKBgQDT+Ou6Mck7c2guAv2RDl9CH6o267Sr2fcU
29uKSHchp0qmrMcFfA27NMqC4zsYIw41wnIOrGXwh0jg1UHHqjswGoyO3ED4VCec
7AiHOd97As3y48Mvm5liYx5LeRRJML5zvBH14W490NUHXOsSNWpJ49EnoxkLbcS9
h9R30LfvBQKBgAuHdX3pc5fYapz5DlhHVhfLXVMuYMdSRjKmQCfDvHQvuKrsPFTY
X8bUGXUTfSdVuBfPt2DrtSze/jRfw3vcXwgQ4icDdBoB2VpGjFl3TZw24beRZ11D
DtoImCz6AkbcIrhmq7dE7dUyOlBPOFmsaEcdhGsngkd3lO4DijgJYoStAoGBALZ5
hWvO6f1sttVNK72Qkr5Yhe4rcGe0BjiFn3NmOG/XFLNw7w8woJ10r5SHSlrX6qaq
0gcBr+ukej9aNftM9xDM+g0NXDyqLUOwCRNn5ZN+cY5TNbapCxygWPb9r1HmTT1S
PF/fhUhfmtNw0MfnBa3AGVA23htxjjjeUjAmlACFAoGBAKxnvtLNDatmsuvxkKo4
zUvrDVc4lRCZBXZvY6HXnaK9BLeyJ16Hb8yz9Ig222gKr4PAJCW5DOJKbHqHuGhZ
AAjXo3wdH4AmODAh/kkysZlyQ2pY52r0C7xjsIeEGJbrwIBLWL3tEd3Hsjwnj6gd
MdU1l0PsqOEv0kYbuKHVaGmm
-----END PRIVATE KEY-----"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize client with RSA private key for SNAP authentication
    let client = DokuClient::builder()
        .client_id("MCH-0003-SNAP")
        .secret_key("SK-SNAP-SECRET-KEY")
        .private_key(SAMPLE_RSA_PRIVATE_KEY_PEM)
        .sandbox()
        .build()?;

    println!("Requesting SNAP B2B Access Token using RSA SHA256withRSA...");
    // 2. Fetch B2B Token
    let token_res = match client.snap().get_b2b_token().await {
        Ok(t) => {
            println!("Got B2B Access Token: {}", t.access_token);
            t.access_token
        }
        Err(e) => {
            eprintln!("Token request failed: {}", e);
            return Ok(());
        }
    };

    // 3. Create SNAP VA (Mandiri)
    let request = SnapCreateVaRequest {
        partner_service_id: "02186".to_string(),
        customer_no: Some("2040000016".to_string()),
        virtual_account_no: Some("021862040000016".to_string()),
        virtual_account_name: "Testing VA".to_string(),
        virtual_account_email: Some("testing@doku.com".to_string()),
        virtual_account_phone: Some("6281200001111".to_string()),
        trx_id: format!("INV-{}", chrono::Utc::now().timestamp()),
        total_amount: SnapAmount::idr("12500.00"),
        virtual_account_trx_type: Some("1".to_string()),
        expired_date: Some("2026-11-17T23:59:59+07:00".to_string()),
        additional_info: None,
    };

    println!("Creating SNAP Mandiri Virtual Account...");
    match client
        .snap()
        .create_va(SnapBank::Mandiri, &request, &token_res)
        .await
    {
        Ok(res) => {
            println!("Response code: {}", res.response_code);
            println!("Response message: {}", res.response_message);
            if let Some(va_data) = res.virtual_account_data {
                println!("VA Number: {:?}", va_data.virtual_account_no);
            }
        }
        Err(e) => {
            eprintln!("Failed to create SNAP VA: {}", e);
        }
    }

    Ok(())
}
