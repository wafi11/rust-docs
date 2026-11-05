use apalis::prelude::TaskSink;
use apalis_redis::{ConnectionManager, RedisStorage};
use axum::{extract::State, http::StatusCode, response::Html, Json};
use serde::Deserialize;

use crate::queue::ChatJob;

#[derive(Clone)]
pub struct ChatState {
    pub storage: RedisStorage<ChatJob, ConnectionManager>,
}

#[derive(Deserialize)]
pub struct NewMessage {
    pub name: Option<String>,
    pub text: String,
}

pub async fn page() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn send(
    State(state): State<ChatState>,
    Json(payload): Json<NewMessage>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let text = payload.text.trim().to_string();
    if text.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "pesan tidak boleh kosong".to_string()));
    }

    let mut storage = state.storage.clone();
    storage
        .push(ChatJob {
            name: payload.name.unwrap_or_else(|| "kamu".to_string()),
            desc: text,
            is_reply: false,
        })
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "ok": true })))
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="id">
<head>
<meta charset="UTF-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>Apalis Chat Test</title>
<style>
  * { box-sizing: border-box; }
  body {
    margin: 0; min-height: 100vh; font-family: ui-sans-serif, system-ui, sans-serif;
    background: #0f172a; color: #e2e8f0; display: flex; flex-direction: column;
  }
  header {
    padding: 14px 20px; background: #1e293b; border-bottom: 1px solid #334155;
    display: flex; align-items: center; justify-content: space-between;
  }
  header h1 { font-size: 15px; margin: 0; font-weight: 600; }
  header a { color: #38bdf8; font-size: 13px; text-decoration: none; }
  #messages {
    flex: 1; overflow-y: auto; padding: 20px; display: flex; flex-direction: column; gap: 10px;
    max-width: 780px; width: 100%; margin: 0 auto;
  }
  .row { display: flex; }
  .row.right { justify-content: flex-end; }
  .bubble {
    max-width: 78%; padding: 9px 12px; border-radius: 14px; font-size: 14px; line-height: 1.4;
  }
  .bubble.user { background: #0284c7; border-bottom-right-radius: 4px; }
  .bubble.bot { background: #1e293b; border: 1px solid #334155; border-bottom-left-radius: 4px; }
  .who { font-size: 11px; opacity: .7; margin-bottom: 2px; }
  .text { white-space: pre-wrap; word-break: break-word; }
  .status { font-size: 10px; margin-top: 4px; opacity: .6; text-align: right; }
  .status.done { color: #4ade80; opacity: .9; }
  .status.failed { color: #f87171; opacity: .9; }
  form {
    display: flex; gap: 8px; padding: 14px 20px; background: #1e293b;
    border-top: 1px solid #334155;
  }
  form .inner {
    display: flex; gap: 8px; max-width: 780px; width: 100%; margin: 0 auto;
  }
  #name { width: 110px; }
  input {
    background: #0f172a; color: #e2e8f0; border: 1px solid #334155;
    border-radius: 8px; padding: 10px 12px; font-size: 14px; outline: none;
  }
  input:focus { border-color: #38bdf8; }
  #text { flex: 1; }
  button {
    background: #0284c7; color: white; border: 0; border-radius: 8px;
    padding: 0 18px; font-size: 14px; cursor: pointer;
  }
  button:hover { background: #0369a1; }
  #error { color: #f87171; font-size: 12px; padding: 0 20px 8px; text-align: center; display: none; }
  .empty { text-align: center; opacity: .5; font-size: 13px; margin-top: 40px; }
</style>
</head>
<body>
<header>
  <h1>Apalis Chat Test</h1>
  <a href="/" target="_blank">Buka Apalis Board</a>
</header>
<div id="messages"></div>
<div id="error"></div>
<form id="form">
  <div class="inner">
    <input id="name" placeholder="nama" value="kamu" />
    <input id="text" placeholder="Tulis pesan..." autocomplete="off" />
    <button type="submit">Kirim</button>
  </div>
</form>
<script>
const list = document.getElementById('messages');
const form = document.getElementById('form');
const textInput = document.getElementById('text');
const nameInput = document.getElementById('name');
const errorBox = document.getElementById('error');

const labels = {
  Pending: 'menunggu',
  Running: 'diproses',
  Done: 'selesai',
  Failed: 'gagal',
  Retry: 'retry',
};

function showError(msg) {
  errorBox.textContent = msg;
  errorBox.style.display = 'block';
  setTimeout(() => { errorBox.style.display = 'none'; }, 4000);
}

async function load() {
  const res = await fetch('/api/v1/tasks?queue=chat');
  if (!res.ok) throw new Error('HTTP ' + res.status);
  const tasks = await res.json();
  tasks.sort((a, b) => (a.ctx.task_id > b.ctx.task_id ? 1 : -1));

  list.innerHTML = '';
  if (tasks.length === 0) {
    list.innerHTML = '<div class="empty">Belum ada pesan. Kirim pesan pertama.</div>';
    return;
  }

  for (const task of tasks) {
    let job;
    try {
      job = JSON.parse(new TextDecoder().decode(new Uint8Array(task.args)));
    } catch (e) {
      continue;
    }
    const mine = !job.is_reply;
    const row = document.createElement('div');
    row.className = 'row ' + (mine ? 'right' : 'left');

    const bubble = document.createElement('div');
    bubble.className = 'bubble ' + (mine ? 'user' : 'bot');

    const who = document.createElement('div');
    who.className = 'who';
    who.textContent = job.name || (mine ? 'kamu' : 'bot');

    const text = document.createElement('div');
    text.className = 'text';
    text.textContent = job.desc;

    const status = document.createElement('div');
    const raw = task.ctx.status || '';
    status.className = 'status ' + raw.toLowerCase();
    status.textContent = labels[raw] || raw.toLowerCase();

    bubble.append(who, text, status);
    row.append(bubble);
    list.append(row);
  }

  list.scrollTop = list.scrollHeight;
}

form.addEventListener('submit', async (event) => {
  event.preventDefault();
  const text = textInput.value.trim();
  if (!text) return;
  const name = nameInput.value.trim() || 'kamu';
  textInput.value = '';

  try {
    const res = await fetch('/chat/messages', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ name, text }),
    });
    if (!res.ok) throw new Error(await res.text());
    await load();
  } catch (err) {
    showError('Gagal mengirim: ' + err.message);
  }
});

load().catch((e) => showError('Gagal memuat pesan: ' + e.message));
setInterval(() => load().catch(() => {}), 1500);
</script>
</body>
</html>
"#;
