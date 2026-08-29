from __future__ import annotations

from typing import TYPE_CHECKING, TypedDict, cast

import httpx2
import strawberry
from fastapi import Request

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.group import Group
from graphql_api.schema.member import Member
from keycloak_client.client import keycloak_client

if TYPE_CHECKING:
    from graphql_api.schema.organization import Organization


class _MemberDict(TypedDict):
    id: str
    username: str
    firstName: str | None
    lastName: str | None
    email: str
    emailVerified: bool
    enabled: bool


def _to_member(member: _MemberDict) -> Member:
    # Be defensive: upstream may omit optional fields like firstName/lastName.
    # Required fields (id, username, email) should be present; raise a
    # controlled error if they're missing so callers get a clear message.
    id_val = member.get("id")
    username = member.get("username")
    email = member.get("email")

    if not id_val or not username or not email:
        raise UpstreamServiceError("Missing required member fields from upstream")

    return Member(
        id=strawberry.ID(id_val),
        username=username,
        firstName=member.get("firstName") or None,
        lastName=member.get("lastName") or None,
        email=email,
        emailVerified=bool(member.get("emailVerified", False)),
        enabled=bool(member.get("enabled", False)),
    )


def get_organization_members(root: Organization, info: strawberry.Info) -> list[Member]:
    client = keycloak_client(cast(Request, info.context["request"]))

    try:
        members = client.organizations(root.id).members.get()
        print("MEM", members)
    except httpx2.HTTPError as err:
        raise UpstreamServiceError("Failed to fetch organization members") from err
    return [_to_member(cast(_MemberDict, member)) for member in members]


# def get_group_members(root: Group, info: strawberry.Info) -> list[Member]:
#     client = keycloak_client(cast(Request, info.context["request"]))

#     try:
#         members = client.groups(root.id).members.get()
#     except httpx2.HTTPError as err:
#         raise UpstreamServiceError("Failed to fetch group members") from err
#     return [_to_member(cast(_MemberDict, member)) for member in members]
