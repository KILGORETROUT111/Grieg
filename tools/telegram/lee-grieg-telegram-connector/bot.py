import os, asyncio, json, tempfile, typing as T
import httpx
from dotenv import load_dotenv
from telegram.ext import Application, MessageHandler, CommandHandler, filters
from telegram import Update

load_dotenv()
BOT = os.environ["TELEGRAM_BOT_TOKEN"]
URL = os.environ.get("LEE_ENDPOINT", "http://localhost:8000/api/v1/evaluate")
JSONL_MODE = os.environ.get("JSONL_MODE","engine").lower()
MAX_LINES = int(os.environ.get("JSONL_MAX_LINES","1000"))

def fmt(d: T.Any) -> str:
    if isinstance(d, dict):
        view = d.get("result") if isinstance(d.get("result"), dict) else d
        parts = []
        for k in ("rc","phase","value","ast","text"):
            if k in view and view[k] is not None: parts.append(f"{k}: {view[k]}")
        if parts: return "\n".join(parts)
    return str(d)

def pick_prompt(obj: T.Any) -> str:
    if isinstance(obj, str): return obj
    if isinstance(obj, dict):
        for k in ("prompt","input","expr","code","text"):
            v = obj.get(k)
            if v is not None: return str(v)
    return str(obj)

async def call_engine(payload: dict) -> str:
    async with httpx.AsyncClient(timeout=60) as cl:
        r = await cl.post(URL, json=payload); r.raise_for_status()
        return fmt(r.json())

async def cmd_start(u: Update, _):
    await u.message.reply_text(
        "GriegEngine bot online.\n\n"
        "Commands:\n"
        "/expr <code> [--mem] [--ast]\n"
        "Upload a *.jsonl* file with caption /jsonl"
    )

async def cmd_expr(u: Update, _):
    args = (u.message.text or "").split(maxsplit=1)
    expr = args[1] if len(args) > 1 else "ping"
    mem = "--mem" in expr; ast = "--ast" in expr
    expr = expr.replace("--mem","").replace("--ast","").strip()
    payload = {"prompt": expr, "mem": mem, "ast": ast}
    try: out = await call_engine(payload)
    except Exception as e: out = f"Engine error: {e}"
    await u.message.reply_text(out[:4000])

async def handle_jsonl(u: Update, ctx):
    doc = u.message.document
    if not doc: return
    # download file
    try:
        fobj = await ctx.bot.get_file(doc.file_id)
        with tempfile.NamedTemporaryFile(delete=False, suffix=".jsonl") as tmp:
            path = tmp.name
        await fobj.download_to_drive(custom_path=path)
    except Exception as e:
        await u.message.reply_text(f"Download error: {e}"); return

    ok=fail=0; previews=[]
    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as fh:
            async with httpx.AsyncClient(timeout=60) as cl:
                for i, line in enumerate(fh, 1):
                    if i > MAX_LINES: break
                    line=line.strip()
                    if not line: continue
                    try: obj=json.loads(line)
                    except json.JSONDecodeError: obj=line
                    payload = {"prompt": pick_prompt(obj)}
                    try:
                        r=await cl.post(URL, json=payload); r.raise_for_status()
                        if len(previews)<6: previews.append(f"{i}: {fmt(r.json())}")
                        ok+=1
                    except Exception as e:
                        if len(previews)<6: previews.append(f"{i}: ERROR {e}")
                        fail+=1
    finally:
        try: os.remove(path)
        except: pass

    msg=[f"jsonl run complete: ok={ok}, fail={fail}"]
    if previews: msg.append("\n".join(previews))
    await u.message.reply_text("\n".join(msg)[:4000])

async def echo(u: Update, _):
    raw=(u.message.text or "").strip()
    if not raw: return
    try: out = await call_engine({"prompt": raw})
    except Exception as e: out=f"Engine error: {e}"
    await u.message.reply_text(out[:4000])

async def main():
    app = Application.builder().token(BOT).build()
    app.add_handler(CommandHandler("start", cmd_start))
    app.add_handler(CommandHandler("expr", cmd_expr))
    app.add_handler(MessageHandler(filters.Document.ALL & filters.CaptionRegex(r"^/jsonl$"), handle_jsonl))
    app.add_handler(MessageHandler(filters.TEXT & ~filters.COMMAND, echo))
    await app.initialize(); await app.start(); await app.updater.start_polling()
    try: await asyncio.Event().wait()
    finally: await app.updater.stop(); await app.stop(); await app.shutdown()

if __name__ == "__main__":
    asyncio.run(main())
