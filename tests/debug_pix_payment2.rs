use std::env;

#[tokio::test]
async fn debug_pix_payment_response_small_value() {
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
    println!("Resposta da busca de customer: {}", search_body);

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

    // Criar pagamento PIX com valor pequeno (R$ 1,00)
    let payment_response = client
        .post("https://www.asaas.com/api/v3/payments")
        .header("Content-Type", "application/json")
        .header("User-Agent", "asaas_service")
        .header("access_token", &api_key)
        .json(&serde_json::json!({
            "customer": customer_id,
            "billingType": "PIX",
            "value": 1.00,
            "dueDate": "2026-03-06",
            "description": "Teste de pagamento PIX valor pequeno",
            "externalReference": "test_small_value_001",
            "postalService": false
        }))
        .send()
        .await
        .unwrap();

    let payment_status = payment_response.status();
    let payment_body = payment_response.text().await.unwrap();

    println!("\n💳 Resposta da criação do pagamento PIX (valor R$ 1,00):");
    println!("Status: {}", payment_status);
    println!("Body: {}", payment_body);

    // Tentar parsear como AsaasPaymentResponse
    match serde_json::from_str::<asaas_rust::models::AsaasPaymentResponse>(&payment_body) {
        Ok(parsed) => {
            println!("\n✅ Parsing como AsaasPaymentResponse funcionou!");
            println!("ID: {}", parsed.id);
            println!("Status: {}", parsed.status);
            println!("Value: {}", parsed.value);
            println!("Billing Type: {}", parsed.billing_type);
            println!("Due Date: {}", parsed.due_date);
            println!("External Reference: {:?}", parsed.external_reference);
        }
        Err(e) => {
            println!("\n❌ Falha ao parsear como AsaasPaymentResponse: {}", e);

            // Tentar parsear como JSON genérico
            match serde_json::from_str::<serde_json::Value>(&payment_body) {
                Ok(json) => {
                    println!("\nℹ️  Estrutura da resposta:");
                    println!("{}", serde_json::to_string_pretty(&json).unwrap());
                }
                Err(_) => {
                    println!("⚠️  Resposta não é JSON válido");
                }
            }
        }
    }

    // Se for sucesso, tentar obter QR Code
    if payment_status.is_success() {
        let json: serde_json::Value = serde_json::from_str(&payment_body).unwrap();
        if let Some(payment_id) = json.get("id").and_then(|v| v.as_str()) {
            let qr_response = client
                .get(format!(
                    "https://www.asaas.com/api/v3/payments/{}/pixQrCode",
                    payment_id
                ))
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
        }
    }
}
