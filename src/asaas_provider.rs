use crate::models::{
    AsaasAccountResponse, AsaasCustomerRequest, AsaasCustomerResponse, AsaasInvoiceRequest,
    AsaasInvoiceResponse, AsaasListResponse, AsaasPaymentRequest, AsaasPaymentResponse,
    AsaasPixQrCodeResponse,
};

use chrono::Utc;
use reqwest::Response;
use serde::de::DeserializeOwned;
use serde_json::Value;

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
    pub fn new(api_key: String, sandbox: Option<bool>) -> Self {
        let base_url = if sandbox.unwrap_or(false) {
            "https://sandbox.asaas.com/api/v3".to_string()
        } else {
            "https://www.asaas.com/api/v3".to_string()
        };

        Self {
            api_key,
            base_url,
            client: reqwest::Client::new(),
        }
    }

    fn extract_error_messages(json_value: &Value) -> String {
        let mut messages = Vec::new();

        if let Some(errors) = json_value.get("errors").and_then(|v| v.as_array()) {
            for err in errors {
                if let Some(desc) = err.get("description").and_then(|v| v.as_str()) {
                    messages.push(desc.to_string());
                }

                if let Some(code) = err.get("code").and_then(|v| v.as_str()) {
                    messages.push(format!("Código: {}", code));
                }

                if let Some(msg) = err.get("message").and_then(|v| v.as_str()) {
                    messages.push(msg.to_string());
                }
            }
        }

        for key in ["error", "errorMessage", "message", "description"] {
            if let Some(v) = json_value.get(key) {
                if let Some(text) = v.as_str() {
                    messages.push(text.to_string());
                }
            }
        }

        if messages.is_empty() {
            "Erro desconhecido da API Asaas".to_string()
        } else {
            messages.join("; ")
        }
    }

    async fn parse_success_response<T>(
        &self,
        response: Response,
        context: &str,
    ) -> Result<T, AsaasError>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        let raw = response.text().await.map_err(|e| {
            AsaasError::RequestError(format!("{} - erro lendo body da resposta: {}", context, e))
        })?;

        tracing::warn!("API-ASAAS RAW [{}] {} => {}", status.as_u16(), context, raw);

        if !status.is_success() {
            return Err(AsaasError::ApiError(format!(
                "{} - Status {} - {}",
                context,
                status.as_u16(),
                raw
            )));
        }

        let json_value: Value = serde_json::from_str(&raw).map_err(|e| {
            AsaasError::ParseError(format!(
                "{} - JSON inválido [{}]: {} | RAW: {}",
                context,
                status.as_u16(),
                e,
                raw
            ))
        })?;

        if json_value.get("errors").is_some()
            || json_value.get("error").is_some()
            || json_value.get("errorMessage").is_some()
        {
            let msg = Self::extract_error_messages(&json_value);

            return Err(AsaasError::ApiError(format!(
                "{} - erro retornado com status {}: {}",
                context,
                status.as_u16(),
                msg
            )));
        }

        serde_json::from_value::<T>(json_value).map_err(|e| {
            AsaasError::ParseError(format!(
                "{} - erro parseando model [{}]: {} | RAW: {}",
                context,
                status.as_u16(),
                e,
                raw
            ))
        })
    }

    pub async fn create_customer(
        &self,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        let payload = serde_json::to_string_pretty(&customer_data)
            .unwrap_or_else(|_| "payload inválido".to_string());

        tracing::info!("API-ASAAS: criando customer:\n{}", payload);

        let response = self
            .client
            .post(format!("{}/customers", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&customer_data)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "create_customer")
            .await
    }

    pub async fn create_pix_payment(
        &self,
        customer_id: &str,
        value: f64,
        description: Option<String>,
        external_reference: Option<String>,
        due_date: Option<String>,
    ) -> Result<AsaasPaymentResponse, AsaasError> {
        let due_date = due_date.unwrap_or_else(|| {
            (Utc::now() + chrono::Duration::days(10))
                .format("%Y-%m-%d")
                .to_string()
        });

        let request = AsaasPaymentRequest {
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
            notify_customer: Some(false),
        };

        let response = self
            .client
            .post(format!("{}/payments", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "create_pix_payment")
            .await
    }

    pub async fn create_billing_payment(
        &self,
        customer_id: &str,
        value: f64,
        description: Option<String>,
        external_reference: Option<String>,
        due_date: Option<String>,
    ) -> Result<AsaasPaymentResponse, AsaasError> {
        let due_date = due_date.unwrap_or_else(|| {
            (Utc::now() + chrono::Duration::days(10))
                .format("%Y-%m-%d")
                .to_string()
        });

        let request = AsaasPaymentRequest {
            customer: customer_id.to_string(),
            billing_type: "BOLETO".to_string(),
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
            notify_customer: Some(false),
        };

        let response = self
            .client
            .post(format!("{}/payments", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "create_billing_payment")
            .await
    }
    pub async fn get_pix_qr_code(
        &self,
        payment_id: &str,
    ) -> Result<AsaasPixQrCodeResponse, AsaasError> {
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
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "get_pix_qr_code")
            .await
    }

    pub async fn create_invoice(
        &self,
        customer_id: &str,
        service_description: &str,
        value: f64,
        observations: Option<String>,
        effective_date: Option<String>,
    ) -> Result<AsaasInvoiceResponse, AsaasError> {
        let effective_date =
            effective_date.unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

        let request = AsaasInvoiceRequest {
            customer: customer_id.to_string(),
            service_description: service_description.to_string(),
            observations,
            value,
            installments: None,
            effective_date: Some(effective_date),
            installment_value: None,
            taxes: None,
            municipal_service_id: None,
            municipal_service_code: None,
            municipal_service_name: None,
            sandbox: Some(true),
        };

        let response = self
            .client
            .post(format!("{}/invoices", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "create_invoice")
            .await
    }

    pub async fn get_customer_by_external_reference(
        &self,
        external_ref: &str,
    ) -> Result<Option<AsaasCustomerResponse>, AsaasError> {
        let response = self
            .client
            .get(format!(
                "{}/customers?externalReference={}",
                self.base_url, external_ref
            ))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        let list: AsaasListResponse<AsaasCustomerResponse> = self
            .parse_success_response(response, "get_customer_by_external_reference")
            .await?;

        Ok(list.data.into_iter().next())
    }

    pub async fn get_customer_by_cpf_cnpj(
        &self,
        cpf_cnpj: &str,
    ) -> Result<Option<AsaasCustomerResponse>, AsaasError> {
        let response = self
            .client
            .get(format!("{}/customers?cpfCnpj={}", self.base_url, cpf_cnpj))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        let list: AsaasListResponse<AsaasCustomerResponse> = self
            .parse_success_response(response, "get_customer_by_cpf_cnpj")
            .await?;

        Ok(list.data.into_iter().next())
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        let response = self
            .client
            .put(format!("{}/customers/{}", self.base_url, customer_id))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .json(&customer_data)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "update_customer")
            .await
    }

    pub async fn get_my_account(&self) -> Result<AsaasAccountResponse, AsaasError> {
        let response = self
            .client
            .get(format!("{}/myAccount", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| AsaasError::RequestError(e.to_string()))?;

        self.parse_success_response(response, "get_my_account")
            .await
    }
}
