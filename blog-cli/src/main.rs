//! CLI-инструмент для управления блогом.

use crate::cli::{AuthCommand, Cli, Command, PostsCommand};
use crate::config::CliConfig;
use crate::output::{
    print_deleted, print_error, print_logged_in, print_post, print_post_page, print_registered,
};
use crate::token_store::{read_token, write_token};
use blog_client::BlogClient;
use clap::Parser;
use std::path::Path;

mod cli;
mod config;
mod output;
mod token_store;

fn require_token(path: &Path) -> anyhow::Result<String> {
    read_token(path)?.ok_or_else(|| anyhow::anyhow!("token is required: run auth login first"))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    if let Err(error) = run(cli).await {
        print_error(&error, json);
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let command = cli.command;

    let config = CliConfig::from_args(cli.grpc, cli.server, cli.token_file, cli.json);

    let mut client = BlogClient::new(config.transport).await?;

    match command {
        Command::Auth { command } => match command {
            AuthCommand::Register {
                username,
                email,
                password,
            } => {
                let response = client.register(&username, &email, &password).await?;
                write_token(&config.token_file, &response.token)?;
                print_registered(&response, config.json)?;
            }
            AuthCommand::Login { username, password } => {
                let response = client.login(&username, &password).await?;
                write_token(&config.token_file, &response.token)?;
                print_logged_in(&response, config.json)?;
            }
        },

        Command::Posts { command } => match command {
            PostsCommand::Create { title, content } => {
                let token = require_token(&config.token_file)?;
                client.set_token(token);

                let post = client.create_post(&title, &content).await?;
                print_post(&post, config.json)?;
            }
            PostsCommand::Get { id } => {
                let post = client.get_post(id).await?;
                print_post(&post, config.json)?;
            }
            PostsCommand::Update { id, title, content } => {
                let token = require_token(&config.token_file)?;
                client.set_token(token);

                let post = client.update_post(id, &title, &content).await?;
                print_post(&post, config.json)?;
            }
            PostsCommand::Delete { id } => {
                let token = require_token(&config.token_file)?;
                client.set_token(token);

                client.delete_post(id).await?;
                print_deleted(id, config.json)?;
            }
            PostsCommand::List { limit, offset } => {
                let page = client.list_posts(limit, offset).await?;
                print_post_page(&page, config.json)?;
            }
        },
    }

    Ok(())
}
