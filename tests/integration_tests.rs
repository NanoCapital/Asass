use asaas_rust::{AsaasService, models::AsaasAccountResponse};
use reqwest;
use serde_json;

#[tokio::test]
async fn test_get_my_account() {
    let api_key = std::env::var("ASAAS_API_KEY").expect("ASAAS_API_KEY deve estar definida");

    let service = AsaasService::new(api_key.to_string());

    // Primeiro, vamos testar manualmente para ver a resposta bruta
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.asaas.com/api/v3/myAccount")
        .header("Content-Type", "application/json")
        .header("User-Agent", "asaas_service")
        .header("access_token", api_key)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let body = response.text().await.unwrap();

    println!("Status: {}", status);
    println!("Body: {}", body);

    // Testar se conseguimos parsear o JSON diretamente
    let json_str = r#"{"object":"account","personType":"FISICA","companyType":null,"company":null,"cpfCnpj":"09625471669","email":"heberth.silva@live.com","responsibleName":null,"phone":null,"mobilePhone":"31997887263","postalCode":"30670050","address":"Rua Coletora","addressNumber":"1208","complement":null,"province":"Vila Pinho Vale do Jatobá (Barreiro)","city":{"object":"city","id":10072,"ibgeCode":"3106200","name":"Belo Horizonte","districtCode":"05","district":"Belo Horizonte","state":"MG"},"inscricaoEstadual":null,"name":"Heberth Fernandes da Silva","birthDate":"1993-07-08","status":"APPROVED","denialReason":null,"incomeValue":1000}"#;

    match serde_json::from_str::<AsaasAccountResponse>(json_str) {
        Ok(parsed) => {
            println!("✅ Parsing JSON direto funcionou!");
            println!("Nome: {}", parsed.name);
            println!("Email: {}", parsed.email);
        }
        Err(e) => {
            panic!("❌ Falha no parsing JSON: {}", e);
        }
    }

    // Testar o método get_my_account através do AsaasService
    let result = service.get_my_account().await;

    match result {
        Ok(account) => {
            println!("✅ Teste bem-sucedido! Informações da conta:");
            println!("Nome: {}", account.name);
            println!("Email: {}", account.email);
            println!("CPF/CNPJ: {:?}", account.cpf_cnpj);
            println!("Status: {:?}", account.status);

            // Verificações básicas
            assert!(!account.name.is_empty(), "Nome não pode estar vazio");
            assert!(!account.email.is_empty(), "Email não pode estar vazio");
            assert!(account.object == "account", "Object deve ser 'account'");
        }
        Err(e) => {
            panic!("❌ Teste falhou: {}", e);
        }
    }
}