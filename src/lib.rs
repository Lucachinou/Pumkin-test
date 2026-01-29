use std::sync::Arc;
use async_trait::async_trait;
use pumpkin_api_macros::{plugin_impl, plugin_method, with_runtime};
use pumpkin::{command::{args::ConsumedArgs, dispatcher::CommandError, tree::builder::literal, tree::CommandTree, CommandExecutor, CommandSender}, plugin::{player::player_join::PlayerJoinEvent, Context, EventHandler, EventPriority}, server::Server, PumpkinServer};
use pumpkin::command::CommandResult;
use pumpkin::plugin::BoxFuture;
use pumpkin_util::permission::{Permission, PermissionDefault};
use pumpkin_util::PermissionLvl;
use pumpkin_util::text::{color::NamedColor, TextComponent};

struct JoinHandler;

#[with_runtime(global)]
impl EventHandler<PlayerJoinEvent> for JoinHandler {
    fn handle_blocking<'a>(&'a self, _server: &'a Arc<Server>, event: &'a mut PlayerJoinEvent) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            event.join_message = TextComponent::text(format!("{} joined the server!", event.player.gameprofile.name))
                .color_named(NamedColor::White);
            log::info!("{} joined the server!", event.player.gameprofile.name);
        })
    }
}

struct SpawnCommandExecutor;

impl CommandExecutor for SpawnCommandExecutor {
    fn execute<'a>(&'a self, sender: &'a CommandSender, _: &Server, _args: &'a ConsumedArgs<'a>, ) -> CommandResult<'a> {
        Box::pin(async move {
            sender.send_message(TextComponent::text("hello!")).await;

            Ok(())
        })
    }
}



#[plugin_method]
async fn on_load(&mut self, server: Arc<Context>) -> Result<(), String> {
    server.init_log();

    log::info!("Loading plugin..");

    log::info!("Registering events..");
    server.register_event(Arc::new(JoinHandler), EventPriority::Normal, true).await;

    log::info!("Registering permissions..");
    server.register_permission(Permission::new("pumpkin_plugin:hello", "Says hello", PermissionDefault::Allow)).await.unwrap();

    log::info!("Registering commands..");

    let command = CommandTree::new(["hello"], "hello, world!").execute(SpawnCommandExecutor);

    server.register_command(command, "hello").await;

    log::info!("Pumpkin plugin loaded!");
    Ok(())
}

#[plugin_impl]
pub struct Plugin {}

impl Plugin {
    pub fn new() -> Self {
        Plugin {}
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}