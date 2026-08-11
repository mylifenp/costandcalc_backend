
import strawberry


@strawberry.federation.type(keys=["id"])
class User:
    id: strawberry.ID
    username: str
    email: str


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
