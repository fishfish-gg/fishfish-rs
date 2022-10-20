mod create_domain;
mod get_status;
mod create_session;
mod update_domain;
mod delete_domain;
mod get_domains;

pub use create_domain::CreateDomainEndpoint;
pub use get_status::GetStatusEndpoint;
pub use create_session::CreateSessionEndpoint;
pub use update_domain::UpdateDomainEndpoint;
pub use delete_domain::DeleteDomainEndpoint;
pub use get_domains::GetDomainsEndpoint;