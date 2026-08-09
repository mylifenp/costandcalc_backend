import strawberry

from graphql_api.schema.book import Book
from repositories.book import get_book, get_books


@strawberry.type
class BookQuery:
    books: list[Book] = strawberry.field(resolver=get_books)
    book: Book = strawberry.field(resolver=get_book)
