import strawberry

from graphql_api.resolvers.user import add_user
from graphql_api.schema import User


@strawberry.type
class UserMutation:
    add_user: User = strawberry.field(resolver=add_user)
