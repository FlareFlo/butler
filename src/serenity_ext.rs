use poise::Context;
use serenity::all::Member;

pub trait SerenityExt {
    fn has_admin<U, E>(&self, ctx: &Context<U, E>) -> bool;
}

impl SerenityExt for Member {
    fn has_admin<U, E>(&self, ctx: &Context<U, E>) -> bool {
        if let Some(roles) = self.roles(ctx) {
            roles.iter().any(|r| r.permissions.MODERATE_MEMBERS())
        } else {
            false
        }
    }
}
