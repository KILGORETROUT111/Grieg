import os, asyncio
from typing import Final
import httpx
from pydantic import BaseModel
from telegram import Update
from telegram.ext import Application, CommandHandler, MessageHandler, filters, ContextTypes

BOT_TOKEN: Final = os.getenv("TELEGRAM_BOT_TOKEN", "")
LEE_ENDPOINT: Final = os.getenv("LEE_ENDPOINT", "http://localhost:8000/api/v1/evaluate")
LEE_API_KEY: Final = os.getenv("LEE_API_KEY", "")
TIMEOUT = float(os.getenv("LEE_TIMEOUT_SECONDS", "30"))

class EvalRequest(BaseModel):
    prompt: str

async def call_lee(prompt: str) -> str:
    headers = {}
    if LEE_API_KEY:
        headers["Authorization"] = f"Bearer {LEE_API_KEY}"
    async with httpx.AsyncClient(timeout=TIMEOUT) as client:
        r = await client.post(LEE_ENDPOINT, json={"prompt": prompt}, headers=headers)
        r.raise_for_status()
        data = r.json()
        # Accept common shapes: {"text": "..."} or {"result": {"text": "..."}} or {"output": "..."}
        if isinstance(data, dict):
            if "text" in data:
                return str(data["text"])
            if "result" in data and isinstance(data["result"], dict) and "text" in data["result"]:
                return str(data["result"]["text"])
            if "output" in data:
                return str(data["output"])
        return str(data)

async def start(update: Update, context: ContextTypes.DEFAULT_TYPE) -> None:
    await update.message.reply_text("Grieg/LEE connector ready. Send me a prompt.")

async def handle(update: Update, context: ContextTypes.DEFAULT_TYPE) -> None:
    if not update.message or not update.message.text:
        return
    prompt = update.message.text
    try:
        result = await call_lee(prompt)
    except Exception as e:
        result = f"Engine error: {e}"
    await update.message.reply_text(result[:4000])

async def main():
    if not BOT_TOKEN:
        raise SystemExit("TELEGRAM_BOT_TOKEN is not set")
    app = Application.builder().token(BOT_TOKEN).build()
    app.add_handler(CommandHandler("start", start))
    app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, handle))
    await app.initialize()
    await app.start()
    await app.updater.start_polling()
    try:
        await asyncio.Event().wait()
    finally:
        await app.updater.stop()
        await app.stop()
        await app.shutdown()

if __name__ == "__main__":
    asyncio.run(main())
