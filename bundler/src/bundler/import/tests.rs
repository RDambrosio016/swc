use super::ImportHandler;
use crate::bundler::tests::suite;
use std::path::Path;
use swc_common::FileName;
use swc_ecma_visit::FoldWith;

#[test]
#[ignore]
fn ns_import_deglob_simple() {
    suite().run(|t| {
        let m = t.parse(
            "
import * as ns from 'foo';
ns.foo();
",
        );
        let mut v = ImportHandler {
            path: &FileName::Real(Path::new("index.js").to_path_buf()),
            bundler: &t.bundler,
            top_level: false,
            info: Default::default(),
            forces_ns: Default::default(),
            ns_usage: Default::default(),
            deglob_phase: false,
        };
        let m = m.fold_with(&mut v);
        assert!(v.forces_ns.is_empty());
        assert_eq!(v.info.imports.len(), 1);

        t.assert_eq(&m, "foo();");

        Ok(())
    })
}

#[test]
#[ignore]
fn ns_import_deglob_multi() {
    suite().run(|t| {
        let m = t.parse(
            "
import * as ns from 'foo';
ns.foo();
ns.bar();
",
        );
        let mut v = ImportHandler {
            path: &FileName::Real(Path::new("index.js").to_path_buf()),
            bundler: &t.bundler,
            top_level: false,
            info: Default::default(),
            forces_ns: Default::default(),
            ns_usage: Default::default(),
            deglob_phase: false,
        };
        let m = m.fold_with(&mut v);
        assert!(v.forces_ns.is_empty());
        assert_eq!(v.info.imports.len(), 1);
        assert_eq!(v.info.imports[0].specifiers.len(), 2);
        assert!(!format!("{:?}", v.info.imports[0].specifiers).contains("ns"));

        t.assert_eq(
            &m,
            "foo();
bar();",
        );

        Ok(())
    })
}
