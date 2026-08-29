from enum import Enum

import strawberry

from graphql_api.resolvers.group import get_organization_groups
from graphql_api.resolvers.member import get_organization_members
from graphql_api.schema.domain import Domain
from graphql_api.schema.group import Group
from graphql_api.schema.member import Member


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
    members: list[Member] | None = strawberry.field(resolver=get_organization_members)
    groups: list[Group] | None = strawberry.field(resolver=get_organization_groups)


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
class UpdateOrganizationInput:
    name: str | None = None
    alias: str | None = None
    enabled: bool | None = None
    description: str | None = None


@strawberry.input
class SearchOrganizationsInput:
    briefRepresentation: bool | None = None
    exact: bool | None = None
    first: int | None = None
    max: int | None = None
    q: str | None = None
    search: str | None = None


@strawberry.input
class AddOrganizationMemberInput:
    organization_id: strawberry.ID
    user_id: strawberry.ID


@strawberry.type
class AddOrganizationMemberPayload:
    success: bool


@strawberry.input
class RemoveOrganizationMemberInput:
    organization_id: strawberry.ID
    user_id: strawberry.ID


@strawberry.type
class RemoveOrganizationMemberPayload:
    success: bool
