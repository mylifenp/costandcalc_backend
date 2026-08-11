from enum import Enum

import strawberry

from graphql_api.resolvers.member import get_organization_members
from graphql_api.schema import Domain, Member


@strawberry.enum
class Status(Enum):
    SUCCESS = "success"
    FAILED = "failed"


@strawberry.federation.type(keys=["id"])
class Organization:
    id: strawberry.ID
    name: str
    alias: str
    enabled: bool
    description: str | None = None
    domains: list[Domain]
    members: list[Member] = strawberry.field(resolver=get_organization_members)


@strawberry.type
class OrganizationCreateConfirmation:
    name: str
    success: Status


@strawberry.input
class AddOrganizationInput:
    name: str
    alias: str
    enabled: bool
    description: str | None = None

@strawberry.input
class SearchOrganizationsInput:
    briefRepresentation: bool | None = None
    exact: bool | None = None
    first: int | None = None
    max: int | None = None
    q: str | None = None
    search: str | None = None
