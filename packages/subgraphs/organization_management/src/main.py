import os

from dotenv import load_dotenv
from fastapi import FastAPI
from strawberry.fastapi import GraphQLRouter

from graphql_api.schema import schema

load_dotenv()

port_str = os.getenv("ORGANIZATION_MANAGEMENT_PORT")
PORT = int(port_str) if port_str else 8009
HOST = os.getenv("BACKEND_HOST", "0.0.0.0")

graphql_app = GraphQLRouter(schema)

app = FastAPI()

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
