use crate::models::{
    AsaasCustomerRequest, AsaasCustomerResponse, AsaasInvoiceRequest, AsaasInvoiceResponse,
    AsaasPaymentRequest, AsaasPaymentResponse, AsaasPixQrCodeResponse,
};
use chrono::Utc;

#[derive(Debug)]
pub struct AsaasProvider {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug)]
pub enum AsaasError {
    RequestError(String),
    ApiError(String),
    ParseError(String),
}

impl std::fmt::Display for AsaasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AsaasError::RequestError(msg) => write!(f, "Request error: {}", msg),
            AsaasError::ApiError(msg) => write!(f, "API error: {}", msg),
            AsaasError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for AsaasError {}

impl AsaasProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://www.asaas.com/api/v3".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn create_customer(
        &self,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        tracing::info!(
            "🌐 Enviando requisição para criar customer no Asaas - name: {}, email: {}",
            customer_data.name,
            customer_data.email
        );

        let response = self
            .client
            .post(format!("{}/customers", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&customer_data)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao criar customer no Asaas: {}",
                error_text
            )));
        }

        let asaas_response: AsaasCustomerResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ Customer criado com sucesso no Asaas - asaas_customer_id: {}",
            asaas_response.id
        );

        Ok(asaas_response)
    }

    pub async fn create_pix_payment(
        &self,
        customer_id: &str,
        value: f64,
        description: Option<String>,
        external_reference: Option<String>,
    ) -> Result<AsaasPaymentResponse, AsaasError> {
        tracing::info!("💳 Criando pagamento PIX via AsaasProvider - customer_id: {}, valor: R$ {:.2}, external_reference: {:?}",
            customer_id, value, external_reference);

        // Preparar a data de vencimento (hoje + 1 dia)
        let due_date = (Utc::now() + chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string();

        // Criar o payload para a API do Asaas
        let asaas_request = AsaasPaymentRequest {
            customer: customer_id.to_string(),
            billing_type: "PIX".to_string(),
            value,
            due_date,
            description,
            external_reference,
            installment_count: None,
            installment_value: None,
            discount: None,
            interest: None,
            fine: None,
            postal_service: Some(false),
        };

        // Fazer a requisição para a API do Asaas
        let response = self
            .client
            .post(format!("{}/payments", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&asaas_request)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro da API Asaas: {}",
                error_text
            )));
        }

        // Parsear a resposta
        let asaas_response: AsaasPaymentResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ Pagamento PIX criado no Asaas - payment_id: {}, status: {}",
            asaas_response.id,
            asaas_response.status
        );

        Ok(asaas_response)
    }

    pub async fn get_pix_qr_code(
        &self,
        payment_id: &str,
    ) -> Result<AsaasPixQrCodeResponse, AsaasError> {
        tracing::info!(
            "📱 Solicitando QR Code PIX do Asaas para payment_id: {}",
            payment_id
        );

        let response = self
            .client
            .get(format!(
                "{}/payments/{}/pixQrCode",
                self.base_url, payment_id
            ))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao obter QR Code PIX do Asaas: {}",
                error_text
            )));
        }

        let asaas_response: AsaasPixQrCodeResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ QR Code PIX obtido - payload_length: {}, expiration: {}",
            asaas_response.payload.len(),
            asaas_response.expiration_date
        );

        Ok(asaas_response)
    }

    pub async fn create_invoice(
        &self,
        customer_id: &str,
        service_description: &str,
        value: f64,
        observations: Option<String>,
        effective_date: Option<String>,
    ) -> Result<AsaasInvoiceResponse, AsaasError> {
        tracing::info!(
            "📄 Criando NF-e via AsaasProvider - customer_id: {}, valor: R$ {:.2}, service_description: {}",
            customer_id, value, service_description
        );

        // Preparar a data de emissão (hoje se não especificada)
        let effective_date = effective_date.unwrap_or_else(|| {
            (Utc::now()).format("%Y-%m-%d").to_string()
        });

        // Criar o payload para a API do Asaas
        let asaas_request = AsaasInvoiceRequest {
            customer: customer_id.to_string(),
            service_description: service_description.to_string(),
            observations,
            value,
            installments: None,
            effective_date: Some(effective_date.clone()),
            installment_value: None,
            taxes: None,
            municipal_service_id: None,
            municipal_service_code: None,
            municipal_service_name: None,
        };

        // Fazer a requisição para a API do Asaas
        let response = self
            .client
            .post(format!("{}/invoices", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&asaas_request)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro da API Asaas ao criar NF-e: {}",
                error_text
            )));
        }

        // Parsear a resposta
        let asaas_response: AsaasInvoiceResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ NF-e criada no Asaas - invoice_id: {}, status: {}, effective_date: {}",
            asaas_response.id,
            asaas_response.status,
            asaas_response.effective_date
        );

        Ok(asaas_response)
    }

    pub async fn get_customer_by_external_reference(
        &self,
        external_ref: &str,
    ) -> Result<Option<AsaasCustomerResponse>, AsaasError> {
        tracing::info!(
            "🔍 Buscando customer no Asaas por external_reference: {}",
            external_ref
        );

        let response = self
            .client
            .get(format!("{}/customers?externalReference={}", self.base_url, external_ref))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            if response.status() == 404 {
                tracing::info!("Customer não encontrado no Asaas para external_reference: {}", external_ref);
                return Ok(None);
            }
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao buscar customer no Asaas: {}",
                error_text
            )));
        }

        // Asaas retorna uma lista, mesmo que filtrada
        let customers: Vec<AsaasCustomerResponse> = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        if customers.is_empty() {
            tracing::info!("Nenhum customer encontrado para external_reference: {}", external_ref);
            Ok(None)
        } else {
            tracing::info!(
                "Customer encontrado no Asaas - asaas_customer_id: {}",
                customers[0].id
            );
            Ok(Some(customers[0].clone()))
        }
    }

    pub async fn get_customer_by_cpf_cnpj(
        &self,
        cpf_cnpj: &str,
    ) -> Result<Option<AsaasCustomerResponse>, AsaasError> {
        tracing::info!(
            "🔍 Buscando customer no Asaas por cpfCnpj: {}",
            cpf_cnpj
        );

        let response = self
            .client
            .get(format!("{}/customers?cpfCnpj={}", self.base_url, cpf_cnpj))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            if response.status() == 404 {
                tracing::info!("Customer não encontrado no Asaas para cpfCnpj: {}", cpf_cnpj);
                return Ok(None);
            }
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao buscar customer no Asaas: {}",
                error_text
            )));
        }

        // Asaas retorna uma lista, mesmo que filtrada
        let customers: Vec<AsaasCustomerResponse> = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        if customers.is_empty() {
            tracing::info!("Nenhum customer encontrado para cpfCnpj: {}", cpf_cnpj);
            Ok(None)
        } else {
            tracing::info!(
                "Customer encontrado no Asaas - asaas_customer_id: {}",
                customers[0].id
            );
            Ok(Some(customers[0].clone()))
        }
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        tracing::info!(
            "✏️ Atualizando customer no Asaas - customer_id: {}, name: {}, email: {}",
            customer_id,
            customer_data.name,
            customer_data.email
        );

        let response = self
            .client
            .put(format!("{}/customers/{}", self.base_url, customer_id))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&customer_data)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e)))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao atualizar customer no Asaas: {}",
                error_text
            )));
        }

        let asaas_response: AsaasCustomerResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ Customer atualizado com sucesso no Asaas - asaas_customer_id: {}",
            asaas_response.id
        );

        Ok(asaas_response)
    }
}