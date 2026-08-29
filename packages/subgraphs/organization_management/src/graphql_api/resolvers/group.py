from typing import TYPE_CHECKING, TypedDict, cast

import httpx2
import strawberry
from fastapi import Request

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.group import Group
from keycloak_client.client import keycloak_client

if TYPE_CHECKING:
    from graphql_api.schema.organization import Organization


class _GroupDict(TypedDict):
    id: str
    name: str
    description: str | None
    path: str | None
    parent_id: str | None
    sub_group_count: int | None


def _to_group(group: _GroupDict) -> Group:
    # build a plain Group; members are resolved via the Group.members
    # resolver defined in the schema implementation (or via the
    # get_group_members resolver) to avoid import cycles.

    description = group.get("description") if group.get("description") else None
    path = group.get("path") if group.get("path") else None
    parentId = group.get("parent_id") if group.get("parent_id") else None
    subGroupCount = group.get("sub_group_count") if group.get("sub_group_count") else 0
    return Group(
        id=strawberry.ID(group["id"]),
        name=group.get("name"),
        description=description,
        path=path,
        parentId=parentId,
        subGroupCount=subGroupCount,
    )


def get_organization_groups(root: "Organization", info: strawberry.Info) -> list[Group]:
    client = keycloak_client(cast(Request, info.context["request"]))

    try:
        groups = client.organizations(root.id).groups().get()
    except httpx2.HTTPError as err:
        raise UpstreamServiceError("Failed to fetch organization members") from err

    return [_to_group(cast(_GroupDict, group)) for group in groups]
