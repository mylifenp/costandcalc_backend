import strawberry

from graphql_api.resolvers.user import get_user, get_users
from graphql_api.schema.user import User


@strawberry.type
class UserQuery:
    users: list[User] = strawberry.field(resolver=get_users)
    user: User = strawberry.field(resolver=get_user)
