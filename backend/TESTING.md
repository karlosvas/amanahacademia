# Guía de Testing - Backend Amanah Academia

## 📁 Estructura de Tests

```
backend/src/test/
├── mod.rs                     # Declaración de módulos de tests
├── helpers.rs                 # Módulo de helpers (actualmente vacío)
├── helpers/
│   └── fixtures.rs           # Datos de prueba reutilizables
├── services.rs               # Declaración de tests de servicios
├── services/
│   ├── firebase.rs           ✅ 9 tests implementados
│   ├── mailchimp.rs          📝 Pendiente
│   ├── metrics.rs            📝 Pendiente
│   └── payments.rs           📝 Pendiente
├── middleware.rs             📝 Pendiente
├── controllers.rs            📝 Pendiente
└── validations/
    └── validations.rs            ✅ 18 tests implementados
```

## 🎯 Qué testear (Prioridades)

### Alta Prioridad ⚡

1. **Validations** (`src/validations/`)

   - ✅ `validations.rs` - Validaciones de datos y extractor personalizado ValidatedJson
   - Validación de strings no vacíos
   - Validación automática de payloads con Axum

2. **Services** (`src/services/`)

   - ✅ `firebase.rs` - Manejo de respuestas HTTP de Firebase
   - 🔜 `payments.rs` - Procesamiento de pagos con Stripe
   - 🔜 `mailchimp.rs` - Gestión de emails y newsletters
   - 🔜 `metrics.rs` - Métricas y analytics
   - Lógica de negocio pura, transformaciones de datos, cálculos y validaciones

3. **Middleware** (`src/middleware/`)

   - 🔜 `auth.rs` - Autenticación JWT y validación de tokens Firebase
   - Permisos y roles
   - Validación de headers y cookies

4. **Controllers** (`src/controllers/`)
   - 🔜 Lógica específica de controladores
   - Manejo de errores HTTP
   - Validaciones de entrada

### Media Prioridad 🔶

## ❌ Qué NO testear

- **main.rs**: Solo contiene bootstrap de la aplicación
- **Archivos de configuración**: `.env`, `Dockerfile`, `fly.toml`, etc.
- **Routing básico**: Los routers de `routes/` son principalmente declarativos
- **Models simples**: Estructuras que solo definen tipos sin lógica

## 🛠️ Comandos útiles

```bash
# Ejecutar todos los tests
cargo test --lib

# Ejecutar tests mostrando output
cargo test --lib -- --nocapture

# Ejecutar tests de un módulo específico
cargo test test::services::firebase --lib

# Ejecutar un test específico
cargo test test_successful_response_with_valid_json --lib

# Ejecutar tests en modo verbose
cargo test --lib -- --show-output

# Ejecutar tests secuencialmente (útil para debug)
cargo test --lib -- --test-threads=1
```

## 📊 Cobertura actual

### Validations: 18 tests ✅

**Tests de `validate_non_whitespace`**: 11 tests

- `test_validate_non_whitespace_valid_string` - String válido simple
- `test_validate_non_whitespace_valid_with_leading_spaces` - String con espacios al inicio
- `test_validate_non_whitespace_valid_with_trailing_spaces` - String con espacios al final
- `test_validate_non_whitespace_valid_with_both_spaces` - String con espacios en ambos lados
- `test_validate_non_whitespace_empty_string` - String vacío (falla)
- `test_validate_non_whitespace_only_spaces` - Solo espacios (falla)
- `test_validate_non_whitespace_only_tabs` - Solo tabs (falla)
- `test_validate_non_whitespace_mixed_whitespace` - Whitespace mixto (falla)
- `test_validate_non_whitespace_single_character` - Un solo carácter
- `test_validate_non_whitespace_unicode` - Caracteres Unicode
- `test_validate_non_whitespace_special_characters` - Caracteres especiales

**Tests de `ValidatedJson`**: 7 tests

- `test_validated_json_with_valid_payload` - Payload completamente válido
- `test_validated_json_with_invalid_email` - Email inválido (falla)
- `test_validated_json_with_invalid_age` - Edad fuera de rango (falla)
- `test_validated_json_with_empty_name` - Nombre vacío (falla)
- `test_validated_json_with_invalid_json` - JSON malformado (falla)
- `test_validated_json_with_missing_fields` - Campos faltantes (falla)
- `test_validated_json_with_all_edge_cases` - Casos límite válidos

