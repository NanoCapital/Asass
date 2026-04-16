use std::time::Duration;

use crate::asaas_provider::AsaasProvider;
use crate::models::{
    AsaasAccountResponse, AsaasCustomerRequest, AsaasCustomerResponse, CreateInvoiceRequest,
    CreateInvoiceResponse, CreatePixPaymentRequest, CreatePixPaymentResponse, UserData,
};

pub struct AsaasService {
    asaas_provider: AsaasProvider,
}

impl AsaasService {
    pub fn new(api_key: String, sandbox: Option<bool>) -> Self {
        Self {
            asaas_provider: AsaasProvider::new(api_key, sandbox),
        }
    }

    pub async fn create_pix_payment_user(
        &self,
        request: CreatePixPaymentRequest,
        user_data: UserData,
    ) -> Result<CreatePixPaymentResponse, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(
            "🔍 Iniciando processo PIX para user_id: {}, order_id: {}, valor: R$ {:.2}",
            request.user_id,
            request.order_id,
            request.value
        );

        // Verificar se o customer já possui asaas_customer_id
        let customer_id = if let Some(existing_customer_id) = &user_data.asaas_customer_id {
            tracing::info!(
                "👤 Customer já possui asaas_customer_id: {}, pulando criação",
                existing_customer_id
            );
            existing_customer_id.clone()
        } else {
            // Criar customer no Asaas usando os dados do usuário
            let cpf = user_data.cpf.clone();
            let name = user_data.name.clone();
            let email = user_data.email.clone();
            let phone = user_data.phone.clone();
            tracing::info!(
                "👤 Criando customer no Asaas para user_id: {}, cpf: {}",
                request.user_id,
                user_data.cpf
            );

            let customer_data = AsaasCustomerRequest {
                name: name.clone(),
                email: email.clone(),
                phone: Some(phone.clone()),
                mobile_phone: Some(phone.clone()),
                cpf_cnpj: cpf.clone(),
                person_type: Some("FISICA".to_string()),
                company: user_data.company_name.clone(),
                city: user_data.city.clone(),
                state: user_data.state.clone(),
                country: Some("Brasil".to_string()),
                postal_code: user_data.postal_code.clone(),
                address: user_data.address.clone(),
                address_number: user_data.address_number.clone(),
                complement: user_data.complement.clone(),
                province: user_data.province.clone(),
                external_reference: Some(request.user_id.clone()),
                disabled: Some(false),
                additional_emails: user_data.additional_emails.clone(),
                municipal_inscription: user_data.municipal_inscription.clone(),
                state_inscription: user_data.state_inscription.clone(),
                observations: user_data.observations.clone(),
                notification_disabled: Some(true),
                foreign_customer: Some(false),
                group_name: None,
                company_name: user_data.company_name.clone(),
            };

            let customer = self.upsert_customer(customer_data).await?;

            // Force update CPF if provided
            if !cpf.is_empty() {
                tracing::info!(
                    " API-ASAAS: 🔄 Force updating customer CPF: {}",
                    user_data.cpf
                );
                let update_data = AsaasCustomerRequest {
                    cpf_cnpj: cpf.clone(),
                    name: name.clone(),
                    email: email.clone(),
                    phone: Some(phone.clone()),
                    person_type: Some("FISICA".to_string()),
                    ..Default::default()
                };
                self.asaas_provider
                    .update_customer(&customer.id, update_data)
                    .await?;
            }

            customer.id.clone()
        };

        // Criar cobrança PIX no Asaas
        tracing::info!(
            "💳 Criando cobrança PIX no Asaas - customer_id: {}, valor: R$ {:.2}",
            customer_id,
            request.value
        );
        let payment = self
            .asaas_provider
            .create_pix_payment(
                &customer_id,
                request.value,
                request.description,
                Some(request.order_id.clone()),
                request.due_date.clone(),
            )
            .await?;
        tracing::info!(
            "✅ Cobrança PIX criada - asaas_payment_id: {}, status: {}",
            payment.id,
            payment.status
        );

        // Obter QR Code PIX
        tracing::info!(" API-ASAAS: 📱 Obtendo QR Code PIX do Asaas...");
        tokio::time::sleep(Duration::from_millis(500)).await;
        let pix_qr = self.asaas_provider.get_pix_qr_code(&payment.id).await?;
        tracing::info!(
            "✅ QR Code PIX obtido - payload_length: {}, expiration: {}",
            pix_qr.payload.len(),
            pix_qr.expiration_date
        );

        let pix_payment = CreatePixPaymentResponse {
            payment_id: payment.id.clone(),
            asaas_payment_id: payment.id,
            qr_code_base64: pix_qr.encoded_image,
            payload: pix_qr.payload,
            expiration_date: payment.due_date.clone(),
            value: payment.value,
            due_date: payment.due_date,
            status: payment.status,
        };

        tracing::info!(
            "🎉 Processo PIX concluído com sucesso - payment_id: {}, valor: R$ {:.2}",
            pix_payment.payment_id,
            pix_payment.value
        );

