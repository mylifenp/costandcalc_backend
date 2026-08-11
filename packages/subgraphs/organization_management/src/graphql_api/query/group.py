from requests import request
import strawberry

from graphql_api.schema.group import Group


class GroupQuery:
    @strawberry.field
    def group(self, info: strawberry.Info, group_id: str) -> Group:
        access_token = request.headers.get("Access-Token")
        if access_token is None:
            raise Exception("Access-Token header is missing")

        keycloak_client = get_keycloak_client(access_token)
        pass
