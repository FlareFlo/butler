use poise::Context;
use serenity::all::{Member, Permissions};

pub trait SerenityExt {
    fn has_admin<U, E>(&self, ctx: &Context<U, E>) -> bool;
    fn has_permission<U, E>(&self, ctx: &Context<U, E>, check: impl FnMut(Permissions) -> bool) -> bool;
}

impl SerenityExt for Member {
    fn has_admin<U, E>(&self, ctx: &Context<U, E>) -> bool {
        self.has_permission(&ctx, |p| p.administrator())
    }

    fn has_permission<U, E>(&self, ctx: &Context<U, E>, mut check: impl FnMut(Permissions) -> bool) -> bool {
        if let Some(roles) = self.roles(ctx) {
            roles.iter().any(|user|check(user.permissions))
        } else {
            false
        }
    }
}
