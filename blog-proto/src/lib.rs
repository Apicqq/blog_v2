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

    tonic::include_proto!("blog.v1");
}
