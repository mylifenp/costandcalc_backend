import strawberry


@strawberry.federation.type(keys=["id"])
class User:
    id: strawberry.ID
    name: str
    email: str


@strawberry.input
class AddUserInput:
    name: str
    email: str
