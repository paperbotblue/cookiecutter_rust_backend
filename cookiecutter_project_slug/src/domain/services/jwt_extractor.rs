
use actix_web::dev::ServiceRequest;
use async_trait::async_trait;

use crate::domain::error::CommonError;


#[async_trait]
pub trait JwtExtractorService: 'static + Sync + Send {
    fn get_token(&self, req: &ServiceRequest) -> Result<String, CommonError>;
}
