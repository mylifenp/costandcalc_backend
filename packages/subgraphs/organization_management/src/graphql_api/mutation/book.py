import strawberry

from graphql_api.schema.book import AddBookInput, Book


@strawberry.type
class BookMutations:
    @strawberry.mutation
    def add_book(self, input: AddBookInput) -> Book:
        return Book(title=input.title, author=input.author)
