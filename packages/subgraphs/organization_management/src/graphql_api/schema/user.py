
import strawberry

from graphql_api.schema import Member


@strawberry.federation.type(keys=["id"])
class User(Member):
    pass


@strawberry.input
class Credential:
    type: str
    value: str


@strawberry.input
class AddUserInput:
    username: str
    email: str
    enabled: bool
    credentials: list[Credential]
