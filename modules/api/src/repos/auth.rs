use jsonwebtoken::EncodingKey;
use salvo::prelude::*;
use crate::models::*;
use salvo::http::{Method, StatusError};
use time::{Duration, OffsetDateTime};

const SECRET_KEY: &str = "MYSUPERSECRETKEY";

#[handler]
pub async fn login(req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) -> anyhow::Result<()> {
    if req.method() == Method::POST {
        let (username, password) = (
            req.form::<String>("username").await.unwrap_or_default(),
            req.form::<String>("password").await.unwrap_or_default(),
        );

        if !validate(&username, &password) {
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
                res.render(Text::Plain((format!("Hi {}, have logged in successfully!", data.claims.username))));
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
