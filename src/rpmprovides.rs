
use cargo_lock::Lockfile;

/// Run the rpm provides generator. This will take a Cargo.lock
/// (generally put in place by the macros.cargo file)

pub fn exec() {
    for line in std::io::stdin().lines() {
        let uline = line.unwrap();
        let lockfile = Lockfile::load(uline)
            .expect("Unable to process lockfile");

        for pkg in lockfile.packages.iter() {
            // Rpm doesn't like - in version.
            let version = pkg.version.to_string().replace("-", "_");
            println!("rust-crate({}) = {}", pkg.name, version);
        }
    }
}


// [  349s] warning: Invalid version (double separator '-'): 0.9.0+wasi-snapshot-preview1
// [  349s] warning: Invalid version (double separator '-'): 0.10.0+wasi-snapshot-preview1
// [  349s] warning: Invalid version (double separator '-'): 0.11.0+wasi-snapshot-preview1
// [  349s] Provides: cargo-packaging-provides = 99999
