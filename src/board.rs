use std::{
    collections::HashMap,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use apalis::prelude::*;
use apalis_redis::{ConnectionManager, RedisStorage};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use redis::AsyncCommands;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::queue::{ChatJob, QUEUE_NAME};

type ApiError = (StatusCode, String);

#[derive(Clone)]
pub struct BoardState {
    pub storage: RedisStorage<ChatJob, ConnectionManager>,
    pub redis: ConnectionManager,
}

pub fn router(state: BoardState) -> Router {
    Router::new()
        .route("/overview", get(overview))
        .route("/queues", get(queues))
        .route("/tasks", get(all_tasks))
        .route("/workers", get(all_workers))
        .route("/queues/{queue}/tasks", get(queue_tasks).put(push_task))
        .route("/queues/{queue}/stats", get(queue_stats))
        .route("/queues/{queue}/workers", get(queue_workers))
        .route("/queues/{queue}/tasks/{task_id}", get(task_by_id))
        .with_state(state)
}

#[derive(Deserialize, Default)]
struct TaskQuery {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    page: Option<u32>,
    #[serde(default)]
    page_size: Option<u32>,
}

#[derive(Deserialize)]
struct TaskInput {
    args: ChatJob,
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn api_error(error: impl std::fmt::Display) -> ApiError {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}

fn ensure_queue(queue: &str) -> Result<(), ApiError> {
    if queue == QUEUE_NAME {
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, format!("queue '{queue}' tidak dikenal")))
    }
}

fn statistic(title: &str, stat_type: &str, value: impl ToString, priority: u64) -> Value {
    json!({
        "title": title,
        "stat_type": stat_type,
        "value": value.to_string(),
        "priority": priority,
    })
}

async fn done_timestamps(conn: &mut ConnectionManager, queue: &str, set: &str) -> Vec<u64> {
    let ids: Vec<String> = conn
        .smembers(format!("{queue}:{set}"))
        .await
        .unwrap_or_default();
    let mut timestamps = Vec::with_capacity(ids.len());

    for id in ids {
        let value: Option<String> = conn
            .hget(format!("{queue}:meta:{id}"), "done_at")
            .await
            .unwrap_or(None);
        if let Some(timestamp) = value.and_then(|v| v.parse::<u64>().ok()) {
            timestamps.push(timestamp);
        }
    }

    timestamps
}

