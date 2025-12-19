//! Biblioteca Rust para integração com a API do Asaas
//!
//! Esta biblioteca fornece uma interface de alto nível para interagir com os serviços do Asaas,
//! incluindo criação de pagamentos PIX e notas fiscais (NF-e).

pub mod asaas_provider;
pub mod models;
pub mod service;

// Re-exportar tipos principais para facilitar o uso
pub use service::AsaasService;
pub use models::{
    CreateInvoiceRequest,
    CreateInvoiceResponse,
    CreatePixPaymentRequest,
    CreatePixPaymentResponse,
    UserData,
};
