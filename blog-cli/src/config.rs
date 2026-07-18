use blog_client::Transport;
use std::path::PathBuf;

const DEFAULT_HTTP_ADDRESS: &str = "http://127.0.0.1:8080";

const DEFAULT_GRPC_ADDRESS: &str = "http://127.0.0.1:50051";

#[derive(Debug, Clone)]
pub(crate) struct CliConfig {
    pub transport: Transport,
    pub token_file: PathBuf,
    pub json: bool,
}

impl CliConfig {
    pub(crate) fn from_args(
        grpc: bool,
        server: Option<String>,
        token_file: PathBuf,
        json: bool,
    ) -> Self {
        let server = server.unwrap_or_else(|| {
            if grpc {
                DEFAULT_GRPC_ADDRESS.to_string()
            } else {
                DEFAULT_HTTP_ADDRESS.to_string()
            }
        });

        let transport = if grpc {
            Transport::Grpc(server)
        } else {
            Transport::Http(server)
        };

        Self {
            transport,
            token_file,
            json,
        }
    }
}
