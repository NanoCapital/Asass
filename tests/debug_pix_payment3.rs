use asaas_rust::models::{AsaasPaymentRequest, CreatePixPaymentRequest};
use std::env;

#[tokio::test]
async fn debug_pix_payment_response_valid_value() {
    let api_key = env::var("ASAAS_API_KEY").expect("ASAAS_API_KEY deve estar definida");
    
    let client = reqwest::Client::new();
    
    // Buscar customer existente
    let search_response = client
        .get("https://www.asaas.com/api/v3/customers?externalReference=d5613159-1141-4055-9caf-e85e4f73e0e4")
        .header("Content-Type", "application/json")
        .header("User-Agent", "asaas_service")
        .header("access_token", &api_key)
        .send()
        .await
        .unwrap();
    
    let search_body = search_response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&search_body).unwrap();
    let array = if let Some(arr) = json.get("data").and_then(|v| v.as_array()) {
        arr
    } else if let Some(arr) = json.get("customers").and_then(|v| v.as_array()) {
        arr
    } else if let Some(arr) = json.as_array() {
        arr
    } else {
        panic!("Formato de resposta inesperado: {}", search_body);
    };
    
    let customer_id = array[0]["id"].as_str().unwrap().to_string();
    println!("✅ Customer ID: {}", customer_id);
    
    // Criar pagamento PIX com valor válido (R$ 100,00)
    let payment_response = client
        .post("https://www.asaas.com/api/v3/payments")
        .header("Content-Type", "application/json")
        .header("User-Agent", "asaas_service")
        .header("access_token", &api_key)
        .json(&serde_json::json!({
            "customer": customer_id,
            "billingType": "PIX",
            "value": 100.00,
            "dueDate": "2026-03-06",
            "description": "Teste de pagamento PIX valor válido",
            "externalReference": "test_valid_value_001",
            "postalService": false
        }))
        .send()
        .await
        .unwrap();
    
    let payment_status = payment_response.status();
    let payment_body = payment_response.text().await.unwrap();
    
    println!("\n💳 Resposta da criação do pagamento PIX (valor R$ 100,00):");
    println!("Status: {}", payment_status);
    println!("Body: {}", payment_body);
    
    // Se for sucesso, tentar parsear e obter QR Code
    if payment_status.is_success() {
        match serde_json::from_str::<asaas_rust::models::AsaasPaymentResponse>(&payment_body) {
            Ok(parsed) => {
                println!("\n✅ Parsing como AsaasPaymentResponse funcionou!");
                println!("ID: {}", parsed.id);
                println!("Status: {}", parsed.status);
                println!("Value: {}", parsed.value);
                println!("Billing Type: {}", parsed.billing_type);
                println!("Due Date: {}", parsed.due_date);
                println!("External Reference: {:?}", parsed.external_reference);
                
                // Obter QR Code
                let qr_response = client
                    .get(format!("https://www.asaas.com/api/v3/payments/{}/pixQrCode", parsed.id))
                    .header("Content-Type", "application/json")
                    .header("User-Agent", "asaas_service")
                    .header("access_token", &api_key)
                    .send()
                    .await
                    .unwrap();
                
                let qr_status = qr_response.status();
                let qr_body = qr_response.text().await.unwrap();
                
                println!("\n📱 Resposta do QR Code PIX:");
                println!("Status: {}", qr_status);
                println!("Body: {}", qr_body);
                
                match serde_json::from_str::<asaas_rust::models::AsaasPixQrCodeResponse>(&qr_body) {
                    Ok(qr_parsed) => {
                        println!("\n✅ Parsing como AsaasPixQrCodeResponse funcionou!");
                        println!("Encoded Image (primeiros 100 chars): {}", &qr_parsed.encoded_image[..100.min(qr_parsed.encoded_image.len())]);
                        println!("Payload: {}", qr_parsed.payload);
                        println!("Expiration: {}", qr_parsed.expiration_date);
                    }
                    Err(e) => {
                        println!("\n❌ Falha ao parsear QR Code: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("\n❌ Falha ao parsear como AsaasPaymentResponse: {}", e);
            }
        }
    } else {
        println!("\n⚠️  Resposta de erro, não tentou parsear como AsaasPaymentResponse");
    }
}
