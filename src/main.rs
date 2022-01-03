use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::model::AttributeType;
use aws_sdk_cognitoidentityprovider::{Client, Error};
use aws_sdk_config::Region;
use clap::Parser;
use clap::{App, IntoApp, Subcommand};
use clap_complete::{generate, Shell};
use std::io;
use std::process::exit;
use std::str::FromStr;

#[derive(Subcommand)]
enum Commands {
    /// generate auto complete file e.g. --generate=bash
    Generate { target: String },
}
#[derive(Parser)]
#[clap(about, version, author, name = "cog-new-user")]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// cognito client id
    #[clap(long)]
    pub client_id: Option<String>,

    /// cognito user pool id
    #[clap(long)]
    pub user_pool_id: Option<String>,

    /// username to create
    #[clap(long)]
    pub username: Option<String>,

    /// optional email for user
    #[clap(long)]
    pub email: Option<String>,

    /// user's password
    #[clap(long)]
    pub password: Option<String>,

    /// aws region or use profile default
    #[clap(long)]
    pub region: Option<String>,

    /// aws profile or load from env.
    #[clap(long)]
    pub profile: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Args = Args::parse();
    let mut app: App = Args::into_app();
    if let Some(command) = args.command {
        match &command {
            Commands::Generate { target } => {
                let shell = Shell::from_str(target.as_str()).unwrap();
                let app_name = (&mut app).get_name().to_string();
                generate(shell, &mut app, app_name, &mut io::stdout());
                exit(0)
            }
        }
    }

    if let Some(pro) = args.profile {
        std::env::set_var("AWS_PROFILE", pro)
    }
    let region_provider =
        RegionProviderChain::first_try(args.region.map(Region::new)).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let client_id = args.client_id.unwrap_or_else(|| {
        app.print_help().unwrap();
        exit(1)
    });
    let username = args.username.unwrap_or_else(|| {
        app.print_help().unwrap();
        exit(1)
    });
    let password = args.password.unwrap_or_else(|| {
        app.print_help().unwrap();
        exit(1)
    });
    let user_pool_id = args.user_pool_id.unwrap_or_else(|| {
        app.print_help().unwrap();
        exit(1)
    });

    let mut sign_up_pre = client
        .sign_up()
        .client_id(&client_id)
        .username(&username)
        .password(&password);
    if let Some(email) = args.email {
        let email_at = AttributeType::builder().name("email").value(email).build();
        sign_up_pre = sign_up_pre.user_attributes(email_at);
    }

    sign_up_pre.send().await?;

    client
        .admin_confirm_sign_up()
        .user_pool_id(&user_pool_id)
        .username(&username)
        .send()
        .await?;

    Ok(())
}
