---
applyTo: "**/*.rs"
excludeAgent: "cloud-agent"
---

# Code Review Guidelines


## Your Expertise

- **Async Rust**: Tokio runtime, async/await patterns, futures
- **Type Safety**: Serde, JsonSchema, type-safe parameter validation
- **Transports**: Stdio, HTTP
- **Error Handling**: ErrorData, `thiserror`, proper error propagation
- **Testing**: Unit tests, integration tests, tokio-test
- **Performance**: Arc, RwLock, efficient state management
- **Deployment**: Cross-compilation, binary distribution

## Common Tasks

### Tool Implementation

Help developers implement tools using macros:

```rust
use rmcp::tool;
use rmcp::model::Parameters;
use serde::{Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CalculateParams {
    pub a: f64,
    pub b: f64,
    pub operation: String,
}

#[tool(
    name = "calculate",
    description = "Performs arithmetic operations",
    annotations(read_only_hint = true, idempotent_hint = true)
)]
pub async fn calculate(params: Parameters<CalculateParams>) -> Result<f64, String> {
    let p = params.inner();
    match p.operation.as_str() {
        "add" => Ok(p.a + p.b),
        "subtract" => Ok(p.a - p.b),
        "multiply" => Ok(p.a * p.b),
        "divide" if p.b != 0.0 => Ok(p.a / p.b),
        "divide" => Err("Division by zero".to_string()),
        _ => Err(format!("Unknown operation: {}", p.operation)),
    }
}
```

### Server Handler with Macros

Guide developers in using tool router macros:

```rust
use rmcp::{tool_router, tool_handler};
use rmcp::server::{ServerHandler, ToolRouter};

pub struct MyHandler {
    state: ServerState,
    tool_router: ToolRouter,
}

#[tool_router]
impl MyHandler {
    #[tool(name = "greet", description = "Greets a user")]
    async fn greet(params: Parameters<GreetParams>) -> String {
        format!("Hello, {}!", params.inner().name)
    }

    #[tool(name = "increment", annotations(destructive_hint = true))]
    async fn increment(state: &ServerState) -> i32 {
        state.increment().await
    }

    pub fn new() -> Self {
        Self {
            state: ServerState::new(),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for MyHandler {
    // Prompt and resource handlers...
}
```

### Transport Configuration

Assist with different transport setups:

**Stdio (for CLI integration):**

```rust
use rmcp::transport::StdioTransport;

let transport = StdioTransport::new();
let server = Server::builder()
    .with_handler(handler)
    .build(transport)?;
server.run(signal::ctrl_c()).await?;
```

**SSE (Server-Sent Events):**

```rust
use rmcp::transport::SseServerTransport;
use std::net::SocketAddr;

let addr: SocketAddr = "127.0.0.1:8000".parse()?;
let transport = SseServerTransport::new(addr);
let server = Server::builder()
    .with_handler(handler)
    .build(transport)?;
server.run(signal::ctrl_c()).await?;
```

**HTTP with Axum:**

```rust
use rmcp::transport::StreamableHttpTransport;
use axum::{Router, routing::post};

let transport = StreamableHttpTransport::new();
let app = Router::new()
    .route("/mcp", post(transport.handler()));

let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
axum::serve(listener, app).await?;
```

### Prompt Implementation

Guide prompt handler implementation:

```rust
async fn list_prompts(
    &self,
    _request: Option<PaginatedRequestParam>,
    _context: RequestContext<RoleServer>,
) -> Result<ListPromptsResult, ErrorData> {
    let prompts = vec![
        Prompt {
            name: "code-review".to_string(),
            description: Some("Review code for best practices".to_string()),
            arguments: Some(vec![
                PromptArgument {
                    name: "language".to_string(),
                    description: Some("Programming language".to_string()),
                    required: Some(true),
                },
                PromptArgument {
                    name: "code".to_string(),
                    description: Some("Code to review".to_string()),
                    required: Some(true),
                },
            ]),
        },
    ];
    Ok(ListPromptsResult { prompts })
}

async fn get_prompt(
    &self,
    request: GetPromptRequestParam,
    _context: RequestContext<RoleServer>,
) -> Result<GetPromptResult, ErrorData> {
    match request.name.as_str() {
        "code-review" => {
            let args = request.arguments.as_ref()
                .ok_or_else(|| ErrorData::invalid_params("arguments required"))?;

            let language = args.get("language")
                .ok_or_else(|| ErrorData::invalid_params("language required"))?;
            let code = args.get("code")
                .ok_or_else(|| ErrorData::invalid_params("code required"))?;

            Ok(GetPromptResult {
                description: Some(format!("Code review for {}", language)),
                messages: vec![
                    PromptMessage::user(format!(
                        "Review this {} code for best practices:\n\n{}",
                        language, code
                    )),
                ],
            })
        }
        _ => Err(ErrorData::invalid_params("Unknown prompt")),
    }
}
```

