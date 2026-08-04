use async_graphql::{Context, Result};
use backend_libs_auth::details::Claims;


#[derive(Debug)]
pub struct UserValues {
    pub organization_name: String,
    pub organization_ids: Vec<String>,
    pub role: String,
}

pub async fn get_values_from_claims(ctx: &Context<'_>) -> Result<UserValues> {
    let claims = ctx
        .data::<Option<Claims>>()?
        .as_ref()
        .ok_or("Claims not found")?;
    let organisation_name = claims
        .organization
        .as_ref()
        .ok_or("Organization not found")?;

    let organization_ids = claims.organization_ids();

    let roles = claims.roles();
    let role = if roles.contains(&"SUPER_USER".to_string()) {
        "SUPER_USER".to_string()
    } else if roles.contains(&"ADMIN".to_string()) {
        "ADMIN".to_string()
    } else {
        "USER".to_string()
    };

    Ok(UserValues {
        organization_name: organisation_name.clone(),
        organization_ids,
        role,
    })
}

pub async fn assert_is_super_user(ctx: &Context<'_>) -> Result<bool> {
    let admin_org = std::env::var("KEYCLOAK_ADMINISTRATOR_ORGANIZATION")
        .unwrap_or_else(|_| "administration".to_string());
    let user_values = get_values_from_claims(ctx).await?;

    if user_values.organization_name == admin_org && user_values.role == "SUPER_USER" {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn assert_is_organization_admin(
    ctx: &Context<'_>,
    organization_id: &str,
) -> Result<bool> {
    let user_values = get_values_from_claims(ctx).await?;

    if user_values
        .organization_ids
        .contains(&organization_id.to_string())
        && user_values.role == "ADMIN"
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn assert_is_organization_user(ctx: &Context<'_>, organization_id: &str) -> Result<bool> {
    let user_values = get_values_from_claims(ctx).await?;

    if user_values
        .organization_ids
        .contains(&organization_id.to_string())
        && user_values.role == "USER"
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

// pub async fn assert_is_super_user_or_producer_admin(
//     ctx: &Context<'_>,
//     producer_id: &str,
// ) -> Result<()> {
//     // If super user, allow
//     if assert_is_super_user(ctx).await.is_ok() {
//         return Ok(());
//     }
//     // If producer matches and is admin, allow
//     if assert_account_is_active(ctx).await.is_ok()
//         && assert_is_producers_user(ctx, &producer_id).await.is_ok()
//         && assert_is_producer_admin(ctx, &producer_id).await.is_ok()
//     {
//         return Ok(());
//     }
//     // Otherwise, error out
//     Err("Insufficient permissions".into())
// }

// pub async fn assert_is_super_user_or_producer_admin_or_is_own_data(
//     ctx: &Context<'_>,
//     producer_id: &str,
//     user_id: &str,
// ) -> Result<()> {
//     // If super user, allow
//     if assert_is_super_user(ctx).await.is_ok() {
//         return Ok(());
//     }
//     // If admin, allow
//     if assert_account_is_active(ctx).await.is_ok()
//         && assert_is_producer_admin(ctx, &producer_id).await.is_ok()
//     {
//         return Ok(());
//     }
//     // If own data, allow
//     if assert_is_own_data(ctx, &user_id).await.is_ok() {
//         return Ok(());
//     }
//     // Otherwise, error out
//     Err("Insufficient permissions".into())
// }

// pub async fn assert_is_own_data(ctx: &Context<'_>, user_id: &str) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;

//     let input_user_id = parse_oid(&user_id)?;
//     if let Some(uid) = user.id {
//         if uid == input_user_id {
//             return Ok(());
//         }
//     }
//     Err("User ID mismatch".into())
// }

// pub async fn assert_is_super_user(ctx: &Context<'_>) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;
//     let super_user_email =
//         std::env::var("SUPER_USER").unwrap_or_else(|_| "info@meatone.de".to_string());
//     if user.email != super_user_email {
//         return Err("Insufficient permissions".into());
//     }
//     Ok(())
// }

// pub async fn assert_account_is_active(ctx: &Context<'_>) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;

//     if user.account_status != DbAccountStatus::ACTIVE {
//         return Err("User account is not active".into());
//     }
//     Ok(())
// }

// pub async fn assert_is_producers_user(ctx: &Context<'_>, producer_id: &str) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;

//     let input_producer_id = parse_oid(&producer_id)?;
//     if user.producer_id != input_producer_id {
//         return Err("Producer ID mismatch".into());
//     }
//     Ok(())
// }

// pub async fn assert_is_admin(ctx: &Context<'_>) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;
//     if user.role != DbRoleType::ADMIN {
//         return Err("Insufficient permissions".into());
//     }
//     Ok(())
// }

// pub async fn assert_is_producer_admin(ctx: &Context<'_>, producer_id: &str) -> Result<()> {
//     let user = fetch_user_from_claims(ctx).await?.ok_or("User not found")?;
//     if user.producer_id.to_hex() != producer_id {
//         return Err("Producer ID mismatch".into());
//     }
//     assert_is_admin(ctx).await?;
//     Ok(())
// }
