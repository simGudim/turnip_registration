#[cfg(test)]
mod tests {
    use actix_web::{test::{self, TestRequest}, App};
    use actix_web::http::StatusCode;
    use serde_json::json;

    #[actix_web::test]
    async fn test_insert_user() {
        let mut app = test::init_service(
            App::new()
        ).await;
        let request_body = json!({
            "username": "simong_test"
        });
        let resp = TestRequest::post()
            .uri("/user")
            .set_json(request_body)
            .send_request(&mut app)
            .await;
        assert_eq!(resp.status(), StatusCode::from_u16(200).unwrap());
    }
}