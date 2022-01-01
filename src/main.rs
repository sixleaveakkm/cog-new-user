mod args;

use aws_config::meta::region::RegionProviderChain;
use clap::{Parser};
use aws_sdk_config::{Region};
use aws_sdk_cognitoidentityprovider::{Client, Error};
use aws_sdk_cognitoidentityprovider::model::AttributeType;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: args::Args = args::Args::parse();

    if let Some(pro) = args.profile {
        std::env::set_var("AWS_PROFILE", pro)
    }
    let region_provider = RegionProviderChain::first_try(args.region.map(Region::new))
            .or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);


    let mut sign_up_pre = client.sign_up().
            client_id(&args.client_id).
            username(&args.username).
            password(&args.password);
    if let Some(email) = args.email {
        let email_at = AttributeType::builder().name("email").value(email).build();
        sign_up_pre = sign_up_pre.user_attributes(email_at);
    }

    sign_up_pre.send().await?;

    client.admin_confirm_sign_up().user_pool_id(&args.user_pool_id).username(&args.username).send().await?;

    Ok(())
}
