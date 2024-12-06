use axum::{
    extract::{Path, Query, State}, http::StatusCode, routing::{delete, get, patch, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{collections::HashMap, env};
use std::sync::Arc;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // initialize tracing
    tracing_subscriber::fmt::init();

    let pool = Arc::new(SqlitePool::connect(&database_url).await.unwrap());
    // build our application with a route
    let app = Router::new()
        .route("/equip", get(get_items))
        .route("/equip", post(create_item))
        .route("/equip", patch(update_item))
        .route("/equip/:id", delete(delete_item))
        .with_state(pool.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_items(
    State(pool): State<Arc<SqlitePool>>,  
    Query(params): Query<HashMap<String, String>>  
) -> (StatusCode, Json<Vec<EquipData>>) {
    let data = sqlx::query_as!(EquipData, 
    r##"SELECT id, applicant_name, disposaler_name, equipment_type, equipment_name, purchase_url, cost, situation as "situation: Situation" , approval_date FROM equips"##)
    .fetch_all(&*pool).await;
    match data {
        Ok(r ) => (StatusCode::OK, Json(r)),
        Err(e) => panic!("{}", e),
    }
}


async fn create_item(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<CreateItem>,
) -> (StatusCode, Json<ItemID>) {
    let idr = sqlx::query_as!(ItemID, 
        "INSERT INTO equips (applicant_name, disposaler_name, equipment_type, equipment_name, purchase_url, cost) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", 
        payload.applicant_name, payload.disposaler_name, payload.equipment_type, payload.equipment_name, payload.purchase_url, payload.cost)
    .fetch_one(&*pool).await;
    match idr {
        Ok(r ) => (StatusCode::CREATED, Json(r)),
        Err(e) => panic!("{}", e),
    }
}

async fn update_item(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<UpdateItem>,
) -> StatusCode {
    let UpdateItem { id, disposaler_name, situation } = payload;
    if let Some(disposaler_name) = disposaler_name {
        match sqlx::query!(r#"UPDATE equips SET disposaler_name = $1 WHERE id = $2"#, disposaler_name, id).execute(&*pool).await {
            Ok(r) if r.rows_affected() != 0 => (), // OK 次のリクエスト
            Ok(_) => return StatusCode::NOT_FOUND,
            Err(_) => return StatusCode::BAD_REQUEST,
        } 
    }
    // if let Some(situation) = situation {
    //     sqlx::query!(r#"UPDATE equips SET situation = $1 WHERE id = $2"#, situation, id).execute(&*pool).await;
    // }
    return StatusCode::OK;
}

async fn delete_item(
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>
) -> StatusCode {
    print!("{}", id);
    match sqlx::query!(r#"DELETE FROM equips WHERE id = $1"#, id).execute(&*pool).await {
        Ok(r) if r.rows_affected() != 0 => return StatusCode::OK,
        Ok(_) => return StatusCode::NOT_FOUND,
        Err(_) => return StatusCode::BAD_REQUEST,
    } 
}

#[derive(Deserialize)]
struct CreateItem {
    applicant_name: String,
    disposaler_name: String,
    equipment_type: String,
    equipment_name: String,
    purchase_url: String,
    cost: i64,
}

#[derive(Deserialize)]
struct UpdateItem {
    id: i64,
    disposaler_name: Option<String>,
    situation: Option<Situation>,
}

#[derive(Serialize, Deserialize)]
struct ItemID {
    id: i64,
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(rename_all = "lowercase")] // SQL の値と対応させる
enum Situation {
    Pending,
    Approved,
    Purchased,
    Equipment,
}


#[derive(Serialize)]
struct EquipData {
    id: i64,
    applicant_name: String,
    disposaler_name: String,
    equipment_type: String,
    equipment_name: String,
    purchase_url: String,
    cost: i64,
    situation: Situation,
    approval_date: Option<String>,
}