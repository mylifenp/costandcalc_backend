from typing import cast

import strawberry
from fastapi import Request

from graphql_api.schema.group import Group
from keycloak_client.client import keycloak_client


class GroupQuery:
    @strawberry.field
    def group(self, info: strawberry.Info, group_id: str) -> None:
        client = keycloak_client(cast(Request, info.context["request"]))
        pass
