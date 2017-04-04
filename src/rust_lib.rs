use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use syn;

/*
 * Recursively parses a rust library starting at the root crate's directory.
 *
 * Inside a crate, `mod` and `extern crate` declarations are followed
 * and parsed. To find an external crate, the parser looks in the
 * parent directory of the root crate. This could be improved
 * to read the crate's Cargo.toml to find the crate, but it works well
 * enough to find the crates that matter for creating bindings.
 */
pub fn parse<F>(crate_dir: String,
                items_callback: &F)
    where F: Fn(String, &Vec<syn::Item>)
{
    parse_crate(PathBuf::from(crate_dir),
                items_callback);
}

fn parse_crate<F>(crate_dir: PathBuf,
                  items_callback: &F)
    where F: Fn(String, &Vec<syn::Item>)
{
    parse_mod(crate_dir.clone(),
              crate_dir.join("src/lib.rs"),
              items_callback);
}

fn parse_mod<F>(crate_dir: PathBuf,
                mod_path: PathBuf,
                items_callback: &F)
    where F: Fn(String, &Vec<syn::Item>)
{
    let mod_dir = mod_path.parent().unwrap().to_path_buf();
    let mod_parsed = {
        let mut s = String::new();
        let mut f = File::open(mod_path).unwrap();
        f.read_to_string(&mut s).unwrap();
        syn::parse_crate(&s).unwrap()
    };

    let crate_name = crate_dir.file_name().unwrap().to_str().unwrap();

    items_callback(String::from(crate_name),
                   &mod_parsed.items);

    for item in &mod_parsed.items {
        match item.node {
            syn::ItemKind::Mod(ref inline_items) => {
                let next_mod_name = item.ident.to_string();

                if let &Some(ref inline_items) = inline_items {
                    items_callback(String::from(crate_name),
                                   &inline_items);
                    continue;
                }

                let next_mod_path1 = mod_dir.join(next_mod_name.clone() + ".rs");
                let next_mod_path2 = mod_dir.join(next_mod_name.clone()).join("mod.rs");

                if next_mod_path1.exists() {
                    parse_mod(crate_dir.clone(),
                              next_mod_path1,
                              items_callback);
                } else if next_mod_path2.exists() {
                    parse_mod(crate_dir.clone(),
                              next_mod_path2,
                              items_callback);
                } else {
                    writeln!(io::stderr(), "warning, can't find a mod's file").unwrap();
                }
            }
            syn::ItemKind::ExternCrate(_) => {
                let crate_parent = crate_dir.parent().unwrap();

                let next_crate_name = item.ident.to_string();
                let next_crate_path = crate_parent.join(next_crate_name.clone());

                if !next_crate_path.exists() {
                    writeln!(io::stderr(), "warning, can't find an extern crate {}", next_crate_name.clone()).unwrap();
                    continue;
                }

                parse_crate(next_crate_path,
                            items_callback);
            }
            _ => {}
        }
    }
}
