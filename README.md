#####
Auth service
####
Casbin RBAC client permissions service

###
1. configure .env
2. run migration `cargo sqlx migrate run`
3. (optional) prepare offline data for sqlx `cargo sqlx prepare`
4. run server `cargo run`

###
swagger spec 

http://localhost:8082/swagger-ui/index.html?url=/swagger-spec