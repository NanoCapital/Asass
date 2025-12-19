use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasCustomerRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub cpf_cnpj: Option<String>,
    pub person_type: Option<String>,
    pub company_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub address_number: Option<String>,
    pub complement: Option<String>,
    pub external_reference: Option<String>,
    pub province: Option<String>,
    pub disabled: Option<bool>,
    pub additional_emails: Option<String>,
    pub municipal_inscription: Option<String>,
    pub state_inscription: Option<String>,
    pub observations: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsaasCustomerResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub mobile_phone: Option<String>,
    pub cpf_cnpj: Option<String>,
    pub person_type: Option<String>,
    pub company_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub address_number: Option<String>,
    pub complement: Option<String>,
    pub external_reference: Option<String>,
    pub date_created: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasInvoiceRequest {
    pub customer: String,
    pub service_description: String,
    pub observations: Option<String>,
    pub value: f64,
    pub installments: Option<i32>,
    pub effective_date: Option<String>,
    pub installment_value: Option<f64>,
    pub taxes: Option<serde_json::Value>,
    pub municipal_service_id: Option<String>,
    pub municipal_service_code: Option<String>,
    pub municipal_service_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasInvoiceResponse {
    pub id: String,
    pub status: String,
    pub effective_date: String,
    pub value: f64,
    pub xml_url: Option<String>,
    pub pdf_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPaymentRequest {
    pub customer: String,
    pub billing_type: String,
    pub value: f64,
    pub due_date: String,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub installment_count: Option<i32>,
    pub installment_value: Option<f64>,
    pub discount: Option<serde_json::Value>,
    pub interest: Option<serde_json::Value>,
    pub fine: Option<serde_json::Value>,
    pub postal_service: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPaymentResponse {
    pub id: String,
    pub status: String,
    pub value: f64,
    pub net_value: Option<f64>,
    pub billing_type: String,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub installment_number: Option<i32>,
    pub installment_count: Option<i32>,
    pub due_date: String,
    pub date_created: String,
    pub customer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPixQrCodeResponse {
    pub encoded_image: String,
    pub payload: String,
    pub expiration_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub user_id: String,
    pub service_description: String,
    pub value: f64,
    pub observations: Option<String>,
    pub effective_date: Option<String>,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {
    pub invoice_id: String,
    pub asaas_invoice_id: String,
    pub status: String,
    pub value: f64,
    pub effective_date: String,
    pub pdf_url: Option<String>,
    pub xml_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePixPaymentRequest {
    pub user_id: String,
    pub value: f64,
    pub description: Option<String>,
    pub external_reference: Option<String>,
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePixPaymentResponse {
    pub payment_id: String,
    pub asaas_payment_id: String,
    pub qr_code_base64: String,
    pub payload: String,
    pub expiration_date: String,
    pub value: f64,
    pub due_date: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub cpf: String,
    pub company_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub address: Option<String>,
    pub address_number: Option<String>,
    pub complement: Option<String>,
    pub province: Option<String>,
    pub disabled: Option<bool>,
    pub additional_emails: Option<String>,
    pub municipal_inscription: Option<String>,
    pub state_inscription: Option<String>,
    pub observations: Option<String>,
}
