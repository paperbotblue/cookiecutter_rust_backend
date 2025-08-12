#[derive(Debug)]
pub enum ApiResponseCode {
    Ok,
    Created,
    /// 202 Accepted: The request has been accepted for processing, but not completed.
    Accepted,
    /// 204 No Content: The request was successful, but there's no content to return.
    NoContent,

    /// 400 Bad Request: The server couldn't understand the request due to invalid syntax.
    BadRequest, // 400
    /// 401 Unauthorized: Authentication is required and has failed or not been provided.
    Unauthorized, // 401
    /// 403 Forbidden: The client does not have access rights to the content.
    Forbidden, // 403
    /// 404 Not Found: The requested resource could not be found.
    NotFound, // 404
    /// 409 Conflict: The request could not be completed due to a conflict with the current state of the resource.
    Conflict, // 409

    /// 500 Internal Server Error: A generic server error.
    InternalServerError, // 500
}
impl ApiResponseCode {
    pub fn status_code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NoContent => 204,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::InternalServerError => 500,
        }
    }
}
