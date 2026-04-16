use asaas_rust::models::CreatePixPaymentRequest;
use asaas_rust::AsaasService;
use std::env;

#[tokio::test]
async fn test_pix_payment_response() {
    let api_key = env::var("ASAAS_API_KEY").expect("ASAAS_API_KEY deve estar definida");

    let service = AsaasService::new(api_key.to_string(), None);

    // Criar um request de pagamento PIX
    let request = CreatePixPaymentRequest {
        user_id: "test_user_id".to_string(),
        value: 100.0,
        description: Some("Teste de pagamento PIX".to_string()),
        external_reference: Some("test_order_id".to_string()),
        order_id: "test_order_id".to_string(),
        due_date: None,
    };

    // Dados do usuário (apenas para criar customer)
    let user_data = asaas_rust::models::UserData {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        phone: "31999999999".to_string(),
        cpf: "12345678901".to_string(),
        company_name: None,
        city: None,
        state: None,
        postal_code: None,
        address: None,
        address_number: None,
        complement: None,
        province: None,
        disabled: Some(false),
        additional_emails: None,
        municipal_inscription: None,
        state_inscription: None,
        observations: None,
        asaas_customer_id: None,
    };

    // Tentar criar o pagamento PIX
    match service.create_pix_payment_user(request, user_data).await {
        Ok(response) => {
            println!("\n✅ Pagamento PIX criado com sucesso!");
            println!("Payment ID: {}", response.payment_id);
            println!("Status: {}", response.status);
            println!("Valor: R$ {:.2}", response.value);
            println!("Due Date: {}", response.due_date);
            println!("QR Code Base64: {}", response.qr_code_base64);
            println!("Payload: {}", response.payload);
            println!("Expiration Date: {}", response.expiration_date);
        }
        Err(e) => {
            println!("\n❌ Erro ao criar pagamento PIX: {}", e);

            // Se for erro de parsing, vamos tentar ver a resposta bruta
            if e.to_string().contains("Parse error") {
                println!("\n⚠️  Tentando obter resposta bruta da API...");

                // Criar cliente direto para ver a resposta
                let client = reqwest::Client::new();
                let response = client
                    .post("https://www.asaas.com/api/v3/payments")
                    .header("Content-Type", "application/json")
                    .header("User-Agent", "asaas_service")
                    .header("access_token", api_key)
                    .json(&asaas_rust::models::AsaasPaymentRequest {
                        customer: "test".to_string(),
                        billing_type: "PIX".to_string(),
                        value: 100.0,
                        due_date: "2026-03-06".to_string(),
                        description: Some("Teste".to_string()),
                        external_reference: Some("test".to_string()),
                        installment_count: None,
                        installment_value: None,
                        discount: None,
                        interest: None,
                        fine: None,
                        postal_service: Some(false),
                        notify_customer: Some(false),
                    })
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        let status = resp.status();
                        let body = resp
                            .text()
                            .await
                            .unwrap_or_else(|_| "<não foi possível obter corpo>".to_string());

                        println!("\nℹ️  Resposta da API:");
                        println!("Status: {}", status);
                        println!("Body: {}", body);

                        // Tentar parsear o JSON para ver o erro exato
                        match serde_json::from_str::<serde_json::Value>(&body) {
                            Ok(json) => {
                                println!("\nℹ️  JSON parseado:");
                                println!("{}", serde_json::to_string_pretty(&json).unwrap());
                            }
                            Err(parse_err) => {
                                println!("\n❌ Falha ao parsear JSON da resposta: {}", parse_err);
                            }
                        }
                    }
                    Err(req_err) => {
                        println!("\n❌ Erro na requisição direta: {}", req_err);
                    }
                }
            }
        }
    }
}
