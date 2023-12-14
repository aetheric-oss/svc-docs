//! Re-export of used objects

pub use super::client as docs;
pub use super::service::Client as DocsServiceClient;
pub use docs::DocsClient;

pub use lib_common::grpc::Client;
