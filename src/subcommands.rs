#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

pub fn audit() -> std::io::Result<()> {
    use abscissa_core::application::Application;
    use cargo_audit::application::{CargoAuditApplication, APPLICATION};

    CargoAuditApplication::run(&APPLICATION, vec![String::from("audit")]);
    Ok(())
}
