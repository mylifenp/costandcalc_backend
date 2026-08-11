import os

from dotenv import load_dotenv
from fastapi.requests import Request
from mantelo import KeycloakAdmin
from mantelo.client import BearerAuth

from graphql_api.errors import UnauthorizedError

load_dotenv()

KEYCLOAK_URL = os.getenv("KEYCLOAK_URL", "https://auth.datenium.de/").strip('/')
KEYCLOAK_REALM = os.getenv("KEYCLOAK_REALM", "cost_and_calc")
KEYCLOAK_CLIENT_ID = os.getenv("KEYCLOAK_CLIENT_ID", "cost_and_calc_id")
KEYCLOAK_CLIENT_SECRET = os.getenv("KEYCLOAK_CLIENT_SECRET", "SomeSecret")


# client = KeycloakAdmin.from_client_credentials(
#     server_url=KEYCLOAK_URL,
#     realm_name=KEYCLOAK_REALM,
#     client_id=KEYCLOAK_CLIENT_ID,
#     client_secret=KEYCLOAK_CLIENT_SECRET,
# )

def keycloak_client(request: Request) -> KeycloakAdmin:
    token = request.headers.get("Access-Token")
    if not token:
        raise UnauthorizedError("Access token is missing")
    return connect_keycloak_client(token)

def connect_keycloak_client(token: str) -> KeycloakAdmin:
    return KeycloakAdmin(
        server_url=KEYCLOAK_URL,
        realm_name=KEYCLOAK_REALM,
        auth=BearerAuth(lambda: token)
    )

# client = KeycloakAdmin(
#     server_url=KEYCLOAK_URL,
#     realm_name=KEYCLOAK_REALM,
#     auth=BearerAuth(lambda: token)
# )
