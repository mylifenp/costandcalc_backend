import strawberry


@strawberry.type
class Domain:
    name: str
    verified: bool
