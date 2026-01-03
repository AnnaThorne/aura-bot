pub fn setup_commands() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![aura::aura()],
            pre_command: |ctx, msg, command_name| {
                Box::pin(async move {
                    log::info!("User {} invoked command: {}", msg.author.name, command_name);
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    log::error!("Error running command: {:?}", error);
                })
            },
            ..Default::default()
        })
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
}
