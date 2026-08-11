
from __future__ import annotations

from typing import cast

import httpx2
import strawberry
from fastapi import Request

from graphql_api.errors import UpstreamServiceError
from graphql_api.schema.user import AddUserInput, User
from keycloak_client.client import keycloak_client


def _to_user(user: dict) -> User:
    return User(
        id=user["id"],
        username=user["username"],
        email=user["email"],
    )


def get_user(id: strawberry.ID) -> User:
    return User(
        id=id,
        username="Patrick",
        email="patric@test.com",
    )

def get_users(self, info: strawberry.Info) -> list[User]:
    request: Request = info.context["request"]

    access_token = request.headers.get("Access-Token")
    if access_token is None:
        raise Exception("Access-Token header is missing")
    keycloak_client = connect_keycloak_client(access_token)

    try:
        users = keycloak_client.get_users()
        print("Users", users)
        return [_to_user(user) for user in cast(list, users)]
    except Exception as e:
        raise Exception(f"Failed to get users: {e}")

def add_user(self, info: strawberry.Info, input: AddUserInput) -> User:
    user_input = {
        "username": input.username,
        "email": input.email,
        "enabled": input.enabled,
        "credentials": [{
            "type": input.credentials[0].type,
            "value": input.credentials[0].value,
        }],
    }
    client = keycloak_client(info.context["request"])
    try:
        client.users.post(user_input)
        user = client.users.get(username=input.username)[0]
    except httpx2.HTTPError as e:
        raise UpstreamServiceError("Failed to create user") from e
    return _to_user(cast(dict, user))
