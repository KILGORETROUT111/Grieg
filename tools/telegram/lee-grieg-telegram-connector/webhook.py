import os
from fastapi import FastAPI, Request, HTTPException
import httpx

app = FastAPI()
LEE_ENDPOINT = os.getenv("LEE_ENDPOINT", "http://localhost:8000/api/v1/evaluate")
LEE_API_KEY = os.getenv("LEE_API_KEY", "")
TIMEOUT = float(os.getenv("LEE_TIMEOUT_SECONDS", "30"))

@app.post("/telegram/webhook")
async def telegram_webhook(req: Request):
    data = await req.json()
    message = (data.get("message") or {}).get("text") or ""
    if not message:
        return {"ok": True}
    headers = {}
    if LEE_API_KEY:
        headers["Authorization"] = f"Bearer {LEE_API_KEY}"
    async with httpx.AsyncClient(timeout=TIMEOUT) as client:
        r = await client.post(LEE_ENDPOINT, json={"prompt": message}, headers=headers)
        r.raise_for_status()
        result = r.json()
    return {"ok": True, "result": result}
