use std::collections::HashMap;

use poem::{listener::TcpListener, route};
use poem_openapi::{
    payload::Json, types::Password, Object, OpenApi, OpenApiService, Response, Tags,
};
use tokio::sync::Mutex;

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
}

/// Create user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct User {
    /// Id
    #[oai(max_length = 20)]
    id: String,
    /// Name
    #[oai(max_length = 64)]
    name: String,
    /// Password
    #[oai(max_length = 32)]
    password: Password,
}

/// Update user schema
#[derive(Debug, Object, Clone, Eq, PartialEq)]
struct UpdateUser {
    /// Name
    name: Option<String>,
    /// Password
    password: Option<Password>,
}

#[derive(Response)]
enum CreateUserResponse {
    /// Returns when the user is successfully created.
    #[oai(status = 200)]
    Ok,
    /// Returns when the user already exists.
    #[oai(status = 409)]
    UserAlreadyExists,
}

#[derive(Response)]
enum FindUserResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<User>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(Response)]
enum DeleteUserResponse {
    /// Returns when the user is successfully deleted.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(Response)]
enum UpdateUserResponse {
    /// Returns when the user is successfully updated.
    #[oai(status = 200)]
    Ok,
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound,
}

#[derive(Default)]
struct Api {
    users: Mutex<HashMap<String, User>>,
}

#[OpenApi]
impl Api {
    /// Create a new user
    #[oai(path = "/users", method = "post", tag = "ApiTags::User")]
    async fn create_user(&self, user: Json<User>) -> CreateUserResponse {
        let mut users = self.users.lock().await;
        if users.contains_key(&user.0.id) {
            return CreateUserResponse::UserAlreadyExists;
        }
        users.insert(user.0.id.clone(), user.0);
        CreateUserResponse::Ok
    }

    /// Find user by id
    #[oai(path = "/users/:user_id", method = "get", tag = "ApiTags::User")]
    async fn find_user(
        &self,
        #[oai(name = "user_id", in = "path", max_length = 20)] user_id: String,
    ) -> FindUserResponse {
        let users = self.users.lock().await;
        match users.get(&user_id) {
            Some(user) => FindUserResponse::Ok(Json(user.clone())),
            None => FindUserResponse::NotFound,
        }
    }

    /// Delete user by id
    #[oai(path = "/users/:user_id", method = "delete", tag = "ApiTags::User")]
    async fn delete_user(
        &self,
        #[oai(name = "user_id", in = "path", max_length = 20)] user_id: String,
    ) -> DeleteUserResponse {
        let mut users = self.users.lock().await;
        match users.remove(&user_id) {
            Some(_) => DeleteUserResponse::Ok,
            None => DeleteUserResponse::NotFound,
        }
    }

    /// Update user by id
    #[oai(path = "/users/:user_id", method = "put", tag = "ApiTags::User")]
    async fn put_user(
        &self,
        #[oai(name = "user_id", in = "path", max_length = 20)] user_id: String,
        update: Json<UpdateUser>,
    ) -> UpdateUserResponse {
        let mut users = self.users.lock().await;
        match users.get_mut(&user_id) {
            Some(user) => {
                if let Some(name) = update.0.name {
                    user.name = name;
                }
                if let Some(password) = update.0.password {
                    user.password = password;
                }
                UpdateUserResponse::Ok
            }
            None => UpdateUserResponse::NotFound,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000");
    let api_service = OpenApiService::new(Api::default())
        .title("Users")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui("http://localhost:3000");

    poem::Server::new(listener)
        .await
        .unwrap()
        .run(route().nest("/api", api_service).nest("/", ui))
        .await
        .unwrap();
}
