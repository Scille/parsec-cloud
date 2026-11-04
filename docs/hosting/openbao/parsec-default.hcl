# -------------------------------------------------------------------------
# Token self-management
# -------------------------------------------------------------------------

# Allow tokens to look up their own properties
path "auth/token/lookup-self" {
    capabilities = ["read"]
}

# Allow tokens to renew themselves
path "auth/token/renew-self" {
    capabilities = ["update"]
}

# Allow tokens to revoke themselves
path "auth/token/revoke-self" {
    capabilities = ["update"]
}

# Allow a token to look up its own entity by id or name
path "identity/entity/id/{{identity.entity.id}}" {
    capabilities = ["read"]
}
path "identity/entity/name/{{identity.entity.name}}" {
    capabilities = ["read"]
}

# Allow a token to look up its resultant ACL from all policies
path "sys/internal/ui/resultant-acl" {
    capabilities = ["read"]
}

# Allow a token to make requests to the Authorization Endpoint for OIDC providers
path "identity/oidc/provider/+/authorize" {
    capabilities = ["read", "update"]
}

# -------------------------------------------------------------------------
# Parsec per-entity device key store
# (used to protect device files stored on the user's machine)
# -------------------------------------------------------------------------

# Allow an entity to store its own device keys
path "secret/data/{{identity.entity.id}}/*" {
    capabilities = ["create", "update", "patch", "read", "delete"]
}
path "secret/metadata/{{identity.entity.id}}/*" {
    capabilities = ["read", "list", "delete"]
}

# -------------------------------------------------------------------------
# Parsec entity-to-entity authenticated message passing
# (used to sign and verify asynchronous enrollment payloads)
# -------------------------------------------------------------------------

# User creates its own signing key in the transit engine
path "transit/keys/entity-{{identity.entity.id}}" {
    capabilities = ["update"]
}

# User signs a message with its own key
path "transit/sign/entity-{{identity.entity.id}}" {
    capabilities = ["update"]
}

# Any user can read another entity's metadata (to retrieve its email)
path "identity/entity/id/*" {
    capabilities = ["read"]
}

# Any user can verify a signature made by another entity
path "transit/verify/entity-*" {
    capabilities = ["update"]
}
