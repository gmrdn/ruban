use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Cli {
    Add {
        #[structopt(default_value = "", short = "t", long = "tags")]
        tags: String,
        description: String,
    },
    Ls {},
    Rm {
        number: u32,
    },
    Mv {},
}
