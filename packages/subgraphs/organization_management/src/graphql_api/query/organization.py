
import strawberry

from graphql_api.resolvers.organization import get_organization, get_organizations
from graphql_api.schema.organization import Organization


@strawberry.type
class OrganizationQuery:
    organizations: list[Organization] = strawberry.field(resolver=get_organizations)
    organization: Organization = strawberry.field(resolver=get_organization)
