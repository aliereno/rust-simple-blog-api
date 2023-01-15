# rust-simple-blog-api

I have wanted to learn using Rust in server purposes for a long time, I thought starting a simple project could make process of learning faster. So I want to create simple API using Rust.

## Roadmap
My roadmap will be like:
- (1) Initial Project ([>done](https://github.com/aliereno/rust-simple-blog-api/tree/2d48a693a5fcb0bd1cecafbb410018ee2771deff))
- (2) Blog List Endpoint (data served statically)([>done](https://github.com/aliereno/rust-simple-blog-api/tree/79e18cc601dffa7d22cd827885c319659fd31982))
- *(2.1 UPDATE) Change Rocket to Actix*([>done](https://github.com/aliereno/rust-simple-blog-api/commit/b965874d5f1005878c90d050867bb59a58c69026))
- (3) Database integration([>done](https://github.com/aliereno/rust-simple-blog-api/tree/93cf2dcf305a9f22c5ce8c28f7ab27ed51e739f9))
- (3.1) Dockerizing with DB integration([>done](https://github.com/aliereno/rust-simple-blog-api/tree/93cf2dcf305a9f22c5ce8c28f7ab27ed51e739f9))
- (4) Create Blog model and implement migration([>done](https://github.com/aliereno/rust-simple-blog-api/tree/4ec8cebf347aa46296b72522a080babd57a3b974))
- (5) Refactoring CRUD endpoints for database connection([>done](https://github.com/aliereno/rust-simple-blog-api/tree/6a22b548a037e6c30ab08604755491551af9aa0c))
- (6) Write tests for CRUD endpoints([>done, but i will improve](https://github.com/aliereno/rust-simple-blog-api/tree/d68b99069c4ae570c2963e686122582c065368e6))
- (7) Auth endpoints | JWT Authentication
- (8) OpenAPI(Swagger) integration
- (9) Logging


## Running
```
cp .env.example .env
make up
```