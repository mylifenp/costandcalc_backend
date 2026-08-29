from typing import TYPE_CHECKING

import strawberry

from graphql_api.resolvers.member import get_group_members
from graphql_api.schema.member import Member

if TYPE_CHECKING:
    # Keep typing-only imports local to avoid runtime cycles
    from graphql_api.schema.organization import Organization  # noqa: F401


@strawberry.type
class Group:
    id: strawberry.ID
    name: str
    description: str | None = None
    path: str | None = None
    parentId: str | None = None
    subGroupCount: int | None = None
    members: list[Member] | None = strawberry.field(resolver=get_group_members)


@strawberry.input
class GroupInput:
    org_id: strawberry.ID
