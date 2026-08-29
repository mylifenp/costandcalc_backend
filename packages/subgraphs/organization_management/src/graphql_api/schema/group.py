import strawberry

from graphql_api.schema.member import Member


@strawberry.type
class Group:
    id: strawberry.ID
    name: str
    description: str
    path: str
    parentId: str
    subGroupCount: int
    members: list[Member]


@strawberry.input
class GroupInput:
    org_id: strawberry.ID
