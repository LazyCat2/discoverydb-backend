#[get("/users/@me")]
pub async fn whoami(user: User) -> APIResponce<User> {
    user
}
