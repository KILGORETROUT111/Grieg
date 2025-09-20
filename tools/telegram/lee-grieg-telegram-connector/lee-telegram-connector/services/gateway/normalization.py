import time

def normalize_telegram_update(update: dict) -> dict:
    # Support both message and edited_message minimally
    msg = update.get("message") or update.get("edited_message") or {}
    chat = msg.get("chat", {})
    frm = msg.get("from", {})

    kind = "text"
    text = msg.get("text")
    attachments = []

    if "document" in msg:
        kind = "document"
        doc = msg["document"]
        attachments.append({
            "type": "document",
            "file_id": doc.get("file_id"),
            "file_name": doc.get("file_name"),
            "mime": doc.get("mime_type"),
            "size": doc.get("file_size"),
        })
    elif "photo" in msg:
        kind = "photo"
        sizes = msg["photo"]
        if sizes:
            attachments.append({
                "type": "photo",
                "file_id": sizes[-1].get("file_id"),
                "width": sizes[-1].get("width"),
                "height": sizes[-1].get("height"),
            })
    elif "voice" in msg:
        kind = "voice"
        v = msg["voice"]
        attachments.append({
            "type": "voice",
            "file_id": v.get("file_id"),
            "duration": v.get("duration"),
            "mime": v.get("mime_type")
        })

    event = {
        "platform": "telegram",
        "chat": {
            "id": chat.get("id"),
            "type": chat.get("type"),
            "title": chat.get("title")
        },
        "message": {
            "id": msg.get("message_id"),
            "from": {"id": frm.get("id"), "username": frm.get("username"), "name": f"{frm.get('first_name','')} {frm.get('last_name','')}".strip()},
            "date": msg.get("date"),
            "kind": kind,
            "text": text,
            "reply_to": (msg.get("reply_to_message") or {}).get("message_id"),
            "forward_meta": {
                "from_chat_id": ((msg.get("forward_from_chat") or {}).get("id")) if msg.get("forward_from_chat") else None,
                "from_message_id": (msg.get("forward_from_message_id")) if msg.get("forward_from_message_id") else None
            },
            "attachments": attachments
        }
    }
    # Ensure a timestamp if missing
    if not event["message"]["date"]:
        event["message"]["date"] = int(time.time())
    return event
