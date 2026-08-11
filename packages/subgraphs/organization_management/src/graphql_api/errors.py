class AppError(Exception):
    """Base class for all known/expected application errors."""
    code = "APP_ERROR"

    def __init__(self, message: str, code: str | None = None):
        super().__init__(message)
        self.message = message
        if code:
            self.code = code


class NotFoundError(AppError):
    code = "NOT_FOUND"


class UnauthorizedError(AppError):
    code = "UNAUTHORIZED"


class UpstreamServiceError(AppError):
    """Raised when a downstream service (e.g. Keycloak) fails."""
    code = "UPSTREAM_ERROR"
