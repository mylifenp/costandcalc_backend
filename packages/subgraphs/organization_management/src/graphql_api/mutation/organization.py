import strawberry

from graphql_api.resolvers.organization import create_organization
from graphql_api.schema.organization import Organization


@strawberry.type
class OrganizationMutation:
    create_organization: Organization = strawberry.field(resolver=create_organization)
    # @strawberry.mutation
    # def create_organization(
    #     self,
    #     info: strawberry.Info,
    #     input: AddOrganizationInput,
    # ) -> Organization:
    #     client = keycloak_client(info.context["request"])
    #     try:
    #         organization = client.realms(input.realm).organizations.post()
    #     except Exception as e:
    #         raise e
    #     return Organization(
    #         id=organization.id,
    #         name=organization.name,
    #         alias=organization.alias,
    #         enabled=organization.enabled,
    #         description=organization.description,
    #         domains=organization.domains,
    #     )
