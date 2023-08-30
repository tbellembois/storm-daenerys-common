pub enum CommonError {
    NotFound,
    InvalidQueryParamater(String),
    MissingQueryParamater(String),
    InvalidDirectoryName,
    InternalServerError(String),
    Timeout(String),
}