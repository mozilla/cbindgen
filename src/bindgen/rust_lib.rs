use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use bindgen::cargo_expand;
use bindgen::cargo_metadata;
use syn;

/// Parses a single rust source file, not following `mod` or `extern crate`.
pub fn parse_src<F>(src_file: &Path,
                    items_callback: &mut F)
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let src_parsed = {
        let mut s = String::new();
        let mut f = File::open(src_file).unwrap();
        f.read_to_string(&mut s).unwrap();
        syn::parse_crate(&s).unwrap()
    };

    items_callback("", &src_parsed.items);
}

/// Recursively parses a rust library starting at the root crate's directory.
///
/// Inside a crate, `mod` and `extern crate` declarations are followed
/// and parsed. To find an external crate, the parser uses the `cargo metadata`
/// command to find the location of dependencies.
pub fn parse_lib<F>(crate_path: &Path,
                    binding_crate_name: &str,
                    items_callback: &mut F)
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let manifest_path = crate_path.join("Cargo.toml");
    let metadata = match cargo_metadata::metadata(Some(manifest_path.to_str().unwrap())) {
        Result::Ok(metadata) => metadata,
        Result::Err(err) => {
            panic!("error executing `cargo metadata`: {:?}", err);
        }
    };

    let mut context = ParseLibContext {
      manifest_path: manifest_path,
      metadata: metadata,
      cache_src: HashMap::new(),
      cache_expanded_crate: HashMap::new(),
      items_callback: items_callback,
    };

    parse_crate(binding_crate_name, &mut context);
}

struct ParseLibContext<F>
  where F: FnMut(&str, &Vec<syn::Item>)
{
  manifest_path: PathBuf,
  metadata: cargo_metadata::Metadata,
  cache_src: HashMap<PathBuf, Vec<syn::Item>>,
  cache_expanded_crate: HashMap<String, Vec<syn::Item>>,

  items_callback: F,
}

impl<F> ParseLibContext<F>
  where F: FnMut(&str, &Vec<syn::Item>)
{
  fn find_crate_src(&self, package_name: &str) -> Option<PathBuf> {
      let kind_lib = String::from("lib");
      for package in &self.metadata.packages {
          if package.name == package_name {
              for target in &package.targets {
                  if target.kind.contains(&kind_lib) {
                      return Some(PathBuf::from(&target.src_path));
                  }
              }
              break;
          }
      }
      None
  }
}

fn parse_crate<F>(crate_name: &str, context: &mut ParseLibContext<F>)
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let crate_src = context.find_crate_src(crate_name);

    match crate_src {
        Some(crate_src) => {
            parse_mod(crate_name,
                      crate_src.as_path(),
                      context);
        }
        None => info!("can't find crate {}", crate_name),
    }
}

fn parse_expand_crate<F>(crate_name: &str, context: &mut ParseLibContext<F>)
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let mod_parsed = {
        let owned_crate_name = crate_name.to_owned();

        if !context.cache_expanded_crate.contains_key(&owned_crate_name) {
            let s = cargo_expand::expand(&context.manifest_path,
                                         crate_name);
            let i = syn::parse_crate(&s).unwrap();
            context.cache_expanded_crate.insert(owned_crate_name.clone(), i.items);
        }
        context.cache_expanded_crate.get(&owned_crate_name).unwrap().clone()
    };

    (context.items_callback)(crate_name,
                             &mod_parsed);

    for item in &mod_parsed {
        match item.node {
            syn::ItemKind::Mod(ref inline_items) => {
                if let &Some(ref inline_items) = inline_items {
                    (context.items_callback)(crate_name,
                                             &inline_items);
                    continue;
                }

                warn!("external mod found in expanded source");
            }
            syn::ItemKind::ExternCrate(_) => {
                parse_expand_crate(&item.ident.to_string(),
                                   context);
            }
            _ => {}
        }
    }
}

fn parse_mod<F>(crate_name: &str,
                mod_path: &Path,
                context: &mut ParseLibContext<F>)
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let mod_dir = mod_path.parent().unwrap().to_path_buf();

    let mod_parsed = {
        let owned_mod_path = mod_path.to_path_buf();

        if !context.cache_src.contains_key(&owned_mod_path) {
            let mut s = String::new();
            let mut f = File::open(mod_path).unwrap();
            f.read_to_string(&mut s).unwrap();
            let i = syn::parse_crate(&s).unwrap();
            context.cache_src.insert(owned_mod_path.clone(), i.items);
        }
        context.cache_src.get(&owned_mod_path).unwrap().clone()
    };

    (context.items_callback)(crate_name,
                             &mod_parsed);

    for item in &mod_parsed {
        match item.node {
            syn::ItemKind::Mod(ref inline_items) => {
                let next_mod_name = item.ident.to_string();

                if let &Some(ref inline_items) = inline_items {
                    (context.items_callback)(crate_name,
                                             &inline_items);
                    continue;
                }

                let next_mod_path1 = mod_dir.join(next_mod_name.clone() + ".rs");
                let next_mod_path2 = mod_dir.join(next_mod_name.clone()).join("mod.rs");

                if next_mod_path1.exists() {
                    parse_mod(crate_name,
                              next_mod_path1.as_path(),
                              context);
                } else if next_mod_path2.exists() {
                    parse_mod(crate_name,
                              next_mod_path2.as_path(),
                              context);
                } else {
                    info!("can't find mod {} in crate {}", next_mod_name, crate_name);
                }
            }
            syn::ItemKind::ExternCrate(_) => {
                parse_expand_crate(&item.ident.to_string(),
                                   context);
            }
            _ => {}
        }
    }
}
