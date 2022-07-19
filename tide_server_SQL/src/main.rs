use serde::{Deserialize, Serialize};
use tide::{Body, Request, Response, Server};
use dotenv;
use sqlx::{query, MySqlPool};
use sha2::{Sha256, Sha512, Digest};

#[derive(Clone, Debug)]
struct State {
    db_pool: MySqlPool ,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    name: String,
    id: i32,
    owner: String,
    reason: String,
    created: String,
    amountToRaise: i32,
    raised: i32,
    facebook: String,
    twitter: String,
    youtube: String
    }

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Donation {
    raised: i32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Social {
    facebook: String,
    twitter: String,
    youtube: String
}


#[derive(Debug, Clone, Deserialize, Serialize)]
struct Users {
    users: Vec<User>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Counter {
    from: i32,
    to: i32,
}

struct RestEntity {
    base_path: String,
}

impl RestEntity {
    async fn create(mut req: Request<State>) -> tide::Result {
        let user: User = req.body_json().await?;
        // let get a mut ref of our store ( hashMap )
        let db_pool = req.state().db_pool.clone();
        let row = query!(
            r#"
            INSERT INTO users (name, id, owner, reason, amountToRaise, raised, created, facebook, twitter, youtube)
            VALUES ( ?, NULL, ?, ?, ?, ?, ?, ?, ?, ? )
            "#,
            user.name,
            user.owner,
            user.reason,
            user.created,
            user.amountToRaise,
            user.raised,
            user.facebook,
            user.twitter,
            user.youtube
        )
        .fetch_one(&db_pool)
        .await?;

        let mut res = Response::new(201);
        let x = String::from(&user.name);
        res.set_body(Body::from_string(x));
        println!("{:?}", &row);
        Ok(res)
    }

    async fn list(req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        // get all the dinos as a vector
        let rows = query!(
            "
            SELECT * FROM users
            "
        )
        .fetch_all(&db_pool)
        .await?;

        let mut res = Response::new(200);
        let x = String::from(format!("{:?}", &rows,));

        res.set_body(Body::from_string(x));
        println!("{:?}", &rows);
        Ok(res)
    }

    async fn get(req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        let id = req.param("id")?;
        let row = query!(
            r#"
            SELECT * FROM users
            WHERE id = (?)
            "#,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                let x = String::from(format!("{:?}", &row));

                r.set_body(Body::from_string(x));
                r
            }
        };

        Ok(res)
    }

    async fn update(mut req: tide::Request<State>) -> tide::Result {
        let user: User = req.body_json().await?;
        let db_pool = req.state().db_pool.clone();
        let id = req.param("id")?;
        let row = query!(
            r#"
            UPDATE users 
            SET name = ?, 
            id = ?, 
            owner= ?, 
            reason= ?, 
            created= ?, 
            amountToRaise= ?, 
            raised= ?,
            facebook= ?, 
            twitter= ?,
            youtube= ?
                
            WHERE id = ?
            "#,
            user.name,
            user.id,
            user.owner,
            user.reason,
            user.created,
            user.amountToRaise,
            user.raised,
            user.facebook,
            user.twitter,
            user.youtube,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                let x = String::from(&user.facebook);
                r.set_body(Body::from_string(x));
                r
            }
        };

        Ok(res)
    }

    async fn process_donation(mut req: tide::Request<State>) -> tide::Result {
        let donation: Donation = req.body_json().await?;
        let db_pool = req.state().db_pool.clone();
        let id = req.param("id")?;
        let row = query!(
            r#"
            UPDATE users 
            SET raised = ? 
            WHERE id = ?
            "#,
          
            donation.raised,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                r
            }
        };

        Ok(res)
    }

    async fn update_social_media(mut req: tide::Request<State>) -> tide::Result {
        let social: Social = req.body_json().await?;
        let db_pool = req.state().db_pool.clone();
        let id = req.param("id")?;
        let row = query!(
            r#"
            UPDATE users 
            SET facebook = ? , twitter = ?, youtube = ?
            WHERE id = ?
            "#,
          
            social.facebook,
            social.twitter,
            social.youtube,
            id
        )
        .fetch_optional(&db_pool)
        .await?;

        let res = match row {
            None => Response::new(404),
            Some(row) => {
                let mut r = Response::new(200);
                r
            }
        };

        Ok(res)
    }

    // Inserts multiple records into db automatically based on counter requested.
    async fn filler_script(mut req: tide::Request<State>) -> tide::Result {
        let mut counter: Counter = req.body_json().await?;
        while counter.from <= counter.to {
            // let hash = sha2::Sha256::digest(String::from(format!("yolko{}", counter)).as_bytes());
            // let mut result = hash;
        
            // let x: [u8; 32] = result.as_slice().try_into().unwrap();

            let new_name = String::from(format!("bill no. {}", counter.from));

            let user: User = User {
                name: new_name,
                id: 1,
                owner: counter.from.to_string(),
                reason: "a".to_string(),
                amountToRaise: counter.from,
                raised: 0,
                created: "10-05-1996".to_string(),
                facebook: "fb".to_string(),
                twitter: "twtt".to_string(),
                youtube: "non".to_string()

            };
            let db_pool = req.state().db_pool.clone();
            let row = query!(
                r#"
                INSERT INTO users (name, id, owner, reason, amountToRaise, raised, created, facebook, twitter, youtube)
                VALUES ( ?, NULL, ?, ?, ?, ?, ?, ?, ?, ? )
                "#,
                user.name,
                user.owner,
                user.reason,
                user.created,
                user.amountToRaise,
                user.raised,
                user.facebook,
                user.twitter,
                user.youtube
            )
            .fetch_all(&db_pool)
            .await?;
    
          
            println!("{:?}", &row);
            counter.from += 1;
        }
        let mut res = Response::new(201);

        Ok(res)

    }

    async fn delete(mut req: tide::Request<State>) -> tide::Result {
        let db_pool = req.state().db_pool.clone();
        let id = req.param("id")?;
        let row = query!(
            r#"
            DELETE FROM users
            WHERE id = (?)
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

pub async fn make_db_pool() -> MySqlPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    MySqlPool::connect(&db_url).await.unwrap()
}
#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();
    let db_pool = make_db_pool().await;
    let app = server(db_pool).await;
    
    app.listen("127.0.0.1:8081").await.unwrap();
}

