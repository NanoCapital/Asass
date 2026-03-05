use asaas_rust::models::AsaasPaymentResponse;

#[test]
fn test_asaas_payment_response_parsing() {
    let json_str = r#"{
        "id": "pay_000123456789",
        "status": "ACTIVE",
        "value": 100.00,
        "netValue": 98.00,
        "billingType": "PIX",
        "description": "Teste de pagamento PIX",
        "externalReference": "test_order_001",
        "installmentNumber": 1,
        "installmentCount": 1,
        "dueDate": "2026-03-06",
        "dateCreated": "2026-03-05",
        "customer": "cus_000164538552"
    }"#;
    
    match serde_json::from_str::<AsaasPaymentResponse>(json_str) {
        Ok(parsed) => {
            println!("\n✅ Parsing funcionou!");
            println!("ID: {}", parsed.id);
            println!("Status: {}", parsed.status);
            println!("Value: {}", parsed.value);
            println!("Net Value: {:?}", parsed.net_value);
            println!("Billing Type: {}", parsed.billing_type);
            println!("Description: {:?}", parsed.description);
            println!("External Reference: {:?}", parsed.external_reference);
            println!("Installment Number: {:?}", parsed.installment_number);
            println!("Installment Count: {:?}", parsed.installment_count);
            println!("Due Date: {}", parsed.due_date);
            println!("Date Created: {}", parsed.date_created);
            println!("Customer: {}", parsed.customer);
        }
        Err(e) => {
            println!("\n❌ Falha no parsing: {}", e);
            panic!("Teste de parsing falhou");
        }
    }
}

#[test]
fn test_asaas_payment_response_parsing_with_errors() {
    let json_str = r#"{
        "errors": [
            {
                "code": "invalid_value",
                "description": "O valor da cobrança excede o seu limite autorizado. Entre em contato com o suporte para aumentar o limite."
            }
        ]
    }"#;
    
    match serde_json::from_str::<AsaasPaymentResponse>(json_str) {
        Ok(_) => {
            println!("\n❌ Parsing inesperado funcionou!");
            panic!("Deveria falhar ao parsear resposta de erro como AsaasPaymentResponse");
        }
        Err(e) => {
            println!("\n✅ Falha esperada no parsing: {}", e);
            println!("Teste de parsing de erro funcionou como esperado");
        }
    }
}
