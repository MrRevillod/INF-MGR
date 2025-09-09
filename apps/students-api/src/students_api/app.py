from fastapi import FastAPI

app = FastAPI(
    title="Students API",
    description="API for managing student",
    version="1.0.0"
)

@app.get("/")
async def root():
    return {"message": "Students API running"}

@app.get("/health")
async def health():
    return {"status": "ok"}