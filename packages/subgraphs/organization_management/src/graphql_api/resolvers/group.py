import httpx2
import strawberry
from typing import TYPE_CHECKING

from graphql_api.errors import UpstreamServiceError
from graphql_api.resolvers.member import get_group_members
from graphql_api.schema.group import Group
from keycloak_client.client import keycloak_client

if TYPE_CHECKING:
    from graphql_api.schema.organization import Organization


def _to_group(group: dict) -> Group:
    # build a plain Group; members are resolved via the Group.members
    # resolver defined in the schema implementation (or via the
    # get_group_members resolver) to avoid import cycles.
    return Group(
        id=strawberry.ID(group["id"]),
        name=group.get("name"),
        description=group.get("description"),
        path=group.get("path"),
        parentId=group.get("parent_id"),
        subGroupCount=group.get("sub_group_count") or 0,
    )


def get_organization_groups(root: "Organization", info: strawberry.Info) -> list[Group]:
    client = keycloak_client(info.context["request"])

    try:
        groups = client.organizations(root.id).groups().get()
        print("GROUPS", groups)
    except httpx2.HTTPError as err:
        raise UpstreamServiceError("Failed to fetch organization members") from err
    return [_to_group(group) for group in groups]
