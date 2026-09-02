<!-- description costandcalc start-->

CostAndCalc: it is a time management Saas tool for craftsmen and craftswoman. A user that wants something fixed shall take a few pictures and post it online on costandcalc.com. The picture shall be process and analyzed by the AI model to extract relevant information. A preliminary of the cost shall be calculated and displayed to the user. The pool of craftsmen shall be notified of the new job with the estimated cost. He can bid on the job and accept or reject it.
<!-- description costandcalc end-->

<!-- nx configuration start-->
<!-- Leave the start & end comments to automatically receive updates. -->

## General Guidelines for working with Nx

- For navigating/exploring the workspace, invoke the `nx-workspace` skill first - it has patterns for querying projects, targets, and dependencies
- When running tasks (for example build, lint, test, e2e, etc.), always prefer running the task through `nx` (i.e. `nx run`, `nx run-many`, `nx affected`) instead of using the underlying tooling directly
- Prefix nx commands with the workspace's package manager (e.g., `pnpm nx build`, `npm exec nx test`) - avoids using globally installed CLI
- You have access to the Nx MCP server and its tools, use them to help the user
- For Nx plugin best practices, check `node_modules/@nx/<plugin>/PLUGIN.md`. Not all plugins have this file - proceed without it if unavailable.
- NEVER guess CLI flags - always check nx_docs or `--help` first when unsure

## Scaffolding & Generators

- For scaffolding tasks (creating apps, libs, project structure, setup), ALWAYS invoke the `nx-generate` skill FIRST before exploring or calling MCP tools

## When to use nx_docs

- USE for: advanced config options, unfamiliar flags, migration guides, plugin configuration, edge cases
- DON'T USE for: basic generator syntax (`nx g @nx/react:app`), standard commands, things you already know
- The `nx-generate` skill handles generator discovery internally - don't call nx_docs just to look up generator syntax

<!-- nx configuration end-->
<!-- bounded context for costandcalc start-->

# Bounded Context for CostAndCalc

1. Identity & access

- Responsibility: Users, professionals, admins, roles, permissions.
- Core concepts: User, ProfessionalProfile, AdminUser, Role.
- GraphQL: Owns User as a federated entity.

2. Assets & defects

- Responsibility: Things that can be defective (roof, window, etc.) and defect reports.
- Core concepts: Asset, Defect, DefectPhoto.

3. Estimation (AI cost calculation)

- Responsibility: AI-based cost estimation from photos and defect metadata.
- Core concepts: Estimation, EstimationModel, EstimationConfig.

4. Work management (requests & offers)

- Responsibility: Requests to fix defects, offers from professionals, job lifecycle.
- Core concepts: FixRequest, Offer, Job.

5. Pricing & billing

- Responsibility: Invoices, payments, taxes, currency handling.
- Core concepts: Invoice, Payment, Money.

6. Media

- Responsibility: Photo storage, URLs, transformations.
- Core concepts: MediaObject, MediaUsage.

7. Notifications

- Responsibility: Email, push, in-app notifications.
- Core concepts: Notification, Channel.

8. Admin & configuration

- Responsibility: System-wide settings, feature flags, professional onboarding rules.
- Core concepts: SystemConfig, FeatureFlag, ProfessionalRule.

<!-- bounded context for costandcalc end-->

<!-- technical details about this project start-->

# This is the backend monorepo for the project "costandcalc"

the project and the technologies used are described below

The project will implement federated GraphQL using cosmo-router. Each backend server will be created under the `packages/subgraphs/` directory.

- One backend service `subgraphs_organization_management` which is also a part of the monorepo, is created using python version3.14 [https://docs.python.org/3/]. Uv is used as the package manager. For graphql, strawberry is used. It shall be used to work with keycloak organizations/users management. There exists a library called Mantelo [https://pypi.org/project/mantelo/], which could make the working with keycloak easier.
- all other services use rust with axum and async-graphql.
- for persistence, MongoDB is used.
- for authentication, Keycloak is used. The keycloak runs at https://auth.datenium.de, which is one of our server.
- for caching, Redis is used.
- for pub/sub, Redis is used.
- for S3 bucket storage, minio is deployed on our server.
- cosmo-router is used for federated GraphQL. available at `https://api.costandcalc.com/graphql`

Frontends are not a part of this monorepo. They are separate projects. Nextjs is used for the frontend.

<!-- technical details about this project end-->