**Total: 18 tests pasando** ✅

### Services/Firebase: 9 tests ✅

- `test_successful_response_with_valid_json` - Deserialización exitosa
- `test_successful_response_with_invalid_json` - Error de parsing
- `test_error_response_with_firebase_error_format` - Formato de error estándar
- `test_error_response_with_error_object_no_message` - Error sin mensaje
- `test_error_response_without_error_field` - Error sin campo "error"
- `test_error_response_with_non_json_body` - Respuesta texto plano
- `test_error_response_with_unknown_status_code` - Códigos HTTP inusuales
- `test_successful_response_with_empty_object` - Objeto vacío
- `test_unauthorized_error` - Error 401

**Total: 9 tests pasando** ✅

---

## 🔥 Testing de `handle_firebase_response`

### Descripción

La función `handle_firebase_response` en [src/services/firebase.rs](src/services/firebase.rs:4-48) maneja las respuestas HTTP de Firebase, deserializándolas en tipos Rust y procesando errores de forma consistente.

### Tecnologías Utilizadas

- **mockito 1.5**: Servidor HTTP mock para simular respuestas de Firebase
- **tokio**: Runtime asíncrono para tests async
- **reqwest**: Cliente HTTP para hacer peticiones
- **serde_json**: Serialización/deserialización JSON

### Casos de Prueba Implementados

#### 1. Respuesta Exitosa con JSON Válido

**Test**: `test_successful_response_with_valid_json`

- **Escenario**: Firebase devuelve 200 OK con JSON bien formado
- **Verifica**: Deserialización correcta del objeto
- **Status**: 200

#### 2. Respuesta Exitosa con JSON Inválido

**Test**: `test_successful_response_with_invalid_json`

- **Escenario**: 200 OK pero JSON no coincide con el tipo esperado
- **Verifica**: Error "Error parsing Firebase response"
- **Status**: 500 (INTERNAL_SERVER_ERROR)

#### 3. Error con Formato Firebase Estándar

**Test**: `test_error_response_with_firebase_error_format`

- **Escenario**: `{"error": {"message": "INVALID_EMAIL", "code": 400}}`
- **Verifica**: Extracción del mensaje específico de error
- **Status**: 400 (BAD_REQUEST)

#### 4. Error sin Campo Message

**Test**: `test_error_response_with_error_object_no_message`

- **Escenario**: `{"error": {"code": 403, "details": "Forbidden"}}`
- **Verifica**: Devuelve el objeto error completo
- **Status**: 403 (FORBIDDEN)

#### 5. Error sin Campo Error

**Test**: `test_error_response_without_error_field`

- **Escenario**: JSON sin campo "error" estándar
- **Verifica**: Devuelve todo el JSON como string
- **Status**: 500

#### 6. Error con Respuesta No JSON

**Test**: `test_error_response_with_non_json_body`

- **Escenario**: Texto plano: "Not Found"
- **Verifica**: Devuelve el texto raw
- **Status**: 404 (NOT_FOUND)

#### 7. Código de Estado Inusual

**Test**: `test_error_response_with_unknown_status_code`

