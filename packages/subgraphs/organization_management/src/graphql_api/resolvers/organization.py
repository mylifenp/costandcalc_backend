from __future__ import annotations

from typing import Any, Optional, TypedDict, cast

import httpx2
from mantelo.client import KeycloakAdmin
import strawberry
from fastapi import Request

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.domain import Domain
from graphql_api.schema.organization import (
    AddOrganizationInput,
    AddOrganizationMemberInput,
    AddOrganizationMemberPayload,
    Organization,
    OrganizationCreateConfirmation,
    RemoveOrganizationMemberInput,
    RemoveOrganizationMemberPayload,
    SearchOrganizationsInput,
    Status,
    UpdateOrganizationInput,
)
from keycloak_client.client import keycloak_client

organization_groups = ["OWNER", "ADMIN", "MEMBER", "VIEWER", "GUEST"]


class _DomainDict(TypedDict):
    name: str
    verified: bool


class _OrganizationDictRequired(TypedDict):
    id: str
    name: str
    alias: str
    enabled: bool


class _OrganizationDict(_OrganizationDictRequired, total=False):
    description: None | str
    domains: list[_DomainDict]


def _unwrap_response(resp: Any) -> Any:
    """Normalize client responses. Some client methods return the payload
    directly, others return a (response, body) tuple. Return the body/payload
    in either case.
    """
    # Some client wrappers return (Response, body) and some return body directly
    if isinstance(resp, tuple) and len(resp) >= 2:
        return resp[1]
    return resp


def _to_organization(org: _OrganizationDict) -> Organization:
    """Convert a Keycloak organization dict to the GraphQL Organization type."""
    return Organization(
        id=strawberry.ID(org["id"]),
        name=org["name"],
        alias=org["alias"],
        enabled=org["enabled"],
        description=org.get("description") or None,
        domains=[
            Domain(name=d["name"], verified=d["verified"])
            for d in org.get("domains", [])
        ],
    )


def get_organizations(info: strawberry.Info) -> list[Organization]:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        organizations_raw = client.organizations.get()
        organizations_payload = _unwrap_response(organizations_raw)
        organizations = cast(list[_OrganizationDict], organizations_payload)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to fetch organizations") from e
    return [_to_organization(cast(_OrganizationDict, org)) for org in organizations]


def get_organization(info: strawberry.Info, id: str) -> Organization:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        organization_raw = client.organizations(id).get()
        organization = cast(_OrganizationDict, organization_raw)
        print("ORGANIZATION", organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to fetch organization") from e
    return _to_organization(organization)


def create_organization_group(
    client: KeycloakAdmin, organization_id: str, group_name: str
) -> None:
    try:
        client.organizations(organization_id).groups.post({"name": group_name})
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create organization group") from e


def create_organization(
    info: strawberry.Info,
    input: AddOrganizationInput,
) -> OrganizationCreateConfirmation:
    org_input = {
        "name": input.name,
        "alias": input.alias,
        "enabled": input.enabled,
        "description": input.description,
    }
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        resp, _ = client.organizations.as_raw().post(org_input)
        location = resp.headers["Location"]
        organization_id = location.split("/")[-1]
        if organization_id is None:
            raise UpstreamServiceError("Failed to create organization")
        for group_name in organization_groups:
            create_organization_group(client, organization_id, group_name)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create organization") from e
    return OrganizationCreateConfirmation(name=input.name, success=Status.SUCCESS)


def search_organizations(
    info: strawberry.Info, query: SearchOrganizationsInput
) -> list[Organization]:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        organizations_raw = client.organizations.search(query).get()
        organizations_payload = _unwrap_response(organizations_raw)
        organizations = cast(list[_OrganizationDict], organizations_payload)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to search organizations") from e
    return [_to_organization(cast(_OrganizationDict, org)) for org in organizations]


def update_organization(
    info: strawberry.Info,
    id: str,
    input: UpdateOrganizationInput,
) -> Organization:
    org_input = {
        "name": input.name,
        "alias": input.alias,
        "enabled": input.enabled,
        "description": input.description,
    }
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        client.organizations(id).put(org_input)
        organization_raw = client.organizations(id).get()
        organization_payload = _unwrap_response(organization_raw)
        organization = cast(_OrganizationDict, organization_payload)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to update organization") from e
    return _to_organization(organization)


def activate_organization(
    info: strawberry.Info,
    id: str,
) -> Organization:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        organization_raw = client.organizations(id).get()
        organization_payload = _unwrap_response(organization_raw)
        organization = cast(_OrganizationDict, organization_payload)
        organization["enabled"] = True
        client.organizations(id).put(organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to activate organization") from e
    return _to_organization(organization)


def deactivate_organization(
    info: strawberry.Info,
    id: str,
) -> Organization:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        organization_raw = client.organizations(id).get()
        organization_payload = _unwrap_response(organization_raw)
        organization = cast(_OrganizationDict, organization_payload)
        organization["enabled"] = False
        client.organizations(id).put(organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to deactivate organization") from e
    return _to_organization(organization)


def add_organization_member(
    input: AddOrganizationMemberInput, info: strawberry.Info
) -> AddOrganizationMemberPayload:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        # client.organization_user_add(
        #     user_id=str(input.user_id),
        #     organization_id=str(input.organization_id),
        # )
        client.organizations(str(input.organization_id)).members.post(
            str(input.user_id)
        )
    except Exception as err:
        raise UpstreamServiceError("Failed to add user to organization") from err

    return AddOrganizationMemberPayload(success=True)


def remove_organization_member(
    input: RemoveOrganizationMemberInput, info: strawberry.Info
) -> RemoveOrganizationMemberPayload:
    request = cast(Request, info.context["request"])
    client = keycloak_client(request)
    try:
        client.organizations(str(input.organization_id)).members(
            str(input.user_id)
        ).delete()
    except Exception as err:
        raise UpstreamServiceError("Failed to remove user from organization") from err

    return RemoveOrganizationMemberPayload(success=True)
