/// Marker trait for domain-layer errors.
///
/// The application layer can map `DomainError` into its own error model (`AppError`) without
/// depending on concrete domain error types.
pub trait DomainError: std::error::Error + Send + Sync + 'static {}
