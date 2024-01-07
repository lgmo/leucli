use crate::context::Context;

mod git_handler;

pub fn handle_context(context: Context) -> Result<(), &'static str> {
    if context.args.len() < 2 {
        return Ok(());
    }

    let command = &context.args[1];

    if command == "pull" {
        git_handler::handle_pull(context)?;
    } else if command == "wip" {
        git_handler::handle_wip(context)?;
    } else if command == "ucommit" || command == "undo-commit" {
        git_handler::handle_ucommit(context)?;
    } else if command == "commit" {
        git_handler::handle_commit(context)?;
    } else if command == "push" {
        git_handler::handle_push(context)?
    } else if command == "gremlin" {
        git_handler::handle_gremlin(context)?;
    } else if command == "down" {
        git_handler::handle_down(context)?;
    } else if command == "rbranch" || command == "rename-branch" {
        git_handler::handle_rbranch(context)?;
    }

    Ok(())
}
