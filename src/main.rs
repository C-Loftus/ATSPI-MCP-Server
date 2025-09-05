use atspi_mcp::get_active_frame_name;
use rmcp::{
    Json, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EchoRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EchoResponse {
    pub echoed: String,
}

#[derive(Clone)]
pub struct AtspiServer {
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl rmcp::ServerHandler for AtspiServer {}

impl AtspiServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router(router = tool_router)]
impl AtspiServer {
    #[tool(name = "current_apps", description = "Return a list of running applications")]
    pub async fn echo(
        &self,
        params: Parameters<EchoRequest>,
    ) -> Result<Json<EchoResponse>, String> {
        Ok(Json(EchoResponse {
            echoed: params.0.text,
        }))
    }

    #[tool(name = "get_active_frame", description = "Get the name of the active frame, also known as the active window, via atspi")]
    pub async fn get_active_frame(&self) -> String {
        get_active_frame_name().await.unwrap_or("unknown".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Starting Atspi MCP Server...");

    let server = AtspiServer::new();

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
