#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, http, test, test::TestRequest};
    use serde_json::json;

    use crate::{app::create_app, db::tests::get_test_db, models::blog::Blog};

    #[actix_web::test]
    async fn test_blog() {
        let db_pool = get_test_db();

        let app = test::init_service(create_app(&db_pool)).await;

        let request_body = json!({
            "title": "blog test",
            "body": "body test",
            "published": true
        });

        let response = TestRequest::post()
            .uri("/api/blog/")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(
            response.status().is_success(),
            "Failed to create blog {}",
            response.status()
        );

        let response = TestRequest::post()
            .uri("/api/blog/")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(
            response.status().is_success(),
            "Failed to create blog {}",
            response.status()
        );

        let response = TestRequest::get().uri("/api/blog/1").send_request(&app).await;
        assert!(
            response.status().is_success(),
            "Failed to find blog detail {}",
            response.status()
        );
        let blog: Blog = test::read_body_json(response).await;
        assert_eq!(blog.title, "blog test", "Found wrong blog {:?}", blog);
        assert_eq!(blog.body, "body test", "Found wrong blog {:?}", blog);

        let response = TestRequest::get().uri("/api/blog/").send_request(&app).await;
        assert!(
            response.status().is_success(),
            "Failed to fetch list of blogs {}",
            response.status()
        );
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(
            list_of_blogs.len(),
            2,
            "Found wrong length of blogs {:?}",
            list_of_blogs
        );

        let request_body = json!({
            "title": "blog test update",
            "body": "body test update",
            "published": true
        });

        let response = TestRequest::put()
            .uri("/api/blog/1")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(response.status().is_success(), "Failed to update blog");

        let response = TestRequest::get().uri("/api/blog/1").send_request(&app).await;
        assert!(response.status().is_success(), "Failed to find blog detail");
        let blog: Blog = test::read_body_json(response).await;
        assert_eq!(blog.title, "blog test update", "Found wrong blog {:?}", blog);
        assert_eq!(blog.body, "body test update", "Found wrong blog {:?}", blog);

        let response = TestRequest::delete().uri("/api/blog/1").send_request(&app).await;
        assert!(response.status().is_success(), "Failed to find blog detail");

        let response = TestRequest::get().uri("/api/blog/").send_request(&app).await;
        assert!(response.status().is_success(), "Failed to fetch list of blogs");
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(
            list_of_blogs.len(),
            1,
            "Found wrong length of blogs {:?}",
            list_of_blogs
        );

        let request_body = json!({
            "title": "blog test",
            "body": "body test",
            "published": false
        });
        let response = TestRequest::post()
            .uri("/api/blog/")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(response.status().is_success(), "Failed to create blog");

        let response = TestRequest::get().uri("/api/blog/3").send_request(&app).await;
        assert_eq!(
            response.status(),
            http::StatusCode::NOT_FOUND,
            "Expected 404 for getting published:false Blog"
        );
        let response_body = to_bytes(response.into_body()).await.unwrap_or_default();
        assert_eq!(
            response_body,
            b"{\"message\":\"Record not found\"}"[..],
            "Expected valid message for not found Blog request"
        );

        let response = TestRequest::get().uri("/api/blog/").send_request(&app).await;
        assert!(response.status().is_success(), "Failed to fetch list of blogs");
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(
            list_of_blogs.len(),
            1,
            "Found wrong length of blogs {:?}",
            list_of_blogs
        );
    }

    #[actix_web::test]
    async fn test_blog_not_found() {
        let db_pool = get_test_db();

        let app = test::init_service(create_app(&db_pool)).await;

        let response = TestRequest::get().uri("/api/blog/0").send_request(&app).await;
        assert_eq!(
            response.status(),
            http::StatusCode::NOT_FOUND,
            "Expected 404 for getting Blog that not exist"
        );
        let response_body = to_bytes(response.into_body()).await.unwrap_or_default();
        assert_eq!(
            response_body,
            b"{\"message\":\"Record not found\"}"[..],
            "Expected valid message for not found Blog request"
        );

        let response = TestRequest::delete().uri("/api/blog/0").send_request(&app).await;
        assert_eq!(
            response.status(),
            http::StatusCode::NOT_FOUND,
            "Expected 404 for deleting Blog that not exist"
        );
        let response_body = to_bytes(response.into_body()).await.unwrap_or_default();
        assert_eq!(
            response_body,
            b"{\"message\":\"Record not found\"}"[..],
            "Expected valid message for not found Blog request"
        );

        let request_body = json!({
            "title": "blog test",
            "body": "body test",
            "published": true
        });

        let response = TestRequest::put()
            .uri("/api/blog/0")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert_eq!(
            response.status(),
            http::StatusCode::NOT_FOUND,
            "Expected 404 for updating Blog that not exist"
        );
        let response_body = to_bytes(response.into_body()).await.unwrap_or_default();
        assert_eq!(
            response_body,
            b"{\"message\":\"Record not found\"}"[..],
            "Expected valid message for not found Blog request"
        );

        let response = TestRequest::put()
            .uri("/api/blog/0")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert_eq!(
            response.status(),
            http::StatusCode::NOT_FOUND,
            "Expected 404 for updating Blog that not exist"
        );
        let response_body = to_bytes(response.into_body()).await.unwrap_or_default();
        assert_eq!(
            response_body,
            b"{\"message\":\"Record not found\"}"[..],
            "Expected valid message for not found Blog request"
        );
    }

    #[actix_web::test]
    async fn test_invalid_request_params() {
        let db_pool = get_test_db();

        let app = test::init_service(create_app(&db_pool)).await;

        let response = TestRequest::post().uri("/api/blog/").send_request(&app).await;
        assert_eq!(
            response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY,
            "Expected 422 for invalid Request Body"
        );
        // TODO: decide how 422 response will be
        //let response_body = to_bytes(response.into_body()).await.unwrap();
        //assert_eq!(response_body, "{\"message\": \"\"}", "Expected ..");

        let response = TestRequest::put().uri("/api/blog/0").send_request(&app).await;
        assert_eq!(
            response.status(),
            http::StatusCode::UNPROCESSABLE_ENTITY,
            "Expected 422 for invalid Request Body"
        );
        //let response_body = to_bytes(response.into_body()).await.unwrap();
        //assert_eq!(response_body, "{\"message\": \"\"}", "Expected ..");
    }
}
