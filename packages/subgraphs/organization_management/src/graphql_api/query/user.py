import strawberry

from graphql_api.schema.user import User
from repositories.user import get_user, get_users


@strawberry.type
class UserQuery:
    users: list[User] = strawberry.field(resolver=get_users)
    user: User = strawberry.field(resolver=get_user)
