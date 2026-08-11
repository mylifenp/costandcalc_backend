from __future__ import annotations

from typing import TYPE_CHECKING, cast

import httpx2
import strawberry

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.organization import (
    AddOrganizationInput,
    OrganizationCreateConfirmation,
    SearchOrganizationsInput,
    Status,
)

if TYPE_CHECKING:
    from graphql_api.schema import Domain, Organization
from keycloak_client.client import keycloak_client


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


def create_organization(
    self,
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
        client.organizations.post(org_input)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create organization") from e
    return OrganizationCreateConfirmation(name=input.name, success=Status.SUCCESS)


def search_organizations(
    self,
    info: strawberry.Info,
    query: SearchOrganizationsInput
) -> list[Organization]:
    client = keycloak_client(info.context["request"])
    try:
        organizations = client.organizations.search(query).get()
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to search organizations") from e
    return [_to_organization(cast(dict, org)) for org in organizations]
