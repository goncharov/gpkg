use super::Command;
use crate::config::Config;
use crate::install_package;
use crate::node_package_version::NodePackageVersion;
use colored::*;
use log::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Install {
    /// The npm package to install. Takes the format of `name[@version]`
    package: NodePackageVersion,
}

impl Command for Install {
    type Error = install_package::Errors;

    fn apply(self, config: Config) -> Result<(), Self::Error> {
        debug!("Installing package {:?}", &self.package);
        install_package::install_package(&self.package, &config)?;
        Ok(())
    }

    fn handle_error(err: Self::Error) {
        match err {
            Self::Error::PackageAlreadyInstalled(pkg) => {
                let err_str =
                    format!("Package {} is already installed.", pkg.underline().italic()).red();
                eprintln!("{}", err_str);
            }
            err => {
                dbg!(err);
                unimplemented!();
            }
        }
    }
}