fn register_rest_entity(app: &mut Server<State>, entity: RestEntity) {
    app.at(&entity.base_path)
        .get(RestEntity::list)
        .post(RestEntity::create);

    app.at(&format!("{}/:id", entity.base_path))
        .get(RestEntity::get)
        .put(RestEntity::update)
        .delete(RestEntity::delete);
    
    app.at(&format!("{}/:id/socials", entity.base_path))
        .put(RestEntity::update_social_media);
    
    app.at(&format!("{}/:id/finance", entity.base_path))
        .put(RestEntity::process_donation);

    app.at(&format!("{}/:script", entity.base_path))
        .post(RestEntity::filler_script);
}


async fn server(users_store: MySqlPool) -> Server<State> {
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
        name: String::from("Levi"),
        owner: String::from("0x2"),
        reason: String::from("Raising lots of funds."),
        facebook: String::from("facebook.com/levi"),
        twitter: String::from("twitter.com/levi")

    };

    let db_pool = make_db_pool().await;

        //Make the user to test get
       query!(
        r#"
        INSERT INTO users (id, name, owner, reason, facebook, twitter) VALUES
        ($1, $2, $3, $4, $5, $6) returning id, name, owner, reason, facebook, twitter
        "#,
        user.id,
        user.name,
        user.owner,
        user.reason,
        user.facebook,
        user.twitter
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
        name: String::from("Levi"),
        owner: String::from("0x2"),
        reason: String::from("Raising lots of funds."),
        facebook: String::from("facebook.com/levi"),
        twitter: String::from("twitter.com/levi")

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
            name: String::from("Levi"),
            owner: String::from("0x2"),
            reason: String::from("Raising lots of funds."),
            facebook: String::from("facebook.com/levi"),
            twitter: String::from("twitter.com/levi")
    
        };

    //Make the user to test
    let db_pool = make_db_pool().await;
    query!(
        r#"
        INSERT INTO users (id, name, owner, reason, facebook, twitter) VALUES
        ($1, $2, $3, $4, $5, $6) returning id, name, owner, reason, facebook, twitter
        "#,
        user.id,
        user.name,
        user.owner,
        user.reason,
        user.facebook,
        user.twitter
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

