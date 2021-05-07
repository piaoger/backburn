use structopt::StructOpt;


/// A simple archive zip/unzip tool
#[derive(StructOpt, Debug)]
#[structopt(name = "backburn")]
enum CmdArgs {
    /// zip directory: from (dir) -> to (zip)
    #[structopt(name = "metadata")]
    Metadata {
        /// dry run mode
        #[structopt(short, long = "v")]
        vorbose: bool,
    },
}

fn main() {
    let opt = CmdArgs::from_args();
    match opt {
        CmdArgs::Metadata {
            vorbose,
        } => {
            display_metadata(vorbose);
        }

        _ => {
           println!("unknown action");
        }
    };
}


fn display_metadata(vorbose:bool) {
   backburn::metadata::fetch_metadata("aliyun");
}