        Ok(pix_payment)
    }

    pub async fn create_invoice(
        &self,
        request: CreateInvoiceRequest,
        user_data: UserData,
    ) -> Result<CreateInvoiceResponse, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(
            "📄 Iniciando processo de NF-e para user_id: {}, order_id: {}, valor: R$ {:.2}",
            request.user_id,
            request.order_id,
            request.value
        );

        // Criar customer no Asaas usando os dados do usuário (reutilizar lógica existente)
        let cpf = user_data.cpf.clone();
        let name = user_data.name.clone();
        let email = user_data.email.clone();
        let phone = user_data.phone.clone();
        tracing::info!(
            "👤 Criando/verificando customer no Asaas para user_id: {}",
            request.user_id
        );

        let customer_data = AsaasCustomerRequest {
            name: name.clone(),
            email: email.clone(),
            phone: Some(phone.clone()),
            mobile_phone: Some(phone.clone()),
            cpf_cnpj: cpf.clone(),
            person_type: Some("FISICA".to_string()),
            company: user_data.company_name.clone(),
            city: user_data.city.clone(),
            state: user_data.state.clone(),
            country: Some("Brasil".to_string()),
            postal_code: user_data.postal_code.clone(),
            address: user_data.address.clone(),
            address_number: user_data.address_number.clone(),
            complement: user_data.complement.clone(),
            province: user_data.province.clone(),
            external_reference: Some(request.user_id.clone()),
            disabled: Some(false),
            additional_emails: user_data.additional_emails.clone(),
            municipal_inscription: user_data.municipal_inscription.clone(),
            state_inscription: user_data.state_inscription.clone(),
            observations: user_data.observations.clone(),
            notification_disabled: Some(true),
            foreign_customer: Some(false),
            group_name: None,
            company_name: user_data.company_name.clone(),
        };

        let customer = self.upsert_customer(customer_data).await?;
        let customer_id = customer.id.clone();

        // Force update CPF if provided
        if !cpf.is_empty() {
            tracing::info!(
                " API-ASAAS: 🔄 Force updating customer CPF for invoice: {}",
                user_data.cpf
            );
            let update_data = AsaasCustomerRequest {
                cpf_cnpj: cpf.clone(),
                name: name.clone(),
                email: email.clone(),
                phone: Some(phone.clone()),
                person_type: Some("FISICA".to_string()),
                ..Default::default()
            };
            self.asaas_provider
                .update_customer(&customer_id, update_data)
                .await?;
        }

        tracing::info!(
            " API-ASAAS: ✅ Customer upsert realizado no Asaas: {}",
            customer_id
        );

        // Criar NF-e no Asaas
        tracing::info!(
            "📄 Criando NF-e no Asaas - customer_id: {}, valor: R$ {:.2}",
            customer_id,
            request.value
        );

        let invoice = self
            .asaas_provider
            .create_invoice(
                &customer_id,
                &request.service_description,
                request.value,
                request.observations,
                None, // effective_date será hoje
            )
            .await?;
        tracing::info!(
            "✅ NF-e criada - asaas_invoice_id: {}, status: {}",
            invoice.id,
            invoice.status
        );

        let invoice_response = CreateInvoiceResponse {
            invoice_id: invoice.id.clone(),
            asaas_invoice_id: invoice.id,
            status: invoice.status,
            value: invoice.value,
            effective_date: invoice.effective_date,
            pdf_url: invoice.pdf_url,
            xml_url: invoice.xml_url,
        };

        tracing::info!(
            "🎉 Processo de NF-e concluído com sucesso - invoice_id: {}, valor: R$ {:.2}",
            invoice_response.invoice_id,
            invoice_response.value
        );

        Ok(invoice_response)
    }

    pub async fn getbycpf_cnpj(
        &self,
        cpf_cnpj: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(
            " API-ASAAS: 🔍 Buscando customer no Asaas por CPF/CNPJ: {}",
            cpf_cnpj
        );

        match self
            .asaas_provider
            .get_customer_by_cpf_cnpj(cpf_cnpj)
            .await?
        {
            Some(customer) => {
                tracing::info!(
                    " API-ASAAS: Customer encontrado - customer_id: {}",
                    customer.id
                );
                Ok(Some(customer.id))
            }
            None => {
                tracing::info!(
                    " API-ASAAS: Customer não encontrado para CPF/CNPJ: {}",
                    cpf_cnpj
                );
                Ok(None)
            }
        }
    }

    pub async fn upsert_customer(
        &self,
        customer_data: AsaasCustomerRequest,
    ) -> Result<AsaasCustomerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let external_ref = customer_data
            .external_reference
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone();

        tracing::info!(
            "🔄 Iniciando upsert de customer - name: {}, email: {}, external_ref: {}",
            customer_data.name,
            customer_data.email,
            external_ref
        );

        if external_ref.is_empty() {
            return Err("external_reference é obrigatório para upsert".into());
        }

        // Tentar buscar o customer existente
        match self
            .asaas_provider
            .get_customer_by_external_reference(&external_ref)
            .await?
        {
            Some(existing_customer) => {
                tracing::info!(
                    "Customer encontrado, atualizando - asaas_customer_id: {}",
                    existing_customer.id
                );
                self.asaas_provider
                    .update_customer(&existing_customer.id, customer_data)
                    .await
                    .map_err(Into::into)
            }
            None => {
                tracing::info!(" API-ASAAS: Customer não encontrado, criando novo");
                self.asaas_provider
                    .create_customer(customer_data)
                    .await
                    .map_err(Into::into)
            }
        }
    }

    pub async fn get_my_account(
        &self,
    ) -> Result<AsaasAccountResponse, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(" API-ASAAS: 📋 Obtendo informações da conta Asaas via AsaasService...");
        self.asaas_provider
            .get_my_account()
            .await
            .map_err(Into::into)
    }

    pub async fn get_customer_by_external_reference(
        &self,
        external_ref: &str,
    ) -> Result<Option<AsaasCustomerResponse>, Box<dyn std::error::Error + Send + Sync>> {
        tracing::info!(
            "🔍 Buscando customer no Asaas por external_reference: {}",
            external_ref
        );

        self.asaas_provider
            .get_customer_by_external_reference(external_ref)
            .await
            .map_err(Into::into)
    }
}
