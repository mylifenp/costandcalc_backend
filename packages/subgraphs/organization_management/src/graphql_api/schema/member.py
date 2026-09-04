
import strawberry


@strawberry.type
class Member:
    id: strawberry.ID
    username: str
    firstName: str | None = None
    lastName: str | None = None
    email: str
    emailVerified: bool
    enabled: bool