- **Escenario**: Status 418 (I'M_A_TEAPOT)
- **Verifica**: Manejo correcto de códigos poco comunes
- **Status**: 418

#### 8. Respuesta con Objeto Vacío

**Test**: `test_successful_response_with_empty_object`

- **Escenario**: `{}`
- **Verifica**: Deserialización exitosa de objeto vacío
- **Status**: 200

#### 9. Error de Autenticación

**Test**: `test_unauthorized_error`

- **Escenario**: `{"error": {"message": "UNAUTHORIZED", "code": 401}}`
- **Verifica**: Manejo correcto de errores de autenticación
- **Status**: 401 (UNAUTHORIZED)

### Estructura del Test (Patrón AAA)

Cada test sigue **Arrange-Act-Assert**:

```rust
#[tokio::test]
async fn test_successful_response_with_valid_json() {
    // Arrange: Configurar el servidor mock
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/user")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id": "123", "email": "test@example.com"}"#)
        .create();

    // Act: Ejecutar la función
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/user", server.url()))
        .send()
        .await
        .unwrap();

    let result = handle_firebase_response(response).await;

    // Assert: Verificar resultados
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.id, "123");

    mock.assert(); // Verifica que se llamó al endpoint
}
```

### Ejecutar Tests de Firebase

```bash
# Todos los tests de Firebase
cargo test test::services::firebase --lib

# Con output detallado
cargo test test::services::firebase --lib -- --nocapture

# Un test específico
cargo test test_successful_response_with_valid_json --lib
```

### Cobertura de Escenarios

- ✅ **Respuestas Exitosas** (200-299)
- ✅ **Errores de Cliente** (400-499)
- ✅ **Errores de Servidor** (500-599)
- ✅ **Formatos de Error Firebase**
- ✅ **Errores de Parsing**
- ✅ **Respuestas no JSON**
- ✅ **Objetos vacíos**

### Mejores Prácticas Aplicadas

1. **Mock en lugar de peticiones reales**: Usa `mockito` para evitar dependencias externas
2. **Tests atómicos**: Cada test verifica un único escenario
3. **Nombres descriptivos**: Los nombres indican claramente qué se prueba
4. **Assertions explícitas**: Verifica tanto status code como mensajes
5. **Cleanup automático**: `mockito` limpia servidores después de cada test

### Ejemplo: Añadir un Test Personalizado

```rust
#[tokio::test]
async fn test_rate_limit_error() {
    #[derive(Debug, Deserialize)]
    struct RateLimitError {
        retry_after: u32,
    }

    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("POST", "/api")
        .with_status(429)
        .with_body(r#"{"error": {"message": "RATE_LIMIT", "retry_after": 60}}"#)
        .create();

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api", server.url()))
        .send()
        .await
        .unwrap();

    let result: Result<RateLimitError, _> =
        handle_firebase_response(response).await;

    assert!(result.is_err());
    let (status, _msg) = result.unwrap_err();
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);

    mock.assert();
}
```

---

## 🔥 Testing de Validations

### Descripción del Módulo

El módulo de validations en [src/validations/validations.rs](src/validations/validations.rs) proporciona:

1. **`validate_non_whitespace`**: Función de validación que asegura que los strings no estén vacíos ni contengan solo espacios en blanco
2. **`ValidatedJson<T>`**: Extractor personalizado para Axum que automáticamente valida payloads JSON usando el trait `Validate` de la crate `validator`

### Tecnologías Utilizadas en Validations

- **validator 0.16**: Validación declarativa de structs
- **axum**: Framework web con extractores personalizados
- **tower**: Testing de servicios HTTP
- **tokio**: Runtime asíncrono para tests async

### Tests de `validate_non_whitespace` (11 tests)

Esta función valida que un string no esté vacío ni contenga solo whitespace.

#### Casos Válidos

- **`test_validate_non_whitespace_valid_string`**: String simple "hello"
- **`test_validate_non_whitespace_valid_with_leading_spaces`**: " hello" con espacios al inicio
- **`test_validate_non_whitespace_valid_with_trailing_spaces`**: "hello " con espacios al final
- **`test_validate_non_whitespace_valid_with_both_spaces`**: " hello world " con espacios en ambos lados
- **`test_validate_non_whitespace_single_character`**: String de un solo carácter "a"
- **`test_validate_non_whitespace_unicode`**: Caracteres Unicode "مرحبا"
- **`test_validate_non_whitespace_special_characters`**: Caracteres especiales "!@#$%"

#### Casos Inválidos (retornan `ValidationError` con código "cannot_be_empty")

- **`test_validate_non_whitespace_empty_string`**: String completamente vacío ""
- **`test_validate_non_whitespace_only_spaces`**: Solo espacios " "
- **`test_validate_non_whitespace_only_tabs`**: Solo tabs "\t\t\t"
- **`test_validate_non_whitespace_mixed_whitespace`**: Whitespace mixto " \t \n \r "

### Tests de `ValidatedJson<T>` (7 tests)

Este extractor personalizado valida automáticamente payloads JSON usando el trait `Validate`.

#### Estructura de Prueba

```rust
#[derive(Debug, Deserialize, Serialize, Validate)]
struct TestPayload {
    #[validate(length(min = 1, max = 100))]
    name: String,
    #[validate(range(min = 0, max = 150))]
    age: i32,
    #[validate(email)]
    email: String,
}
```

#### Casos de Prueba

##### Caso Válido

- **`test_validated_json_with_valid_payload`**: Payload completamente válido retorna 200 OK
  - name: "John Doe", age: 30, email: "<john@example.com>"

##### Casos Inválidos (retornan 400 BAD_REQUEST)

- **`test_validated_json_with_invalid_email`**: Email sin formato válido
  - email: "invalid-email" (sin @)
- **`test_validated_json_with_invalid_age`**: Edad fuera del rango permitido
  - age: 200 (máximo es 150)
- **`test_validated_json_with_empty_name`**: Nombre vacío
  - name: "" (mínimo es 1 carácter)
- **`test_validated_json_with_invalid_json`**: JSON malformado
  - Body: `{ invalid json }`
- **`test_validated_json_with_missing_fields`**: Campos requeridos faltantes
  - Solo incluye name, falta age y email

##### Casos Límite Válidos

- **`test_validated_json_with_all_edge_cases`**: Valores en los límites de validación
  - Edad mínima: 0
  - Edad máxima: 150
  - Nombre mínimo: "A" (1 carácter)

### Estructura de los Tests de Validations (Patrón AAA)

```rust
#[tokio::test]
async fn test_validated_json_with_valid_payload() {
    // Arrange: Configurar el router con el handler
    let app = Router::new().route("/test", post(test_handler));

    let body = r#"{
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    }"#;

    // Act: Ejecutar la petición
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/test")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    // Assert: Verificar resultado
    assert_eq!(response.status(), StatusCode::OK);
}
```

### Ejecutar Tests de Validations

```bash
# Todos los tests de validations
cargo test test::validations --lib

# Con output detallado
cargo test test::validations --lib -- --nocapture

# Solo tests de validate_non_whitespace
cargo test validate_non_whitespace --lib

# Solo tests de ValidatedJson
cargo test validated_json --lib
```

### Cobertura de Escenarios de Validations

- ✅ **Strings vacíos y whitespace**
- ✅ **Unicode y caracteres especiales**
- ✅ **Validación de email**
- ✅ **Validación de rangos numéricos**
- ✅ **Validación de longitud de strings**
- ✅ **JSON malformado**
- ✅ **Campos faltantes**
- ✅ **Casos límite (edge cases)**

### Uso en Producción

El extractor `ValidatedJson` se usa en los handlers de Axum de esta forma:

```rust
use crate::validations::validations::ValidatedJson;

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserPayload>
) -> impl IntoResponse {
    // El payload ya está validado automáticamente
    // Si hay errores de validación, se retorna 400 BAD_REQUEST
    // antes de llegar a esta línea
    Json(ResponseAPI::success("User created".to_string(), payload))
}
```

---

## 🚀 Próximos pasos

1. ✅ ~~Implementar tests de `validations/validations.rs`~~ - **Completado**
2. ✅ ~~Implementar tests de `services/firebase.rs`~~ - **Completado**
3. 🔜 Agregar tests para `services/payments.rs`
4. 🔜 Agregar tests para `services/mailchimp.rs`
5. 🔜 Agregar tests para `services/metrics.rs`
6. 🔜 Implementar tests de `middleware/auth.rs`
7. 🔜 Crear helpers para mocks de AppState
8. 🔜 Implementar tests de controladores

## 🎨 Mejores prácticas

1. **Organiza por módulo**: Cada archivo de test corresponde a un archivo fuente
2. **Usa mocks**: Evita dependencias externas con `mockito` para HTTP y mocks personalizados para servicios
3. **Tests pequeños y enfocados**: Un test = una funcionalidad
4. **Nombres descriptivos**: `test_error_response_with_firebase_error_format` es mejor que `test_error_1`
5. **Patrón AAA**: Arrange-Act-Assert para claridad
6. **Async tests**: Usa `#[tokio::test]` para funciones async
7. **Assertions específicas**: Verifica valores exactos, no solo `is_ok()` o `is_err()`

## 🔗 Recursos útiles

- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Testing in Axum](https://github.com/tokio-rs/axum/tree/main/examples/testing)
- [mockito Documentation](https://docs.rs/mockito/latest/mockito/)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
