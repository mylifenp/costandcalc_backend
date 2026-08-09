import strawberry

from graphql_api.schema.user import User


def get_user(id: strawberry.ID) -> User:
    return User(
        id=id,
        name="Patrick",
        email="patric@test.com",
    )


def get_users():
    return [
        User(
            id= strawberry.ID("1"),
            name= "Patrick",
            email= "patric@test.com",
        ),
    ]
