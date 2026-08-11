import strawberry

from graphql_api.query.group import GroupQuery
from graphql_api.query.organization import OrganizationQuery
from graphql_api.query.user import UserQuery


@strawberry.type
class Query(UserQuery, OrganizationQuery, GroupQuery):
    pass
