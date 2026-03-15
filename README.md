<<<<<<< HEAD
# IVR Builder

سرویس REST API برای ساخت فلوهای IVR — پروژه مستقل که از دیتابیس SQLite مشترک با PBX استفاده می‌کند.

## ساختار پروژه

```
ivr-builder/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs         ← راه‌اندازی سرور و routing
    ├── error.rs        ← ApiError و ApiResult
    ├── models.rs       ← ساختارهای داده + DTO ها
    ├── db.rs           ← init schema (جداول IVR)
    └── api/
        ├── mod.rs
        ├── flows.rs    ← CRUD برای فلوها
        ├── nodes.rs    ← CRUD برای گره‌ها
        └── branches.rs ← CRUD برای branch ها
```

## متغیرهای محیطی

```env
DATABASE_PATH=../sip-pbx-server/data/sip_users.db   # مسیر SQLite مشترک
IVR_BIND_ADDR=0.0.0.0:8090                           # آدرس API
```

## API Reference

### Flows

| Method | Path | توضیح |
|--------|------|-------|
| GET | `/api/flows` | لیست همه فلوها |
| POST | `/api/flows` | ایجاد فلو جدید |
| GET | `/api/flows/:id` | دریافت فلو کامل (با node و branch) |
| PUT | `/api/flows/:id` | بروزرسانی نام/توضیحات |
| DELETE | `/api/flows/:id` | حذف فلو (cascade) |
| PATCH | `/api/flows/:id/entry` | تعیین ورودی اولیه |

### Nodes

| Method | Path | توضیح |
|--------|------|-------|
| POST | `/api/flows/:id/nodes` | ایجاد گره جدید |
| GET | `/api/nodes/:id` | دریافت گره (با branch ها) |
| PUT | `/api/nodes/:id` | بروزرسانی گره |
| DELETE | `/api/nodes/:id` | حذف گره |

### Branches

| Method | Path | توضیح |
|--------|------|-------|
| POST | `/api/nodes/:id/branches` | اضافه کردن branch |
| PUT | `/api/branches/:id` | بروزرسانی branch |
| DELETE | `/api/branches/:id` | حذف branch |

## انواع گره (node_type)

### `menu`
```json
{
  "node_type": "menu",
  "label": "منوی اصلی",
  "config": {
    "audio_file": "sounds/welcome.wav",
    "timeout_secs": 5,
    "retries": 3
  }
}
```
برای این نوع از branch ها با digit استفاده می‌شود.

### `play_audio`
```json
{
  "node_type": "play_audio",
  "config": {
    "audio_file": "sounds/goodbye.wav",
    "next_node_id": "uuid-یا-null"
  }
}
```

### `record_audio`
```json
{
  "node_type": "record_audio",
  "config": {
    "max_duration_secs": 30,
    "save_path": "ivr_records/{call_id}_{timestamp}.wav",
    "next_node_id": "uuid-یا-null"
  }
}
```

### `receive_digits`
```json
{
  "node_type": "receive_digits",
  "config": {
    "prompt_audio": "sounds/enter_code.wav",
    "min_digits": 4,
    "max_digits": 10,
    "terminator": "#",
    "timeout_secs": 10,
    "next_node_id": "uuid"
  }
}
```

### `connect_call`
```json
{
  "node_type": "connect_call",
  "config": {
    "account": "soroush",
    "timeout_secs": 30,
    "no_answer_node_id": "uuid-یا-null"
  }
}
```

### `hangup`
```json
{
  "node_type": "hangup",
  "config": {
    "audio_file": "sounds/busy.wav"
  }
}
```

## Digit های مجاز در branch

`0` `1` `2` `3` `4` `5` `6` `7` `8` `9` `*` `#` `timeout` `invalid`

## مثال: ساخت یک IVR کامل

```bash
BASE=http://localhost:8090

# 1. ایجاد فلو
FLOW=$(curl -s -X POST $BASE/api/flows \
  -H "Content-Type: application/json" \
  -d '{"name":"منوی اصلی","description":"IVR خوشامدگویی"}')
FLOW_ID=$(echo $FLOW | jq -r .id)

# 2. گره منوی اصلی
MENU=$(curl -s -X POST $BASE/api/flows/$FLOW_ID/nodes \
  -H "Content-Type: application/json" \
  -d '{
    "node_type":"menu","label":"منوی اصلی",
    "config":{"audio_file":"welcome.wav","timeout_secs":5,"retries":3}
  }')
MENU_ID=$(echo $MENU | jq -r .id)

# 3. گره اتصال به سروش
NODE_SOROUSH=$(curl -s -X POST $BASE/api/flows/$FLOW_ID/nodes \
  -H "Content-Type: application/json" \
  -d '{
    "node_type":"connect_call","label":"اتصال به سروش",
    "config":{"account":"soroush","timeout_secs":30}
  }')
SOROUSH_ID=$(echo $NODE_SOROUSH | jq -r .id)

# 4. گره خداحافظی
NODE_BYE=$(curl -s -X POST $BASE/api/flows/$FLOW_ID/nodes \
  -H "Content-Type: application/json" \
  -d '{"node_type":"hangup","label":"خداحافظی","config":{"audio_file":"bye.wav"}}')
BYE_ID=$(echo $NODE_BYE | jq -r .id)

# 5. branch ها
curl -s -X POST $BASE/api/nodes/$MENU_ID/branches \
  -H "Content-Type: application/json" \
  -d "{\"digit\":\"2\",\"next_node_id\":\"$SOROUSH_ID\",\"label\":\"اتصال به سروش\"}"

curl -s -X POST $BASE/api/nodes/$MENU_ID/branches \
  -H "Content-Type: application/json" \
  -d "{\"digit\":\"9\",\"next_node_id\":\"$BYE_ID\",\"label\":\"خروج\"}"

# 6. تنظیم ورودی فلو
curl -s -X PATCH $BASE/api/flows/$FLOW_ID/entry \
  -H "Content-Type: application/json" \
  -d "{\"node_id\":\"$MENU_ID\"}"

# 7. مشاهده فلو کامل
curl -s $BASE/api/flows/$FLOW_ID | jq .
```

## اجرا

```bash
DATABASE_PATH=../sip-pbx-server/data/sip_users.db \
IVR_BIND_ADDR=0.0.0.0:8090 \
cargo run
```
# flow-ivr-api
Rust Pbx
>>>>>>> 1b60624adf6ee3dce20260ace33af7cfcaf6387c
