use rand::Rng;

use crate::helper::{get_bool_from_input, get_string_from_input};


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Geo {
    lat: String,
    lng: String
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code, non_snake_case)]
struct Company {
    name: String,
    catchPhrase: String,
    bs: String
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code, non_snake_case)]
struct PostResponse {
    id: i32,
    title: String,
    body: String
}

#[derive(serde::Serialize)]
struct PostRequest {
    id: u32,
    title: String,
    body: String
}

async fn try_create_post(client: &reqwest::Client) -> Result<()> {
    let id = rand::thread_rng().gen_range(u8::MIN..u8::MAX);
    let mut title = String::new();
    let mut body = String::new();

    get_string_from_input("Put your post title:", &mut title);
    get_string_from_input("Put your post body:", &mut body);

    let post = PostRequest {
        id: id as u32,
        title,
        body
    };

    let resp: PostResponse = client.post("https://jsonplaceholder.typicode.com/posts")
        .json(&post)
        .send().await?
        .json().await?;

    println!("Response: {:#?}", resp);

    Ok(())
}

async fn try_get_users(client: &reqwest::Client) -> Result<()> {
    let resp: Vec<User> = client.get("https://jsonplaceholder.typicode.com/users")
        .send().await?
        .json().await?;

    let users = resp
        .iter()
        .map(|u| format!("#{} {} (@{}) | {} / {}", u.id, u.name, u.username, u.email, u.phone))
        .collect::<Vec<_>>()
        .join("\n");

    println!("Users:\n{}\nWARNING: They are not real users!", users);

    Ok(())
}

#[tokio::main]
pub async fn request_init() {
    let client = reqwest::Client::new();

    let _ = try_get_users(&client).await;

    let try_create = get_bool_from_input("Do you want to try post creating? w/POST method request");
    if try_create { 
        let _ = try_create_post(&client).await;
    }
}
