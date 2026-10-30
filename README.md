# taskr

A small task-list REST API built with [Axum](https://github.com/tokio-rs/axum) and Tokio. Tasks are kept in memory, so all data is lost when the server stops.

## Requirements

- Rust toolchain with edition 2024 support (Rust 1.85+)

## Getting started

```sh
cargo run
```

The server listens on `http://localhost:3000` (bound to `0.0.0.0:3000`). The port is currently hard-coded in `src/main.rs`.

On startup the store is seeded with three example tasks.

## API

| Method   | Path          | Description                  | Success response        |
|----------|---------------|------------------------------|-------------------------|
| `GET`    | `/health`     | Health check                 | `200` with body `OK`    |
| `GET`    | `/tasks`      | List all tasks               | `200` with JSON array   |
| `POST`   | `/tasks`      | Create a task                | `201` with created task |
| `GET`    | `/tasks/{id}` | Get a single task            | `200` with task         |
| `PATCH`  | `/tasks/{id}` | Update title and/or done     | `200` with updated task |
| `DELETE` | `/tasks/{id}` | Delete a task                | `204` no content        |

### Task object

```json
{ "id": 1, "title": "Go shopping", "done": false }
```

### Request bodies

**`POST /tasks`**

```json
{ "title": "Buy milk" }
```

**`PATCH /tasks/{id}`** — both fields are optional; only the fields provided are changed.

```json
{ "title": "Buy oat milk", "done": true }
```

### Errors

`GET`, `PATCH` and `DELETE` on `/tasks/{id}` return `404 Not Found` when no task has that id. Malformed JSON or a non-numeric id is rejected by Axum with a `4xx` response.

## Examples

```sh
# list tasks
curl http://localhost:3000/tasks

# create a task
curl -X POST http://localhost:3000/tasks \
  -H 'Content-Type: application/json' \
  -d '{"title": "Buy milk"}'

# mark task 1 as done
curl -X PATCH http://localhost:3000/tasks/1 \
  -H 'Content-Type: application/json' \
  -d '{"done": true}'

# delete task 2
curl -X DELETE http://localhost:3000/tasks/2
```

## Project layout

```
src/
├── main.rs            # entry point: builds state and starts the server
├── routes.rs          # route table
├── state.rs           # shared in-memory state (Arc<Mutex<..>>)
├── models.rs          # Task and request body types
└── handlers/
    └── task.rs        # request handlers
```

## Limitations

- Storage is in-memory only; there is no persistence.
- No authentication.
- Ids are assigned sequentially and never reused.
