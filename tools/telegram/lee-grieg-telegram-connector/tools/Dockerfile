FROM python:3.11-slim
WORKDIR /app
COPY . /app
RUN pip install --no-cache-dir -U pip &&             pip install --no-cache-dir python-telegram-bot==21.* httpx==0.27.* pydantic==2.* uvicorn==0.30.* fastapi==0.112.*
ENV PORT=8080 HOST=0.0.0.0
CMD ["python","tools/run_polling.py"]
