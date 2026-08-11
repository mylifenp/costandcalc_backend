import asyncio
import os

from dotenv import load_dotenv
from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse
from strawberry.fastapi import GraphQLRouter

from schema import schema

load_dotenv()

port_str = os.getenv("ORGANIZATION_MANAGEMENT_PORT")
PORT = int(port_str) if port_str else 8009
HOST = os.getenv("BACKEND_HOST", "0.0.0.0")

graphql_app = GraphQLRouter(schema)

app = FastAPI()



app = FastAPI()

@app.middleware("http")
async def timeout_middleware(request: Request, call_next):
    try:
        return await asyncio.wait_for(call_next(request), timeout=5.0)
    except asyncio.TimeoutError:
        return JSONResponse(
            {"detail": "Request timed out"},
            status_code=504,
        )

app.include_router(
    graphql_app,
    prefix="/graphql",
)


# ---------------------------------------------------------------------------
# Development server
# ---------------------------------------------------------------------------

if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "main:app",
        host=HOST,
        port=PORT,
        reload=True,
    )
