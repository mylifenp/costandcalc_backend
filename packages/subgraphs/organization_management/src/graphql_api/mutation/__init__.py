import strawberry

from graphql_api.mutation.book import BookMutations
from graphql_api.mutation.user import UserMutation


@strawberry.type
class Mutation(BookMutations, UserMutation):
    pass