### Resource Implementation

Help with resource handlers:

```rust
async fn list_resources(
    &self,
    _request: Option<PaginatedRequestParam>,
    _context: RequestContext<RoleServer>,
) -> Result<ListResourcesResult, ErrorData> {
    let resources = vec![
        Resource {
            uri: "file:///config/settings.json".to_string(),
            name: "Server Settings".to_string(),
            description: Some("Server configuration".to_string()),
            mime_type: Some("application/json".to_string()),
        },
    ];
    Ok(ListResourcesResult { resources })
}

async fn read_resource(
    &self,
    request: ReadResourceRequestParam,
    _context: RequestContext<RoleServer>,
) -> Result<ReadResourceResult, ErrorData> {
    match request.uri.as_str() {
        "file:///config/settings.json" => {
            let settings = self.load_settings().await
                .map_err(|e| ErrorData::internal_error(e.to_string()))?;

            let json = serde_json::to_string_pretty(&settings)
                .map_err(|e| ErrorData::internal_error(e.to_string()))?;

            Ok(ReadResourceResult {
                contents: vec![
                    ResourceContents::text(json)
                        .with_uri(request.uri)
                        .with_mime_type("application/json"),
                ],
            })
        }
        _ => Err(ErrorData::invalid_params("Unknown resource")),
    }
}
```

### State Management

Advise on shared state patterns:

```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ServerState {
    counter: Arc<RwLock<i32>>,
    cache: Arc<RwLock<HashMap<String, String>>>,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(RwLock::new(0)),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn increment(&self) -> i32 {
        let mut counter = self.counter.write().await;
        *counter += 1;
        *counter
    }

    pub async fn set_cache(&self, key: String, value: String) {
        let mut cache = self.cache.write().await;
        cache.insert(key, value);
    }

    pub async fn get_cache(&self, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
}
```

### Error Handling

Guide proper error handling:

```rust
// Application-level errors with thiserror
async fn load_data() -> Result<Data, Error> {
    let content = tokio::fs::read_to_string("data.json")
        .await?;

    let data: Data = serde_json::from_str(&content)?;

    Ok(data)
}

#[derive(thiserror::Error, Debug)]
enum LoadDataError {
  #[error("failed to open JSON file")]
  Open(#[from] io::Error),
  #[error("failed to parse JSON")]
  Parse(#[from] serde_json::Error)
}
```

### Testing

Provide testing guidance:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::model::Parameters;

    #[tokio::test]
    async fn test_calculate_add() {
        let params = Parameters::new(CalculateParams {
            a: 5.0,
            b: 3.0,
            operation: "add".to_string(),
        });

        let result = calculate(params).await.unwrap();
        assert_eq!(result, 8.0);
    }

    #[tokio::test]
    async fn test_server_handler() {
        let handler = MyHandler::new();
        let context = RequestContext::default();

        let result = handler.list_tools(None, context).await.unwrap();
        assert!(!result.tools.is_empty());
    }
}
```

### Performance Optimization

Advise on performance:

1. **Use appropriate lock types:**

   - `RwLock` for read-heavy workloads
   - `Mutex` for write-heavy workloads
   - Consider `DashMap` for concurrent hash maps

2. **Minimize lock duration:**

   ```rust
   // Good: Clone data out of lock
   let value = {
       let data = self.data.read().await;
       data.clone()
   };
   process(value).await;

   // Bad: Hold lock during async operation
   let data = self.data.read().await;
   process(&*data).await; // Lock held too long
   ```

3. **Use buffered channels:**

   ```rust
   use tokio::sync::mpsc;
   let (tx, rx) = mpsc::channel(100); // Buffered
   ```

4. **Batch operations:**
   ```rust
   async fn batch_process(&self, items: Vec<Item>) -> Vec<Result<(), Error>> {
       use futures::future::join_all;
       join_all(items.into_iter().map(|item| self.process(item))).await
   }
   ```

## Deployment Guidance

## Communication Style

- Explain Rust-specific patterns (ownership, lifetimes, async)
- Include error handling in all examples
- Suggest performance optimizations when relevant
- Help debug compilation errors and async issues
- Recommend testing strategies
- Guide on proper macro usage

## Key Principles

1. **Type Safety First**: Use JsonSchema for all parameters
2. **Async All The Way**: All handlers must be async
3. **Proper Error Handling**: Use Result types and ErrorData
4. **Test Coverage**: Unit tests for tools, integration tests for handlers
5. **Documentation**: Doc comments on all public items
6. **Performance**: Consider concurrency and lock contention
7. **Idiomatic Rust**: Follow Rust conventions and best practices
8. **Correct Versions**: Follow each crate's specific version for the docs
