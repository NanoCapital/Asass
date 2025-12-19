# Serviço Asaas API

Este é um microsserviço responsável por gerenciar pagamentos via plataforma Asaas. Ele fornece uma API REST para criação de pagamentos PIX e gerenciamento de customers.

## Funcionalidades

- ✅ Criação de customers no Asaas
- ✅ Criação de pagamentos PIX
- ✅ Geração de QR Codes PIX
- ✅ Emissão de NF-e (Nota Fiscal Eletrônica)
- ✅ Simulação de dados de usuários (para desenvolvimento)

## Endpoints da API

### Base URL
```
http://localhost:3000
```

### 1. Health Check

**GET** `/`

Verifica se o serviço está funcionando corretamente.

**Resposta de Sucesso (200):**
```json
{
  "status": "ok",
  "service": "Asaas API Service",
  "version": "1.0.0"
}
```

### 2. Criar Pagamento PIX

**POST** `/api/v1/payments/pix`

Cria um novo pagamento PIX no Asaas, incluindo a criação automática do customer se necessário.

**Corpo da Requisição:**
```json
{
  "user_id": "string",
  "order_id": "string",
  "provider": "asaas",
  "value": 100.50,
  "description": "Pagamento do pedido #123"
}
```

**Parâmetros:**
- `user_id` (string, obrigatório): ID único do usuário
- `order_id` (string, obrigatório): ID único do pedido
- `provider` (string, obrigatório): Provedor de pagamento (sempre "asaas")
- `value` (number, obrigatório): Valor do pagamento em reais
- `description` (string, opcional): Descrição do pagamento

**Resposta de Sucesso (200):**
```json
{
  "payment_id": "pay_123456789",
  "asaas_payment_id": "pay_123456789",
  "qr_code_base64": "iVBORw0KGgoAAAANSUhEUgAA...",
  "payload": "00020101021226860014br.gov.bcb.pix...",
  "expiration_date": "2024-12-31T23:59:59Z",
  "value": 100.50,
  "due_date": "2024-12-08T12:00:00Z",
  "status": "PENDING"
}
```

**Respostas de Erro:**
- `400 Bad Request`: Parâmetros inválidos
- `404 Not Found`: Usuário não encontrado
- `500 Internal Server Error`: Erro interno do servidor

### 3. Emitir NF-e

**POST** `/api/v1/invoices`

Emite uma Nota Fiscal Eletrônica (NF-e) através da plataforma Asaas, criando automaticamente o customer se necessário.

**Corpo da Requisição:**
```json
{
  "user_id": "string",
  "order_id": "string",
  "provider": "asaas",
  "service_description": "Descrição do serviço prestado",
  "value": 150.00,
  "observations": "Observações adicionais da NF-e"
}
```

**Parâmetros:**
- `user_id` (string, obrigatório): ID único do usuário
- `order_id` (string, obrigatório): ID único do pedido
- `provider` (string, obrigatório): Provedor de emissão (sempre "asaas")
- `service_description` (string, obrigatório): Descrição do serviço/produto
- `value` (number, obrigatório): Valor da NF-e em reais
- `observations` (string, opcional): Observações adicionais

**Resposta de Sucesso (200):**
```json
{
  "invoice_id": "inv_123456789",
  "asaas_invoice_id": "inv_123456789",
  "status": "AUTHORIZED",
  "value": 150.00,
  "effective_date": "2024-12-08",
  "pdf_url": "https://www.asaas.com/invoice/pdf/inv_123456789",
  "xml_url": "https://www.asaas.com/invoice/xml/inv_123456789"
}
```

**Respostas de Erro:**
- `400 Bad Request`: Parâmetros inválidos
- `404 Not Found`: Usuário não encontrado
- `500 Internal Server Error`: Erro interno do servidor

### 4. Buscar Dados do Usuário

**GET** `/api/v1/users/{user_id}`

Busca os dados de um usuário específico (usado internamente para simulação).

**Parâmetros de URL:**
- `user_id` (string): ID do usuário

**Resposta de Sucesso (200):**
```json
{
  "email": "user123@example.com",
  "name": "João Silva",
  "phone": "11999999999",
  "cpf": "12345678901"
}
```

**Resposta de Erro (404):**
```json
{
  "error": "User not found",
  "message": "User with id user999 not found"
}
```

## Configuração e Execução

### Variáveis de Ambiente

- `ASAAS_API_KEY`: Chave de API do Asaas (obrigatória)
- `RUST_LOG`: Nível de log (opcional, padrão: info)

### Executando o Serviço

1. **Instalar dependências:**
```bash
cargo build
```

2. **Executar:**
```bash
cargo run
```

O serviço será iniciado na porta 3000.

### Testando a API

**Exemplo de criação de pagamento PIX:**
```bash
curl -X POST http://localhost:3000/api/v1/payments/pix \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "order_id": "order456",
    "provider": "asaas",
    "value": 50.00,
    "description": "Pagamento de teste"
  }'
```

**Exemplo de health check:**
```bash
curl http://localhost:3000/
```

**Exemplo de emissão de NF-e:**
```bash
curl -X POST http://localhost:3000/api/v1/invoices \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "order_id": "order456",
    "provider": "asaas",
    "service_description": "Serviços de desenvolvimento de software",
    "value": 1500.00,
    "observations": "NF-e emitida conforme pedido #456"
  }'
```

## Arquitetura

O serviço é construído usando:

- **Axum**: Framework web para Rust
- **Tokio**: Runtime assíncrono
- **Reqwest**: Cliente HTTP para comunicação com a API do Asaas
- **Serde**: Serialização/Deserialização JSON
- **Tracing**: Sistema de logs

### Fluxo de Criação de Pagamento PIX

1. Recebe requisição POST `/api/v1/payments/pix`
2. Busca dados do usuário (simulado em desenvolvimento)
3. Cria customer no Asaas se necessário
4. Cria cobrança PIX no Asaas
5. Obtém QR Code PIX
6. Retorna resposta com todos os dados do pagamento

### Fluxo de Emissão de NF-e

1. Recebe requisição POST `/api/v1/invoices`
2. Busca dados do usuário (simulado em desenvolvimento)
3. Cria customer no Asaas se necessário
4. Emite NF-e no Asaas
5. Retorna resposta com dados da NF-e (ID, status, URLs para PDF/XML)

## Desenvolvimento

### Estrutura do Projeto

```
src/
├── main.rs          # Ponto de entrada e definição de rotas
├── models.rs        # Modelos de dados da API
├── asaas_provider.rs # Cliente para API do Asaas
├── service.rs       # Lógica de negócio
└── lib.rs          # (se aplicável)
```

### Adicionando Novos Endpoints

1. Defina os modelos de request/response em `models.rs`
2. Implemente a lógica no `AsaasService`
3. Adicione a rota no `main.rs`

## Tratamento de Erros

O serviço implementa tratamento de erros abrangente:

- **400**: Parâmetros inválidos
- **404**: Recurso não encontrado
- **500**: Erro interno do servidor

Todos os erros incluem mensagens descritivas em português.

## Segurança

- A chave da API do Asaas é armazenada em variável de ambiente
- Validação de entrada em todos os endpoints
- CORS habilitado para desenvolvimento

## Próximas Melhorias

- [ ] Autenticação JWT
- [ ] Rate limiting
- [ ] Métricas e monitoramento
- [ ] Integração com banco de dados real para usuários
- [ ] Suporte a webhooks do Asaas
- [ ] Cache de customers criados