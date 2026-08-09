from graphql_api.schema.book import Book


def get_book():
    return Book(
        title="The Great Gatsby",
        author="F. Scott Fitzgerald",
    )


def get_books():
    return [
        Book(
            title="The Great Gatsby",
            author="F. Scott Fitzgerald",
        ),
    ]
