use tower_cookies::{Cookie, Cookies};

pub async fn logout(cookies: Cookies) {
    cookies.add(
        Cookie::build(("refresh-token", ""))
            .path("/")
            .http_only(true)
            .build(),
    );
	println!("logged out");
}
