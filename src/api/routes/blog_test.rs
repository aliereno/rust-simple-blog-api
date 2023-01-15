
#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, test::TestRequest};
    use serde_json::json;

    use crate::{db::test_db::{TestDb}, models::blog::Blog, api::routes::blog::config_blog};

    #[actix_web::test]
    async fn test_blog() {
        let db = TestDb::new();
        let db_pool = db.conn();
 
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(db_pool.clone()))
                .configure(config_blog),
        )
        .await;

        let request_body = json!({
            "title": "blog test",
            "body": "body test",
            "published": true
        });

        let response = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to create blog {:?}", response);

        let response = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to create blog {:?}", response);
        
        let response = TestRequest::get().uri("/1").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to find blog detail");
        let blog: Blog = test::read_body_json(response).await;
        assert_eq!(blog.title, "blog test", "Found wrong blog {:?}", blog);
        assert_eq!(blog.body, "body test", "Found wrong blog {:?}", blog);
        
        let response = TestRequest::get().uri("/").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to fetch list of blogs");
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(list_of_blogs.len(), 2, "Found wrong length of blogs {:?}", list_of_blogs);

        let request_body = json!({
            "title": "blog test update",
            "body": "body test update",
            "published": true
        });

        let response = TestRequest::put().uri("/1").set_json(&request_body).send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to update blog {:?}", response);
        
        let response = TestRequest::get().uri("/1").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to find blog detail");
        let blog: Blog = test::read_body_json(response).await;
        assert_eq!(blog.title, "blog test update", "Found wrong blog {:?}", blog);
        assert_eq!(blog.body, "body test update", "Found wrong blog {:?}", blog);

        
        let response = TestRequest::delete().uri("/1").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to find blog detail");
        
        let response = TestRequest::get().uri("/").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to fetch list of blogs");
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(list_of_blogs.len(), 1, "Found wrong length of blogs {:?}", list_of_blogs);

        let request_body = json!({
            "title": "blog test",
            "body": "body test",
            "published": false
        });
        let response = TestRequest::post().uri("/").set_json(&request_body).send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to create blog {:?}", response);

        let response = TestRequest::get().uri("/3").send_request(&mut app).await;
        assert_eq!(response.status(), 404, "Expected 404 for getting published:false Blog");
        
        let response = TestRequest::get().uri("/").send_request(&mut app).await;
        assert!(response.status().is_success(), "Failed to fetch list of blogs");
        let list_of_blogs: Vec<Blog> = test::read_body_json(response).await;
        assert_eq!(list_of_blogs.len(), 1, "Found wrong length of blogs {:?}", list_of_blogs);
    }
}