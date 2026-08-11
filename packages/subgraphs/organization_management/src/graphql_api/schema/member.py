
import strawberry


@strawberry.type
class Member:
    id: strawberry.ID
    username: str
    firstName: str
    lastName: str
    email: str
    emailVerified: bool
    enabled: bool
