use jsonwebtoken::EncodingKey;
use salvo::prelude::*;
use crate::models::*;
use salvo::http::{Method, StatusError};
use time::{Duration, OffsetDateTime};
use crate::middlewares::mongo_crud::MongoController;

// JWT secret for encode/decode.
const SECRET_KEY: &str = "MYSUPERSECRETKEY";

#[handler]
pub async fn register(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> &'static str {
    if req.method() == Method::POST {
        let mut connection_base = depot.obtain::<MongoController>();
        let result = match req.parse_json::<NewUser>().await.unwrap() {
            NewUser => connection_base.expect("Cannot register user!").register_user(NewUser).await,
            ParseError => false,
        };
        return "Hello World!";
    }
    return "Hello World!";

}

// Basic api handler http://127.0.0.1:5800/auth/login. It generates JWT
// but i will rewrite this function for add some features - refresh token and uuid/role.
#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> anyhow::Result<()> {
    let mut collection_base = depot.obtain::<MongoController>();

    if req.method() == Method::POST {
        let (username, password) = (
            req.form::<String>("username").await.unwrap_or_default(),
            req.form::<String>("password").await.unwrap_or_default(),
        );
        if !collection_base.expect("SOME_REASON").validate_user(&username, &password).await {
            res.render(Text::Html(LOGIN_HTML));
            return Ok(());
        }
        let exp = OffsetDateTime::now_utc() + Duration::days(14);
        let claim = TokenClaims {
            username,
            exp: exp.unix_timestamp(),
        };

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claim,
            &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
        )?;
        let tokens = Token {
            token,
            exp: exp.unix_timestamp(),
        };
        res.render(Json(tokens));
    } else {
        match depot.jwt_auth_state() {
            JwtAuthState::Authorized => {
                let data = depot.jwt_auth_data::<TokenClaims>().unwrap();
                res.render(Text::Plain(format!("Hi {}, have logged in successfully!", data.claims.username)));
            }
            JwtAuthState::Unauthorized => {
                res.render(Text::Html(LOGIN_HTML));
            }
            JwtAuthState::Forbidden => {
                res.render(Text::Plain(format!("Forbidden")));
            }
        }
    }
    Ok(())
}

// Temp login validation server.
fn validate(username: &str, password: &str) -> bool {
    username == "root" && password == "pwd"
}

static LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>JWT Auth Demo</title>
    </head>
    <body>
        <h1>JWT Auth</h1>
        <form action="/auth/login" method="post">
        <label for="username"><b>Username</b></label>
        <input type="text" placeholder="Enter Username" name="username" required>
    
        <label for="password"><b>Password</b></label>
        <input type="password" placeholder="Enter Password" name="password" required>
    
        <button type="submit">Login</button>
    </form>
    </body>
</html>
"#;
