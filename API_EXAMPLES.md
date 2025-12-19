# Exemplos de Uso da API Asaas

Este documento contém exemplos práticos de como usar a API do serviço Asaas em diferentes linguagens de programação.

## Endpoints Disponíveis

### 1. Criar Pagamento PIX
**POST** `/api/v1/payments/pix`

### 2. Health Check
**GET** `/`

### 3. Buscar Usuário (Desenvolvimento)
**GET** `/api/v1/users/{user_id}`

## Exemplos por Linguagem

### JavaScript/Node.js

```javascript
const axios = require('axios');

const ASAAS_BASE_URL = 'http://localhost:3000';

// Criar pagamento PIX
async function criarPagamentoPIX(userId, orderId, valor, descricao) {
  try {
    const response = await axios.post(`${ASAAS_BASE_URL}/api/v1/payments/pix`, {
      user_id: userId,
      order_id: orderId,
      provider: 'asaas',
      value: valor,
      description: descricao
    });

    console.log('Pagamento criado:', response.data);
    return response.data;
  } catch (error) {
    console.error('Erro ao criar pagamento:', error.response?.data || error.message);
    throw error;
  }
}

// Health check
async function verificarSaude() {
  try {
    const response = await axios.get(`${ASAAS_BASE_URL}/`);
    console.log('Status do serviço:', response.data);
    return response.data;
  } catch (error) {
    console.error('Erro no health check:', error.message);
    throw error;
  }
}

// Exemplo de uso
async function exemploUso() {
  try {
    // Verificar se o serviço está funcionando
    await verificarSaude();

    // Criar um pagamento PIX
    const pagamento = await criarPagamentoPIX(
      'user123',
      'order456',
      99.90,
      'Compra de produto X'
    );

    console.log('QR Code (base64):', pagamento.qr_code_base64);
    console.log('Payload PIX:', pagamento.payload);
    console.log('Status:', pagamento.status);

  } catch (error) {
    console.error('Erro no exemplo:', error);
  }
}

exemploUso();
```

### Python

```python
import requests
import json

ASAAS_BASE_URL = 'http://localhost:3000'

def criar_pagamento_pix(user_id, order_id, valor, descricao=None):
    """Cria um pagamento PIX via API Asaas"""
    url = f"{ASAAS_BASE_URL}/api/v1/payments/pix"

    payload = {
        "user_id": user_id,
        "order_id": order_id,
        "provider": "asaas",
        "value": valor,
        "description": descricao
    }

    headers = {
        'Content-Type': 'application/json'
    }

    try:
        response = requests.post(url, json=payload, headers=headers)
        response.raise_for_status()

        pagamento = response.json()
        print("Pagamento criado com sucesso!")
        print(f"Payment ID: {pagamento['payment_id']}")
        print(f"Status: {pagamento['status']}")
        print(f"Valor: R$ {pagamento['value']}")

        return pagamento

    except requests.exceptions.RequestException as e:
        print(f"Erro ao criar pagamento: {e}")
        if hasattr(e, 'response') and e.response:
            print(f"Detalhes do erro: {e.response.json()}")
        raise

def health_check():
    """Verifica se o serviço Asaas está funcionando"""
    try:
        response = requests.get(f"{ASAAS_BASE_URL}/")
        response.raise_for_status()

        status = response.json()
        print(f"Serviço: {status['service']}")
        print(f"Status: {status['status']}")
        print(f"Versão: {status['version']}")

        return status

    except requests.exceptions.RequestException as e:
        print(f"Erro no health check: {e}")
        raise

# Exemplo de uso
if __name__ == "__main__":
    try:
        # Verificar saúde do serviço
        print("=== Health Check ===")
        health_check()

        print("\n=== Criando Pagamento PIX ===")
        # Criar pagamento
        pagamento = criar_pagamento_pix(
            user_id="user123",
            order_id="order789",
            valor=149.99,
            descricao="Compra online na loja X"
        )

        # Salvar QR Code em arquivo (exemplo)
        if 'qr_code_base64' in pagamento:
            import base64
            qr_data = pagamento['qr_code_base64']
            with open('qr_code.png', 'wb') as f:
                f.write(base64.b64decode(qr_data))
            print("QR Code salvo como 'qr_code.png'")

    except Exception as e:
        print(f"Erro no exemplo: {e}")
```

### cURL

```bash
# Health Check
curl -X GET http://localhost:3000/

# Criar Pagamento PIX
curl -X POST http://localhost:3000/api/v1/payments/pix \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "user123",
    "order_id": "order456",
    "provider": "asaas",
    "value": 75.50,
    "description": "Pagamento de serviço"
  }'

# Buscar dados de usuário (desenvolvimento)
curl -X GET http://localhost:3000/api/v1/users/user123
```

### PHP

