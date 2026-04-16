use crate::models::{
    AsaasAccountResponse, AsaasCustomerRequest, AsaasCustomerResponse, AsaasInvoiceRequest,
    AsaasInvoiceResponse, AsaasListResponse, AsaasPaymentRequest, AsaasPaymentResponse,
    AsaasPixQrCodeResponse,
};

use chrono::Utc;
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
        let base_url = if let Some(true) = sandbox {
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

    // Função auxiliar para extrair mensagens de erro da resposta da API
    fn extract_error_messages(json_value: &Value) -> String {
        if let Some(errors) = json_value.get("errors").and_then(|e| e.as_array()) {
            let messages: Vec<String> = errors
                .iter()
                .filter_map(|err| {
                    err.get("description")
                        .and_then(|d| d.as_str())
                        .map(|s| s.to_string())
                })
                .collect();
            if messages.is_empty() {
                "Erro desconhecido da API Asaas".to_string()
            } else {
                messages.join("; ")
            }
        } else if let Some(error) = json_value
            .get("error")
            .or_else(|| json_value.get("message"))
        {
            if let Some(msg) = error.as_str() {
                msg.to_string()
            } else {
                "Erro da API Asaas (formato não reconhecido)".to_string()
            }
        } else {
            "Resposta de erro da API Asaas sem mensagem detalhada".to_string()
        }
    }

    pub async fn create_customer(
        &self,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        let payload = serde_json::to_string_pretty(&customer_data)
            .map_err(|e| AsaasError::ParseError(format!("JSON serialize error: {}", e)))?;
        tracing::info!(" API-ASAAS: 📤 Customer payload to Asaas:\n{}", payload);

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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao criar customer no Asaas - Status: {}, Detalhes: {}",
                status.as_u16(),
                error_text
            )));
        }

        let asaas_response: AsaasCustomerResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!(
                "Erro ao parsear resposta Asaas - Status: {}: {}",
                status.as_u16(),
                e
            ))
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
        due_date: Option<String>,
    ) -> Result<AsaasPaymentResponse, AsaasError> {
        tracing::info!(" API-ASAAS: 💳 Criando pagamento PIX via AsaasProvider - customer_id: {}, valor: R$ {:.2}, external_reference: {:?}", customer_id, value, external_reference);

        let due_date_new = due_date.unwrap_or_else(|| {
            (Utc::now() + chrono::Duration::days(10))
                .format("%Y-%m-%d")
                .to_string()
        });

        // Criar o payload para a API do Asaas
        let asaas_request = AsaasPaymentRequest {
            customer: customer_id.to_string(),
            billing_type: "PIX".to_string(),
            value,
            due_date: due_date_new,
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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| {
            AsaasError::RequestError(format!("Erro ao ler resposta da API Asaas: {}", e))
        })?;

        // Log da resposta bruta para debug
        tracing::debug!(
            "Resposta bruta da API Asaas (status {}): {}",
            status.as_u16(),
            response_text
        );

        // Verificar se há erros na resposta (mesmo que status seja 200)
        if let Ok(json_value) = serde_json::from_str::<Value>(&response_text) {
            if json_value.get("errors").is_some() || json_value.get("error").is_some() {
                let error_messages = Self::extract_error_messages(&json_value);
                return Err(AsaasError::ApiError(format!(
                    "Erro da API Asaas (Status: {}): {}",
                    status.as_u16(),
                    error_messages
                )));
            }
        }

        // Se não há erros, tentar parsear como AsaasPaymentResponse
        let asaas_response: AsaasPaymentResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                AsaasError::ParseError(format!(
                    "Erro ao parsear resposta Asaas: {} - resposta: {}",
                    e, response_text
                ))
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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

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
        let effective_date =
            effective_date.unwrap_or_else(|| (Utc::now()).format("%Y-%m-%d").to_string());

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
            sandbox: Some(true),
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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

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
            .get(format!(
                "{}/customers?externalReference={}",
                self.base_url, external_ref
            ))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

        if !response.status().is_success() {
            if response.status() == 404 {
                tracing::info!(
                    "Customer não encontrado no Asaas para external_reference: {}",
                    external_ref
                );
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
        let status_code = response.status().as_u16();

        let list: AsaasListResponse<AsaasCustomerResponse> =
            response.json().await.map_err(|e| {
                AsaasError::ParseError(format!(
                    "Erro ao parsear resposta Asaas - Status: {}: {}",
                    status_code, e
                ))
            })?;

        if list.data.is_empty() {
            tracing::info!(
                "Nenhum customer encontrado para external_reference: {}",
                external_ref
            );
            Ok(None)
        } else {
            tracing::info!(
                "Customer encontrado no Asaas - asaas_customer_id: {}",
                list.data[0].id
            );
            Ok(Some(list.data[0].clone()))
        }
    }

    pub async fn get_customer_by_cpf_cnpj(
        &self,
        cpf_cnpj: &str,
    ) -> Result<Option<AsaasCustomerResponse>, AsaasError> {
        tracing::info!(
            " API-ASAAS: 🔍 Buscando customer no Asaas por cpfCnpj: {}",
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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

        if !response.status().is_success() {
            if response.status() == 404 {
                tracing::info!(
                    "Customer não encontrado no Asaas para cpfCnpj: {}",
                    cpf_cnpj
                );
                return Ok(None);
            }
            let status = response.status().as_u16();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao buscar customer no Asaas - Status: {}, Detalhes: {}",
                status, error_text
            )));
        }

        // Asaas retorna uma lista, mesmo que filtrada
        let status_code = response.status().as_u16();

        let list: AsaasListResponse<AsaasCustomerResponse> =
            response.json().await.map_err(|e| {
                AsaasError::ParseError(format!(
                    "Erro ao parsear resposta Asaas - Status: {}: {}",
                    status_code, e
                ))
            })?;

        if list.data.is_empty() {
            tracing::info!(
                " API-ASAAS: Nenhum customer encontrado para cpfCnpj: {}",
                cpf_cnpj
            );
            Ok(None)
        } else {
            tracing::info!(
                "Customer encontrado no Asaas - asaas_customer_id: {}",
                list.data[0].id
            );
            Ok(Some(list.data[0].clone()))
        }
    }

    pub async fn update_customer(
        &self,
        customer_id: &str,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, AsaasError> {
        let payload = serde_json::to_string_pretty(&customer_data)
            .map_err(|e| AsaasError::ParseError(format!("JSON serialize error: {}", e)))?;
        tracing::info!(
            " API-ASAAS: 📤 Update customer payload to Asaas:\n{}",
            payload
        );

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
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

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

    pub async fn get_my_account(&self) -> Result<AsaasAccountResponse, AsaasError> {
        tracing::info!(" API-ASAAS: 📋 Obtendo informações da conta Asaas...");
        tracing::info!(" API-ASAAS: 🔐 API Key: {}", &self.api_key);

        let response = self
            .client
            .get(format!("{}/myAccount", self.base_url))
            .header("Content-Type", "application/json")
            .header("User-Agent", "asaas_service")
            .header("access_token", &self.api_key)
            .send()
            .await
            .map_err(|e| {
                AsaasError::RequestError(format!("Erro na requisição para Asaas: {}", e))
            })?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Erro desconhecido".to_string());
            return Err(AsaasError::ApiError(format!(
                "Erro ao obter informações da conta Asaas: {}",
                error_text
            )));
        }

        let asaas_response: AsaasAccountResponse = response.json().await.map_err(|e| {
            AsaasError::ParseError(format!("Erro ao parsear resposta Asaas: {}", e))
        })?;

        tracing::info!(
            "✅ Informações da conta obtidas - name: {}, email: {}",
            asaas_response.name,
            asaas_response.email
        );

        Ok(asaas_response)
    }
}
