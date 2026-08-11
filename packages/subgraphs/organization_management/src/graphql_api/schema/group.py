import strawberry


@strawberry.type
class Group:
    id: strawberry.ID
    name: str
    description: str
    path: str
    parentId: str
    subGroupCount: int


@strawberry.input
class GroupInput:
    org_id: strawberry.ID
