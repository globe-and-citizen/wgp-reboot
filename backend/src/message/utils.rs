

pub fn create_jwt_token(username: String) -> String {
    // todo: Implement a real JWT token generation
    format!("token_for_{}", username)
}

pub fn get_username_from_token(token: &str) -> Option<&str> {
    token.strip_prefix("token_for_")
}