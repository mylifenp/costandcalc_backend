import httpx2
import strawberry

from graphql_api.errors import UpstreamServiceError
from graphql_api.resolvers.member import get_group_members
from graphql_api.schema.group import Group
from graphql_api.schema.organization import Organization
from graphql_api.schema.organization import Organization
from keycloak_client.client import keycloak_client


def _to_group(group: dict) -> Group:
    return Group(
        id = strawberry.ID(group["id"]),
        name=group["name"],
        description=group["description"],
        path=group["path"],
        parentId=group["parent_id"],
        subGroupCount=group["sub_group_count"],
        members = strawberry.field(resolver=get_group_members),
    )


def get_organization_groups(root: Organization, info: strawberry.Info) -> list[Group]:
    client = keycloak_client(info.context["request"])

    try:
        groups = client.organizations(root.id).groups().get()
        print("GROUPS", groups)
    except httpx2.HTTPError as err:
        raise UpstreamServiceError("Failed to fetch organization members") from err
    return [_to_group(group) for group in groups]
