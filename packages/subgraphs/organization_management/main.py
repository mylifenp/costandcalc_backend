import os

import strawberry
from dotenv import load_dotenv
from fastapi import FastAPI
from strawberry.fastapi import GraphQLRouter

load_dotenv()

PORT = int(os.getenv("ORGANIZATION_MANAGENEBT_PORT"))
HOST = os.getenv("BACKEND_HOST", "0.0.0.0")

@strawberry.federation.type(keys=["id"])
class User:
    id: strawberry.ID
    name: str

@strawberry.type
class Query:
    @strawberry.field
    def user(self) -> User:
        return User(id=strawberry.ID("1"), name="Patrick")

schema = strawberry.federation.Schema(query=Query, federation_version="2.5")

graphql_app = GraphQLRouter(schema)

app = FastAPI()

app.include_router(
    graphql_app,
    prefix="/graphql",
)

if __name__ == "__main__":
    import uvicorn
    uvicorn.run("main:app", host=HOST, port=PORT, reload=True)
