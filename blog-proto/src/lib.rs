//! Общие protobuf-контракты для сервисов блога.

use prost as _;
use tonic as _;
use tonic_prost as _;

/// Сгенерированные protobuf-типы и gRPC-сервисы.
pub mod generated {
    #![allow(clippy::default_trait_access)]
    #![allow(clippy::doc_markdown)]
    #![allow(clippy::match_single_binding)]
    #![allow(clippy::missing_errors_doc)]
    #![allow(clippy::too_many_lines)]

    tonic::include_proto!("blog.v1");

    /// Закодированный descriptor set protobuf-контрактов блога для gRPC reflection.
    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("blog_descriptor");
}
