# rust-simple-blog-api

I have wanted to learn using Rust in server purposes for a long time, I thought starting a simple project could make process of learning faster. So I want to create simple API using Rust.

## Roadmap
My roadmap will be like:
- (1) Initial Project ([>done](https://github.com/aliereno/rust-simple-blog-api/tree/2d48a693a5fcb0bd1cecafbb410018ee2771deff))
- (2) Blog List Endpoint (data served statically)([>done](https://github.com/aliereno/rust-simple-blog-api/tree/79e18cc601dffa7d22cd827885c319659fd31982))
- *(2.1 UPDATE) Change Rocket to Actix*([>done](https://github.com/aliereno/rust-simple-blog-api/commit/b965874d5f1005878c90d050867bb59a58c69026))
- (3) Database integration([>done](https://github.com/aliereno/rust-simple-blog-api/commits/main))
- (3.1) Dockerizing with DB integration([>done](https://github.com/aliereno/rust-simple-blog-api/commits/main))
- (4) Create Blog model and implement migration
- (5) Refactoring CRUD endpoints for database connection
- (6) Auth endpoints | JWT Authentication
- (7) OpenAPI(Swagger) integration
- (8) Logging


## Running
```
cp .env.example .env
make up
```