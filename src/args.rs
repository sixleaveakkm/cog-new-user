#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
	#[clap(long)]
	pub client_id: String,

	#[clap(long)]
	pub user_pool_id: String,

	#[clap(long)]
	pub username: String,

	#[clap(long)]
	pub email: Option<String>,

	#[clap(long)]
	pub password: String,

	#[clap(long)]
	pub region: Option<String>,

	#[clap(long)]
	pub profile: Option<String>,
}
