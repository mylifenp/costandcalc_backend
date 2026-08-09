import strawberry

from graphql_api.mutation import Mutation
from graphql_api.query import Query

schema = strawberry.federation.Schema(
    query=Query,
    mutation=Mutation,
    federation_version="2.5",
)