```php
<?php

const ASAAS_BASE_URL = 'http://localhost:3000';

function criarPagamentoPIX($userId, $orderId, $valor, $descricao = null) {
    $url = ASAAS_BASE_URL . '/api/v1/payments/pix';

    $payload = [
        'user_id' => $userId,
        'order_id' => $orderId,
        'provider' => 'asaas',
        'value' => $valor,
        'description' => $descricao
    ];

    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, json_encode($payload));
    curl_setopt($ch, CURLOPT_HTTPHEADER, [
        'Content-Type: application/json'
    ]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

    $response = curl_exec($ch);
    $httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);

    if (curl_errno($ch)) {
        throw new Exception('Erro cURL: ' . curl_error($ch));
    }

    curl_close($ch);

    if ($httpCode !== 200) {
        $error = json_decode($response, true);
        throw new Exception('Erro da API: ' . ($error['message'] ?? $response));
    }

    $pagamento = json_decode($response, true);

    echo "Pagamento criado com sucesso!\n";
    echo "Payment ID: {$pagamento['payment_id']}\n";
    echo "Status: {$pagamento['status']}\n";
    echo "Valor: R$ {$pagamento['value']}\n";

    return $pagamento;
}

function healthCheck() {
    $url = ASAAS_BASE_URL . '/';

    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);

    $response = curl_exec($ch);
    $httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);

    if (curl_errno($ch)) {
        throw new Exception('Erro cURL: ' . curl_error($ch));
    }

    curl_close($ch);

    if ($httpCode !== 200) {
        throw new Exception('Erro no health check: ' . $response);
    }

    $status = json_decode($response, true);

    echo "Serviço: {$status['service']}\n";
    echo "Status: {$status['status']}\n";
    echo "Versão: {$status['version']}\n";

    return $status;
}

// Exemplo de uso
try {
    echo "=== Health Check ===\n";
    healthCheck();

    echo "\n=== Criando Pagamento PIX ===\n";
    $pagamento = criarPagamentoPIX(
        'user123',
        'order789',
        199.99,
        'Compra de produto premium'
    );

    // Salvar payload PIX para uso posterior
    if (isset($pagamento['payload'])) {
        file_put_contents('pix_payload.txt', $pagamento['payload']);
        echo "Payload PIX salvo em 'pix_payload.txt'\n";
    }

} catch (Exception $e) {
    echo "Erro: " . $e->getMessage() . "\n";
}

?>
```

### Java

```java
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.HashMap;
import java.util.Map;

public class AsaasApiClient {

    private static final String ASAAS_BASE_URL = "http://localhost:3000";
    private final HttpClient httpClient;
    private final ObjectMapper objectMapper;

    public AsaasApiClient() {
        this.httpClient = HttpClient.newHttpClient();
        this.objectMapper = new ObjectMapper();
    }

    public Map<String, Object> criarPagamentoPIX(String userId, String orderId, double valor, String descricao) throws Exception {
        String url = ASAAS_BASE_URL + "/api/v1/payments/pix";

        Map<String, Object> payload = new HashMap<>();
        payload.put("user_id", userId);
        payload.put("order_id", orderId);
        payload.put("provider", "asaas");
        payload.put("value", valor);
        payload.put("description", descricao);

        String jsonPayload = objectMapper.writeValueAsString(payload);

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create(url))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(jsonPayload))
            .build();

        HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

        if (response.statusCode() != 200) {
            throw new RuntimeException("Erro da API: " + response.body());
        }

        @SuppressWarnings("unchecked")
        Map<String, Object> pagamento = objectMapper.readValue(response.body(), Map.class);

        System.out.println("Pagamento criado com sucesso!");
        System.out.println("Payment ID: " + pagamento.get("payment_id"));
        System.out.println("Status: " + pagamento.get("status"));
        System.out.println("Valor: R$ " + pagamento.get("value"));

        return pagamento;
    }

    public Map<String, Object> healthCheck() throws Exception {
        String url = ASAAS_BASE_URL + "/";

        HttpRequest request = HttpRequest.newBuilder()
            .uri(URI.create(url))
            .GET()
            .build();

        HttpResponse<String> response = httpClient.send(request, HttpResponse.BodyHandlers.ofString());

        if (response.statusCode() != 200) {
            throw new RuntimeException("Erro no health check: " + response.body());
        }

        @SuppressWarnings("unchecked")
        Map<String, Object> status = objectMapper.readValue(response.body(), Map.class);

        System.out.println("Serviço: " + status.get("service"));
        System.out.println("Status: " + status.get("status"));
        System.out.println("Versão: " + status.get("version"));

        return status;
    }

    // Exemplo de uso
    public static void main(String[] args) {
        AsaasApiClient client = new AsaasApiClient();

        try {
            System.out.println("=== Health Check ===");
            client.healthCheck();

            System.out.println("\n=== Criando Pagamento PIX ===");
            Map<String, Object> pagamento = client.criarPagamentoPIX(
                "user123",
                "order456",
                299.99,
                "Compra de produto Java"
            );

            // Aqui você pode processar o QR code, payload, etc.

        } catch (Exception e) {
            System.err.println("Erro: " + e.getMessage());
        }
    }
}
```

## Tratamento de Erros

Todos os exemplos incluem tratamento básico de erros. Em produção, considere implementar:

- Retry logic para falhas temporárias
- Logging detalhado
- Métricas de monitoramento
- Validação de entrada mais rigorosa

## Formatos de Resposta

### Sucesso (200)
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

### Erro (400/404/500)
```json
{
  "error": "Invalid request parameters",
  "message": "user_id, order_id must not be empty and value must be positive"
}
```

## Notas Importantes

- O `qr_code_base64` pode ser convertido para imagem PNG/JPG
- O `payload` é o código copia e cola do PIX
- O status inicial é sempre "PENDING" até o pagamento ser confirmado
- Os pagamentos expiram conforme definido no Asaas (geralmente 24-48 horas)