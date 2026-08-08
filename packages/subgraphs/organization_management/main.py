import os

import strawberry
from dotenv import load_dotenv
from fastapi import FastAPI, Request
from strawberry.fastapi import GraphQLRouter

load_dotenv()

port_str = os.getenv("ORGANIZATION_MANAGEMENT_PORT")
PORT = int(port_str) if port_str else 8009

HOST = os.getenv("BACKEND_HOST", "0.0.0.0")


# ---------------------------------------------------------------------------
# GraphQL Types
# ---------------------------------------------------------------------------

@strawberry.federation.type(keys=["id"])
class User:
    id: strawberry.ID
    name: str


@strawberry.type
class Query:

    @strawberry.field
    async def user(self) -> User:
        return User(
            id=strawberry.ID("1"),
            name="Patrick",
        )

    @strawberry.field
    async def hello(self, info: strawberry.Info) -> str:
        """
        Temporary debugging endpoint.

        NextAuth sends:
            Access-Token: <Keycloak access token>
            ID-Token: <Keycloak ID token>
        """

        # Strawberry's default FastAPI context contains the request.
        request: Request = info.context["request"]

        access_token = request.headers.get("Access-Token")
        id_token = request.headers.get("ID-Token")

        if access_token is None:
            return "ERROR: Access-Token header is missing"

        if id_token is None:
            return "ERROR: ID-Token header is missing"

        # DEBUG ONLY.
        # Do not return tokens from a real production GraphQL API.
        return (
            "Access-Token received: "
            + access_token[:20]
            + "...\n"
            "ID-Token received: "
            + id_token[:20]
            + "..."
        )


# ---------------------------------------------------------------------------
# Strawberry Schema
# ---------------------------------------------------------------------------

schema = strawberry.federation.Schema(
    query=Query,
    federation_version="2.5",
)


# ---------------------------------------------------------------------------
# FastAPI
# ---------------------------------------------------------------------------

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
