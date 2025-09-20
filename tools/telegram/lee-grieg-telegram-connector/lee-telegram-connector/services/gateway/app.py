import os, hashlib, json, time
from fastapi import FastAPI, Request, Response
from fastapi.responses import JSONResponse
from pydantic import BaseModel
import redis
from normalization import normalize_telegram_update
from dotenv import load_dotenv
from fastapi.middleware.cors import CORSMiddleware

load_dotenv()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379/0")
r = redis.Redis.from_url(REDIS_URL, decode_responses=True)

ALLOW_ORIGINS = [o.strip() for o in os.environ.get("ALLOW_ORIGINS","*").split(",")]

app = FastAPI(title="LEE Telegram Gateway")
app.add_middleware(
    CORSMiddleware,
    allow_origins=ALLOW_ORIGINS if ALLOW_ORIGINS != ["*"] else ["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

QUEUE_KEY = "lee_ingest"

@app.get("/health")
async def health():
    return {"ok": True, "time": time.time()}

@app.post("/ingest/tg")
async def ingest_tg(request: Request):
    raw = await request.body()
    try:
        data = json.loads(raw.decode("utf-8"))
    except Exception:
        return JSONResponse({"ok": False, "error": "invalid json"}, status_code=400)

    event = normalize_telegram_update(data)
    # raw signature for audit
    m = hashlib.sha256()
    m.update(raw)
    event["raw_sig"] = m.hexdigest()

    # Enqueue
    r.lpush(QUEUE_KEY, json.dumps(event))
    return {"ok": True}

@app.get("/debug/peek")
async def peek():
    # Inspect last normalized event (for testing)
    val = r.lindex(QUEUE_KEY, 0)
    return {"last": json.loads(val) if val else None}
