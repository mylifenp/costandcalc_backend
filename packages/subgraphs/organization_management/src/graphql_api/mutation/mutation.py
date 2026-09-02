import strawberry

from graphql_api.mutation.organization import OrganizationMutation
from graphql_api.mutation.user import UserMutation


@strawberry.type
class Mutation(UserMutation, OrganizationMutation):
    pass
