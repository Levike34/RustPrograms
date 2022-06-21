use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, Server};
use dotenv;
use uuid::Uuid;
use sqlx::Pool;
use sqlx::{query, query_as, PgPool};

#[derive(Clone, Debug)]
struct State {
    db_pool: PgPool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    id: Uuid,
    name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Users {
    users: Vec<User>,
}

struct RestEntity {
    base_path: String,
}

impl RestEntity {
    async fn create(mut req: Request<State>) -> tide::Result {
        let user: User = req.body_json().await?;
        // let get a mut ref of our store ( hashMap )
        let db_pool = req.state().db_pool.clone();
        let row = query_as!(
            User,
            r#"
            INSERT INTO pixelworld (id, name) VALUES
            ($1, $2) returning id, name
            "#,
            user.id,
            user.name
        )
        .fetch_one(&db_pool)
        .await?;

        let mut res = Response::new(201);
        res.set_body(Body::from_json(&row)?);
        Ok(res)
    }

    async fn list(req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        // get all the dinos as a vector
        let rows = query_as!(
            User,
            r#"
            SELECT id, name from pixelworld
            "#
        )
        .fetch_all(&db_pool)
        .await?;

        let mut res = Response::new(200);
        res.set_body(Body::from_json(&rows)?);
        Ok(res)
    }

    async fn get(req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
        let row = query_as!(
            User,
            r#"
            SELECT  id, name from pixelworld
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                r.set_body(Body::from_json(&row)?);
                r
            }
        };

        Ok(res)
    }

    async fn update(mut req: tide::Request<State>) -> tide::Result {
        let user: User = req.body_json().await?;
        let db_pool = req.state().db_pool.clone();
        let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
        let row = query_as!(
            User,
            r#"
            UPDATE pixelworld SET name = $2
            WHERE id = $1
            returning id, name
            "#,
            id,
            user.name
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                r.set_body(Body::from_json(&row)?);
                r
            }
        };

        Ok(res)
    }

    async fn delete(req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        let id: Uuid = Uuid::parse_str(req.param("id")?).unwrap();
        let row = query!(
            r#"
            delete from pixelworld
            WHERE id = $1
            returning id
            "#,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(_) => Response::new(204),
        };

        Ok(res)
    }
}

pub async fn make_db_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    Pool::new(&db_url).await.unwrap()
}
#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();
    let db_pool = make_db_pool().await;
    let app = server(db_pool).await;

    app.listen("127.0.0.1:8080").await.unwrap();
}

fn register_rest_entity(app: &mut Server<State>, entity: RestEntity) {
    app.at(&entity.base_path)
        .get(RestEntity::list)
        .post(RestEntity::create);

    app.at(&format!("{}/:id", entity.base_path))
        .get(RestEntity::get)
        .put(RestEntity::update)
        .delete(RestEntity::delete);
}

async fn server(users_store: PgPool) -> Server<State> {
    let state = State {
        db_pool: users_store, //Default::default(),
    };

    let mut app = tide::with_state(state);
    app.at("/").get(|_| async { Ok("ok") });

    let users_endpoint = RestEntity {
        base_path: String::from("/users"),
    };
    
    register_rest_entity(&mut app, users_endpoint);

    app
}


#[async_std::test]
async fn get_user() -> tide::Result<()> {
    dotenv::dotenv().ok();
    use tide::http::{Method, Request, Response, Url};

    let user = User {
        id: Uuid::new_v4(),
        name: String::from("levi")
    };

    let db_pool = make_db_pool().await;

        //Make the user to test get
       query!(
        r#"
        INSERT INTO pixelworld (id, name) VALUES
        ($1, $2) returning id, name
        "#,
        user.id,
        user.name
    )
    .fetch_one(&db_pool)
    .await?;

    let app = server(db_pool).await;

    let url = Url::parse(format!("https://example.com/users/{}", &user.id).as_str()).unwrap();
    let req = Request::new(Method::Get, url);

    let res: Response = app.respond(req).await?;
    assert_eq!(200, res.status());
    Ok(())
}

#[async_std::test]
async fn create_user() -> tide::Result<()> {
    dotenv::dotenv().ok();
    use tide::http::{Method, Request, Response, Url};

    let user = User {
        id: Uuid::new_v4(),
        name: String::from("levi")
    };

    let db_pool = make_db_pool().await;
    let app = server(db_pool).await;

    let url = Url::parse("https://example.com/users").unwrap();
    let mut req = Request::new(Method::Post, url);
    req.set_body(serde_json::to_string(&user)?);
    let res: Response = app.respond(req).await?;
    assert_eq!(201, res.status());
    Ok(())
}

#[async_std::test]
async fn delete_user() -> tide::Result<()> {
    dotenv::dotenv().ok();
    use tide::http::{Method, Request, Response, Url};

    let user = User {
        id: Uuid::new_v4(),
        name: String::from("levi")
    };

    //Make the user to test
    let db_pool = make_db_pool().await;
           query!(
            r#"
            INSERT INTO pixelworld (id, name) VALUES
            ($1, $2) returning id, name
            "#,
            user.id,
            user.name
        )
        .fetch_one(&db_pool)
        .await?;

    let app = server(db_pool).await;

    let url = Url::parse(format!("https://example.com/users/{}", &user.id).as_str()).unwrap();
    let mut req = Request::new(Method::Delete, url);
    req.set_body(serde_json::to_string(&user)?);
    let res: Response = app.respond(req).await?;
    assert_eq!(204, res.status());
    Ok(())
}