async fn queue_metrics(
    conn: &mut ConnectionManager,
    queue: &str,
) -> redis::RedisResult<(Vec<Value>, Vec<usize>)> {
    let now = now_secs();
    let active: u64 = conn.llen(format!("{queue}:active")).await?;
    let done: u64 = conn.scard(format!("{queue}:done")).await?;
    let failed: u64 = conn.scard(format!("{queue}:dead")).await?;
    let workers: Vec<String> = conn.zrange(format!("{queue}:workers"), 0, -1).await?;

    let mut running = 0u64;
    for worker in &workers {
        let count: u64 = conn
            .scard(format!("{queue}:inflight:{worker}"))
            .await
            .unwrap_or(0);
        running += count;
    }

    let done_ts = done_timestamps(conn, queue, "done").await;
    let failed_ts = done_timestamps(conn, queue, "dead").await;

    let hour_ago = now.saturating_sub(3600);
    let day_ago = now.saturating_sub(86_400);
    let week_ago = now.saturating_sub(7 * 86_400);

    let done_1h = done_ts.iter().filter(|ts| **ts >= hour_ago).count() as u64;
    let done_24h = done_ts.iter().filter(|ts| **ts >= day_ago).count() as u64;
    let failed_1h = failed_ts.iter().filter(|ts| **ts >= hour_ago).count() as u64;
    let failed_24h = failed_ts.iter().filter(|ts| **ts >= day_ago).count() as u64;
    let failed_7d = failed_ts.iter().filter(|ts| **ts >= week_ago).count() as u64;

    let total = active + done + failed + running;
    let total_1h = done_1h + failed_1h;
    let total_24h = done_24h + failed_24h;

    let (success_rate, failure_rate) = if done + failed > 0 {
        (
            done as f64 / (done + failed) as f64 * 100.0,
            failed as f64 / (done + failed) as f64 * 100.0,
        )
    } else {
        (0.0, 0.0)
    };

    let success_rate_24h = if total_24h > 0 {
        done_24h as f64 / total_24h as f64 * 100.0
    } else {
        0.0
    };

    let most_recent = done_ts
        .iter()
        .chain(failed_ts.iter())
        .max()
        .copied()
        .unwrap_or(0);

    let mut activity = vec![0usize; 7];
    for timestamp in &done_ts {
        if *timestamp <= now {
            let age_days = ((now - *timestamp) / 86_400) as usize;
            if age_days < 7 {
                activity[6 - age_days] += 1;
            }
        }
    }

    let stats = vec![
        statistic("RUNNING_JOBS", "Number", running, 1),
        statistic("PENDING_JOBS", "Number", active, 2),
        statistic("FAILED_JOBS", "Number", failed, 3),
        statistic("DONE_JOBS", "Number", done, 4),
        statistic("TOTAL_JOBS", "Number", total, 5),
        statistic("SUCCESS_RATE", "Percentage", format!("{success_rate:.2}"), 6),
        statistic("FAILURE_RATE", "Percentage", format!("{failure_rate:.2}"), 7),
        statistic("MOST_RECENT_JOB", "Timestamp", most_recent, 8),
        statistic("JOBS_PAST_HOUR", "Number", total_1h, 9),
        statistic("JOBS_PAST_24_HOURS", "Number", total_24h, 10),
        statistic("FAILED_JOBS_PAST_7_DAYS", "Number", failed_7d, 11),
        statistic(
            "SUCCESS_RATE_PAST_24H",
            "Percentage",
            format!("{success_rate_24h:.2}"),
            12,
        ),
        statistic(
            "AVG_JOBS_PER_HOUR_PAST_24H",
            "Decimal",
            format!("{:.2}", total_24h as f64 / 24.0),
            13,
        ),
    ];

    Ok((stats, activity))
}

async fn fetch_workers(
    conn: &mut ConnectionManager,
    queue: &str,
) -> redis::RedisResult<Vec<Value>> {
    let workers_key = format!("{queue}:workers");
    let entries: Vec<(String, f64)> = conn.zrange_withscores(&workers_key, 0, -1).await?;
    let mut workers = Vec::with_capacity(entries.len());

    for (name, last_heartbeat) in entries {
        let metadata: HashMap<String, String> = conn
            .hgetall(format!("{queue}:workers:metadata:{name}"))
            .await
            .unwrap_or_default();

        workers.push(json!({
            "id": name,
            "queue": workers_key,
            "backend": metadata.get("backend_name").cloned().unwrap_or_default(),
            "started_at": 0,
            "last_heartbeat": last_heartbeat as u64,
            "layers": metadata.get("service").cloned().unwrap_or_default(),
        }));
    }

    Ok(workers)
}

fn task_json(task: &Task<Vec<u8>>, decode_args: bool) -> Value {
    let args = if decode_args {
        serde_json::from_slice::<Value>(&task.args).unwrap_or_else(|_| json!(&task.args))
    } else {
        json!(&task.args)
    };

    json!({
        "args": args,
        "ctx": task.ctx().as_ref(),
    })
}

