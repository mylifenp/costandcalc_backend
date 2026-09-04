from __future__ import annotations

from typing import TypedDict, cast

import httpx2
import strawberry
from fastapi import Request

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.user import AddUserInput, User
from keycloak_client.client import keycloak_client


class _UserDict(TypedDict):
    id: str
    username: str
    email: str
    firstName: str
    lastName: str
    emailVerified: bool
    enabled: bool


def _to_user(user: _UserDict) -> User:
    return User(
        id=strawberry.ID(user["id"]),
        username=user["username"],
        email=user["email"],
        firstName=user["firstName"],
        lastName=user["lastName"],
        emailVerified=user["emailVerified"],
        enabled=user["enabled"],
    )


def get_user(id: strawberry.ID, info: strawberry.Info) -> User:
    request: Request = cast(Request, info.context["request"])
    client = keycloak_client(request)

    try:
        user = client.get_user(id)
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to fetch user") from e
    return _to_user(cast(_UserDict, user))


def get_users(info: strawberry.Info) -> list[User]:
    request: Request = cast(Request, info.context["request"])
    client = keycloak_client(request)

    try:
        users = client.get_users()
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to get users: {e}") from e
    # return [_to_user(user) for user in cast(list, users)]
    return [_to_user(cast(_UserDict, user)) for user in users]


def add_user(info: strawberry.Info, input: AddUserInput) -> User:
    user_input = {
        "username": input.username,
        "email": input.email,
        "enabled": input.enabled,
        "credentials": [
            {
                "type": input.credentials[0].type,
                "value": input.credentials[0].value,
            }
        ],
    }
    client = keycloak_client(cast(Request, info.context["request"]))
    try:
        client.users.post(user_input)
        user = cast(_UserDict, client.users.get(username=input.username)[0])
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create user") from e
    return _to_user(user)
