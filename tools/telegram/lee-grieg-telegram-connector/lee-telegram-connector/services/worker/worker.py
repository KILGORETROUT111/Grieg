import os, json, time, hashlib
import redis
from dotenv import load_dotenv
from models import ensure_schema, insert_event, insert_claim, engine
from sqlalchemy import text
from rules import extract_claims

load_dotenv()

REDIS_URL = os.environ.get("REDIS_URL", "redis://localhost:6379/0")
QUEUE_KEY = "lee_ingest"

r = redis.Redis.from_url(REDIS_URL, decode_responses=True)

def process_event(conn, ev: dict):
    # Persist event
    event_id = insert_event(conn, ev)
    # Extract simple claims
    for c in extract_claims(ev):
        insert_claim(conn, event_id, ev["message"]["from"]["id"], c["type"], c["payload"])
    conn.commit()

def main():
    ensure_schema()
    print("[worker] ready; waiting for events...")
    while True:
        item = r.brpop(QUEUE_KEY, timeout=5)
        if not item:
            continue
        _key, payload = item
        try:
            ev = json.loads(payload)
        except Exception as e:
            print("[worker] bad payload:", e)
            continue
        with engine.begin() as conn:
            process_event(conn, ev)

if __name__ == "__main__":
    main()