async fn load_tasks(
    state: &BoardState,
    query: &TaskQuery,
    decode_args: bool,
) -> Result<Vec<Value>, ApiError> {
    let filter = Filter {
        status: None,
        page: 1,
        page_size: Some(10_000),
    };
    let mut tasks = state
        .storage
        .list_all_tasks(&filter)
        .await
        .map_err(api_error)?;

    if let Some(status) = &query.status {
        tasks.retain(|task| task.status().to_string() == *status);
    }

    tasks.sort_by_key(|task| {
        std::cmp::Reverse(task.task_id().map(|id| id.to_string()).unwrap_or_default())
    });

    let page = query.page.unwrap_or(1).max(1) as usize;
    let page_size = query.page_size.unwrap_or(10).clamp(1, 500) as usize;
    let start = (page - 1) * page_size;

    Ok(tasks
        .iter()
        .skip(start)
        .take(page_size)
        .map(|task| task_json(task, decode_args))
        .collect())
}

async fn overview(State(state): State<BoardState>) -> Result<Json<Vec<Value>>, ApiError> {
    let mut conn = state.redis.clone();
    let (stats, _) = queue_metrics(&mut conn, QUEUE_NAME).await.map_err(api_error)?;
    Ok(Json(stats))
}

async fn queues(State(state): State<BoardState>) -> Result<Json<Vec<Value>>, ApiError> {
    let mut conn = state.redis.clone();
    let (stats, activity) = queue_metrics(&mut conn, QUEUE_NAME).await.map_err(api_error)?;
    let workers: Vec<String> = conn
        .zrange(format!("{QUEUE_NAME}:workers"), 0, -1)
        .await
        .map_err(api_error)?;

    Ok(Json(vec![json!({
        "name": QUEUE_NAME,
        "stats": stats,
        "workers": workers,
        "activity": activity,
    })]))
}

async fn all_tasks(
    State(state): State<BoardState>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<Vec<Value>>, ApiError> {
    Ok(Json(load_tasks(&state, &query, false).await?))
}

async fn queue_tasks(
    Path(queue): Path<String>,
    State(state): State<BoardState>,
    Query(query): Query<TaskQuery>,
) -> Result<Json<Vec<Value>>, ApiError> {
    ensure_queue(&queue)?;
    Ok(Json(load_tasks(&state, &query, true).await?))
}

async fn queue_stats(
    Path(queue): Path<String>,
    State(state): State<BoardState>,
) -> Result<Json<Vec<Value>>, ApiError> {
    ensure_queue(&queue)?;
    let mut conn = state.redis.clone();
    let (stats, _) = queue_metrics(&mut conn, &queue).await.map_err(api_error)?;
    Ok(Json(stats))
}

async fn all_workers(State(state): State<BoardState>) -> Result<Json<Vec<Value>>, ApiError> {
    let mut conn = state.redis.clone();
    let workers = fetch_workers(&mut conn, QUEUE_NAME).await.map_err(api_error)?;
    Ok(Json(workers))
}

async fn queue_workers(
    Path(queue): Path<String>,
    State(state): State<BoardState>,
) -> Result<Json<Vec<Value>>, ApiError> {
    ensure_queue(&queue)?;
    let mut conn = state.redis.clone();
    let workers = fetch_workers(&mut conn, &queue).await.map_err(api_error)?;
    Ok(Json(workers))
}

async fn task_by_id(
    Path((queue, task_id)): Path<(String, String)>,
    State(state): State<BoardState>,
) -> Result<Json<Value>, ApiError> {
    ensure_queue(&queue)?;

    let task_id = TaskId::from_str(&task_id)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let mut storage = state.storage.clone();

    match storage.fetch_by_id(&task_id).await.map_err(api_error)? {
        Some(task) => Ok(Json(task_json(&task, true))),
        None => Err((StatusCode::NOT_FOUND, "task tidak ditemukan".to_string())),
    }
}

async fn push_task(
    Path(queue): Path<String>,
    State(state): State<BoardState>,
    Json(input): Json<TaskInput>,
) -> Result<Json<Value>, ApiError> {
    ensure_queue(&queue)?;
    let mut storage = state.storage.clone();
    storage.push(input.args).await.map_err(api_error)?;
    Ok(Json(Value::Null))
}
