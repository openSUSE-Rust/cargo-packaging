
use std::path::Path;
use tracing::debug;
use auditable_info::{
    audit_info_from_file,
    Error
};

/// Run the rpm provides generator. This takes an elf and parses it for the
/// appropriate rust audit header, if any.

pub fn exec() {
    for line in std::io::stdin().lines() {
        match line
            .map_err(Error::Io)
            .and_then(|uline| {
                let p = Path::new(&uline);
                audit_info_from_file(&p, Default::default()) 
            })
            {
            Ok(info) => {
                for pkg in info.packages.iter() {
                    // Rpm doesn't like - in version.
                    let version = pkg.version.to_string().replace("-", "_");
                    println!("bundled(rust-crate:{}) = {}", pkg.name, version);
                }
            }
            Err(Error::NoAuditData) => {
                // May not be a rust binary, skip.
            }
            Err(e) => {
                debug!(?e);
                println!("bundled(rust-crate:BROKEN) = 9999.0.0")
            }
        }
    }
}

