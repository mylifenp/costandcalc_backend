import strawberry

from graphql_api.schema.user import AddUserInput, User


@strawberry.type
class UserMutation:
    @strawberry.mutation
    def add_user(self, input: AddUserInput) -> User:
        return User(id=strawberry.ID("5454"), name=input.name, email=input.email)
