use axum::{RequestPartsExt, async_trait, extract::{FromRequestParts, State}, http::{Request, request::Parts}, middleware::{self, Next}, response::{IntoResponse, Response}};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};
use crate::{ctx::Ctx, error::{Error, Result}, model::ModelController};

use crate::web::AUTH_TOKEN;

pub async fn mw_require_auth(
    ctx: Result<Ctx>,
    req: axum::extract::Request,
    next: middleware::Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    // Extractor context
    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: axum::extract::Request,
    next: middleware::Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    /* Extract the auth token from cookies */
    let auth_token = cookies
    .get(AUTH_TOKEN)
    .map(|c| c.value().to_string());

    /** 
     * Build Ctx - produces: Result<Ctx, Error> 
     * No cookie - Err(AuthFailNoAuthTokenCookie)
     * Bad token - Err(AuthFailTokenWrongFormat)
     * Valid token - Ok(Ctx { user_id })
    */
    let result_ctx = match auth_token
    .ok_or(Error::AuthFailNoAuthTokenCookie)
    .and_then(parse_token) {
        Ok((user_id, _exp, _sign )) => {
            Ok(Ctx::new(user_id))
        }
        Err(e) => Err(e)
    };

    /* Remove cookie if there is an error and it is not NoAuthTokenCookie */
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error:: AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    /* Store the ctx_result in the request extension */
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
} 

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts.extensions
        .get::<Result<Ctx>>()
        .ok_or(Error::AuthFailCtxNotInRequestExt)?
        .clone()
    } 
}


fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id,   exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id
    .parse()
    .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}