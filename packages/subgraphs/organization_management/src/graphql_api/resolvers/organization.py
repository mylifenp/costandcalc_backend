from __future__ import annotations

from typing import cast, Any, List, Optional, TypedDict

import httpx2
import strawberry

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



class _DomainDict(TypedDict):
    name: str
    verified: bool


class _OrganizationDict(TypedDict, total=False):
    id: str
    name: str
    alias: str
    enabled: bool
    description: Optional[str]
    domains: List[_DomainDict]


def _to_organization(org: dict) -> Organization:
    """Convert a Keycloak organization dict to the GraphQL Organization type."""
    return Organization(
        id=org["id"],
        name=org["name"],
        alias=org["alias"],
        enabled=org["enabled"],
        description=org.get("description") or None,
        domains=[
            Domain(name=d["name"], verified=d["verified"])
            for d in org.get("domains", [])
        ],
    )


def get_organizations(self, info: strawberry.Info) -> list[Organization]:
    client = keycloak_client(info.context["request"])
    try:
        organizations = client.organizations.get()
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to fetch organizations") from e
    return [_to_organization(cast(dict, org)) for org in organizations]


def get_organization(self, info: strawberry.Info, id: strawberry.ID) -> Organization:
    client = keycloak_client(info.context["request"])
    try:
        organization = client.organizations(id).get()
        print("ORGANIZATION", organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to fetch organization") from e
    return _to_organization(cast(dict, organization))


def create_organization_group(client, organization_id: str, group_name: str) -> None:
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
    client = keycloak_client(info.context["request"])
    try:
        resp, _ = client.organizations.as_raw().post(org_input)
        location = resp.headers["Location"]
        organization_id = location.split("/")[-1]
        print("ORGANIZATION_ID", organization_id)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create organization") from e
    return OrganizationCreateConfirmation(name=input.name, success=Status.SUCCESS)


def search_organizations(
    info: strawberry.Info, query: SearchOrganizationsInput
) -> list[Organization]:
    client = keycloak_client(info.context["request"])
    try:
        organizations = client.organizations.search(query).get()
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to search organizations") from e
    return [_to_organization(cast(dict, org)) for org in organizations]


def update_organization(
    info: strawberry.Info,
    id: strawberry.ID,
    input: UpdateOrganizationInput,
) -> Organization:
    org_input = {
        "name": input.name,
        "alias": input.alias,
        "enabled": input.enabled,
        "description": input.description,
    }
    client = keycloak_client(info.context["request"])
    try:
        client.organizations(id).put(org_input)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to update organization") from e
    return _to_organization(cast(dict, org_input))


def activate_organization(
    info: strawberry.Info,
    id: strawberry.ID,
) -> Organization:
    client = keycloak_client(info.context["request"])
    try:
        organization = client.organizations(id).get()
        organization["enabled"] = True
        client.organizations(id).put(organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to activate organization") from e
    return _to_organization(cast(dict, organization))


def deactivate_organization(
    self,
    info: strawberry.Info,
    id: strawberry.ID,
) -> Organization:
    client = keycloak_client(info.context["request"])
    try:
        organization = client.organizations(id).get()
        organization["enabled"] = False
        client.organizations(id).put(organization)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to deactivate organization") from e
    return _to_organization(cast(dict, organization))


def add_organization_member(
    input: AddOrganizationMemberInput, info: strawberry.Info
) -> AddOrganizationMemberPayload:
    client = keycloak_client(info.context["request"])
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
    client = keycloak_client(info.context["request"])
    try:
        client.organizations(str(input.organization_id)).members(
            str(input.user_id)
        ).delete()
    except Exception as err:
        raise UpstreamServiceError("Failed to remove user from organization") from err

    return RemoveOrganizationMemberPayload(success=True)
