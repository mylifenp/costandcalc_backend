import logging

import strawberry
from graphql.error import GraphQLError

from graphql_api.errors import AppError
from graphql_api.mutation import Mutation
from graphql_api.query import Query

logger = logging.getLogger(__name__)


class AppSchema(strawberry.federation.Schema):
    def process_errors(self, errors, execution_context=None):
        # Still log everything server-side, including internals
        for error in errors:
            logger.error("GraphQL error", exc_info=error.original_error or error)

    def format_error(self, error: GraphQLError):
        original = error.original_error

        if isinstance(original, AppError):
            return {
                "message": original.message,
                "extensions": {"code": original.code},
            }

        # Unknown/unexpected errors: don't leak internals to the client
        return {
            "message": "Internal server error",
            "extensions": {"code": "INTERNAL_SERVER_ERROR"},
        }


schema = AppSchema(query=Query, mutation=Mutation, federation_version="2.5")
