use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasCustomerRequest {
    pub name: String,
    #[serde(rename = "cpfCnpj")]
    pub cpf_cnpj: String,
    pub email: String,
    pub phone: Option<String>,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: Option<String>,
    pub address: Option<String>,
    #[serde(rename = "addressNumber")]
    pub address_number: Option<String>,
    pub complement: Option<String>,
    pub province: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
    #[serde(rename = "notificationDisabled")]
    pub notification_disabled: Option<bool>,
    #[serde(rename = "additionalEmails")]
    pub additional_emails: Option<String>,
    #[serde(rename = "municipalInscription")]
    pub municipal_inscription: Option<String>,
    #[serde(rename = "stateInscription")]
    pub state_inscription: Option<String>,
    pub observations: Option<String>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    pub company: Option<String>,
    #[serde(rename = "foreignCustomer")]
    pub foreign_customer: Option<bool>,
    pub person_type: Option<String>,
    pub company_name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsaasCustomerResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "cpfCnpj")]
    pub cpf_cnpj: Option<String>,
    #[serde(rename = "personType")]
    pub person_type: Option<String>,
    #[serde(rename = "companyName")]
    pub company_name: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<String>,
    #[serde(rename = "addressNumber")]
    pub address_number: Option<String>,
    #[serde(rename = "complement")]
    pub complement: Option<String>,
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
    #[serde(rename = "dateCreated")]
    pub date_created: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasInvoiceRequest {
    #[serde(rename = "customer")]
    pub customer: String,
    #[serde(rename = "serviceDescription")]
    pub service_description: String,
    #[serde(rename = "observations")]
    pub observations: Option<String>,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "installments")]
    pub installments: Option<i32>,
    #[serde(rename = "effectiveDate")]
    pub effective_date: Option<String>,
    #[serde(rename = "installmentValue")]
    pub installment_value: Option<f64>,
    #[serde(rename = "taxes")]
    pub taxes: Option<serde_json::Value>,
    #[serde(rename = "municipalServiceId")]
    pub municipal_service_id: Option<String>,
    #[serde(rename = "municipalServiceCode")]
    pub municipal_service_code: Option<String>,
    #[serde(rename = "municipalServiceName")]
    pub municipal_service_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasInvoiceResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "effectiveDate")]
    pub effective_date: String,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "xmlUrl")]
    pub xml_url: Option<String>,
    #[serde(rename = "pdfUrl")]
    pub pdf_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPaymentRequest {
    #[serde(rename = "customer")]
    pub customer: String,
    #[serde(rename = "billingType")]
    pub billing_type: String,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "dueDate")]
    pub due_date: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
    #[serde(rename = "installmentCount")]
    pub installment_count: Option<i32>,
    #[serde(rename = "installmentValue")]
    pub installment_value: Option<f64>,
    #[serde(rename = "discount")]
    pub discount: Option<serde_json::Value>,
    #[serde(rename = "interest")]
    pub interest: Option<serde_json::Value>,
    #[serde(rename = "fine")]
    pub fine: Option<serde_json::Value>,
    #[serde(rename = "postalService")]
    pub postal_service: Option<bool>,
    #[serde(rename = "notifyCustomer")]
    pub notify_customer: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPaymentResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "netValue")]
    pub net_value: Option<f64>,
    #[serde(rename = "billingType")]
    pub billing_type: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "externalReference")]
    pub external_reference: Option<String>,
    #[serde(rename = "installmentNumber")]
    pub installment_number: Option<i32>,
    #[serde(rename = "installmentCount")]
    pub installment_count: Option<i32>,
    #[serde(rename = "dueDate")]
    pub due_date: String,
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    #[serde(rename = "customer")]
    pub customer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasPixQrCodeResponse {
    #[serde(rename = "encodedImage")]
    pub encoded_image: String,
    #[serde(rename = "payload")]
    pub payload: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasCity {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "ibgeCode")]
    pub ibge_code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "districtCode")]
    pub district_code: String,
    #[serde(rename = "district")]
    pub district: String,
    #[serde(rename = "state")]
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsaasAccountResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "personType")]
    pub person_type: Option<String>,
    #[serde(rename = "companyType")]
    pub company_type: Option<String>,
    #[serde(rename = "company")]
    pub company: Option<String>,
    #[serde(rename = "cpfCnpj")]
    pub cpf_cnpj: Option<String>,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "responsibleName")]
    pub responsible_name: Option<String>,
    #[serde(rename = "phone")]
    pub phone: Option<String>,
    #[serde(rename = "mobilePhone")]
    pub mobile_phone: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<String>,
    #[serde(rename = "addressNumber")]
    pub address_number: Option<String>,
    #[serde(rename = "complement")]
    pub complement: Option<String>,
    #[serde(rename = "province")]
    pub province: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<AsaasCity>,
    #[serde(rename = "inscricaoEstadual")]
    pub inscricao_estadual: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "birthDate")]
    pub birth_date: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "denialReason")]
    pub denial_reason: Option<String>,
    #[serde(rename = "incomeValue")]
    pub income_value: Option<f64>,
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
    pub due_date: Option<String>,
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
    pub asaas_customer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsaasListResponse<T> {
    pub object: String,
    pub has_more: bool,
    pub total_count: i32,
    pub limit: i32,
    pub offset: i32,
    pub data: Vec<T>,
}
