use atspi::connection::set_session_accessibility;
use atspi_mcp::{get_active_frame_name, get_running_apps};
use rmcp::{
    ServiceExt,
    handler::server::{router::tool::ToolRouter},
    tool, tool_handler, tool_router,
    transport::stdio,
};

#[derive(Clone)]
pub struct AtspiServer {
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl rmcp::ServerHandler for AtspiServer {}
impl AtspiServer {
    pub async fn new() -> Result<AtspiServer, Box<dyn std::error::Error>> {
        Ok(Self {
            tool_router: Self::tool_router(),
        })
    }
}

#[tool_router(router = tool_router)]
impl AtspiServer {
    #[tool(name = "running_app", description = "Return a list of running applications")]
    pub async fn get_running_apps(
        &self,
    ) -> String {
        get_running_apps().await.unwrap()
    }

    #[tool(name = "get_active_frame", description = "Get the name of the active frame, also known as the active window, via atspi")]
    pub async fn get_active_frame(&self) -> String {
        get_active_frame_name().await.unwrap_or_else(|error | error.to_string())
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("Running Atspi MCP Server...");

	set_session_accessibility(true).await?;

    let server = AtspiServer::new().await.unwrap();

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
