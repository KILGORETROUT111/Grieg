import os
from sqlalchemy import create_engine, text, Table, Column, Integer, String, JSON, MetaData, BigInteger
from sqlalchemy.dialects.postgresql import JSONB
from sqlalchemy.exc import OperationalError

DATABASE_URL = os.environ.get("DATABASE_URL", "postgresql+psycopg2://postgres:postgres@localhost:5432/lee")
engine = create_engine(DATABASE_URL, future=True)
md = MetaData()

events = Table("events", md,
    Column("id", Integer, primary_key=True, autoincrement=True),
    Column("platform", String, nullable=False),
    Column("chat_id", String, nullable=False),
    Column("message_id", String, nullable=False),
    Column("sender_id", String, nullable=False),
    Column("sender_username", String),
    Column("sender_name", String),
    Column("ts", BigInteger, nullable=False),
    Column("kind", String, nullable=False),
    Column("text", String),
    Column("attachments", JSONB),
    Column("raw_sig", String)
)

claims = Table("claims", md,
    Column("id", Integer, primary_key=True, autoincrement=True),
    Column("event_id", Integer, nullable=False),
    Column("actor_id", String, nullable=False),
    Column("type", String, nullable=False),  # commitment|contradiction|support|temporal
    Column("payload", JSONB)
)

def ensure_schema():
    try:
        md.create_all(engine)
    except OperationalError as e:
        raise

def insert_event(conn, ev):
    ins = events.insert().values(
        platform=ev["platform"],
        chat_id=str(ev["chat"]["id"]),
        message_id=str(ev["message"]["id"]),
        sender_id=str(ev["message"]["from"]["id"]),
        sender_username=ev["message"]["from"].get("username"),
        sender_name=ev["message"]["from"].get("name"),
        ts=int(ev["message"]["date"]),
        kind=ev["message"]["kind"],
        text=ev["message"].get("text"),
        attachments=ev["message"].get("attachments", []),
        raw_sig=ev.get("raw_sig")
    ).returning(events.c.id)
    res = conn.execute(ins)
    return res.scalar_one()

def insert_claim(conn, event_id, actor_id, ctype, payload):
    conn.execute(claims.insert().values(
        event_id=event_id,
        actor_id=str(actor_id),
        type=ctype,
        payload=payload
    ))
