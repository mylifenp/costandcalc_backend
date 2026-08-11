from __future__ import annotations

from typing import TYPE_CHECKING, cast

import httpx2
import strawberry

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema import Member
from keycloak_client.client import keycloak_client

if TYPE_CHECKING:
    from graphql_api.schema import Organization


def _to_member(member: dict) -> Member:
    return Member(
        id=member["id"],
        username=member["username"],
        firstName=member["firstName"],
        lastName=member["lastName"],
        email=member["email"],
        emailVerified=member["emailVerified"],
        enabled=member["enabled"],
    )


def get_organization_members(root: Organization, info: strawberry.Info) -> list[Member]:
    client = keycloak_client(info.context["request"])

    try:
        members = client.organizations(root.id).members.get()
    except httpx2.HTTPError as err:
        raise UpstreamServiceError("Failed to fetch organization members") from err
    return [_to_member(cast(dict, member)) for member in members]
