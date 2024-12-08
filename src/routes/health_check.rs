use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    use super::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let resp = health_check().await;
        assert!(resp.status().is_success());
    }
}
