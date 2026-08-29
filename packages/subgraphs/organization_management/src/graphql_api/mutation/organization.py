import strawberry

from graphql_api.resolvers.organization import (
    activate_organization,
    add_organization_member,
    create_organization,
    deactivate_organization,
    remove_organization_member,
    update_organization,
)
from graphql_api.schema.organization import (
    AddOrganizationMemberPayload,
    Organization,
    OrganizationCreateConfirmation,
    RemoveOrganizationMemberPayload,
)


@strawberry.type
class OrganizationMutation:
    create_organization: OrganizationCreateConfirmation = strawberry.mutation(
        resolver=create_organization
    )
    update_organization: Organization = strawberry.mutation(
        resolver=update_organization
    )
    activate_organization: Organization = strawberry.mutation(
        resolver=activate_organization
    )
    deactivate_organization: Organization = strawberry.mutation(
        resolver=deactivate_organization
    )
    add_organization_member: AddOrganizationMemberPayload = strawberry.mutation(
        resolver=add_organization_member
    )
    remove_organization_member: RemoveOrganizationMemberPayload = strawberry.mutation(
        resolver=remove_organization_member
    )
