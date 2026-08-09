import strawberry

from graphql_api.query.book import BookQuery
from graphql_api.query.user import UserQuery


@strawberry.type
class Query(UserQuery, BookQuery):
    pass
