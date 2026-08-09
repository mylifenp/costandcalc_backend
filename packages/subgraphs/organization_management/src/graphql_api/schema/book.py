import strawberry


@strawberry.type
class Book:
    title: str
    author: str

@strawberry.input
class AddBookInput:
    title: str
    author: str
