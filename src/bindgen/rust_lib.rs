use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use bindgen::cargo_expand;
use bindgen::cargo_metadata;
use syn;

const STD_CRATES: &[&'static str] = &["std",
                                      "std_unicode",
                                      "alloc",
                                      "collections",
                                      "core",
                                      "proc_macro"];

type ParseResult = Result<(), String>;

/// Parses a single rust source file, not following `mod` or `extern crate`.
pub fn parse_src<F>(src_file: &Path,
                    items_callback: &mut F) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let src_parsed = {
        let mut s = String::new();
        let mut f = File::open(src_file).map_err(|_| format!("parsing: cannot open file `{:?}`", src_file))?;
        f.read_to_string(&mut s).map_err(|_| format!("parsing: cannot open file `{:?}`", src_file))?;
        syn::parse_crate(&s).map_err(|msg| format!("parsing:\n{}", msg))?
    };

    items_callback("", &src_parsed.items);

    Ok(())
}

/// Recursively parses a rust library starting at the root crate's directory.
///
/// Inside a crate, `mod` and `extern crate` declarations are followed
/// and parsed. To find an external crate, the parser uses the `cargo metadata`
/// command to find the location of dependencies.
pub fn parse_lib<F>(crate_path: &Path,
                    binding_crate_name: &str,
                    expand: &[String],
                    items_callback: &mut F) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let manifest_path = crate_path.join("Cargo.toml");

    let metadata = match cargo_metadata::metadata(Some(manifest_path.to_str().unwrap())) {
        Ok(metadata) => metadata,
        Err(msg) => {
            return Err(format!("executing `cargo metadata`: {:?}", msg));
        }
    };

    let mut context = ParseLibContext {
        manifest_path: manifest_path,
        metadata: metadata,
        expand: expand.to_owned(),
        cache_src: HashMap::new(),
        cache_expanded_crate: HashMap::new(),
        items_callback: items_callback,
    };

    parse_crate(binding_crate_name, &mut context)
}

struct ParseLibContext<F>
  where F: FnMut(&str, &Vec<syn::Item>)
{
    manifest_path: PathBuf,
    metadata: cargo_metadata::Metadata,
    expand: Vec<String>,
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

fn parse_crate<F>(crate_name: &str, context: &mut ParseLibContext<F>) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    if STD_CRATES.contains(&crate_name) {
        return Ok(());
    }

    if context.expand.contains(&crate_name.to_owned()) {
        return parse_expand_crate(crate_name, context);
    }

    let crate_src = context.find_crate_src(crate_name);

    match crate_src {
        Some(crate_src) => {
            parse_mod(crate_name, crate_src.as_path(), context)
        },
        None => {
            // This should be an error, but is common enough to just elicit a warning
            warn!("parsing crate `{}`: can't find lib.rs with `cargo metadata`", crate_name);
            Ok(())
        },
    }
}

fn parse_expand_crate<F>(crate_name: &str, context: &mut ParseLibContext<F>) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let mod_parsed = {
        let owned_crate_name = crate_name.to_owned();

        if !context.cache_expanded_crate.contains_key(&owned_crate_name) {
            let s = cargo_expand::expand(&context.manifest_path, crate_name)?;
            let i = syn::parse_crate(&s).map_err(|msg| format!("parsing crate `{}`:\n{}", crate_name, msg))?;
            context.cache_expanded_crate.insert(owned_crate_name.clone(), i.items);
        }

        context.cache_expanded_crate.get(&owned_crate_name).unwrap().clone()
    };

    process_expanded_mod(crate_name, &mod_parsed, context)
}

fn process_expanded_mod<F>(crate_name: &str,
                           items: &Vec<syn::Item>,
                           context: &mut ParseLibContext<F>) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    (context.items_callback)(crate_name, items);

    for item in items {
        match item.node {
            syn::ItemKind::Mod(ref inline_items) => {
                if let &Some(ref inline_items) = inline_items {
                    process_expanded_mod(crate_name, inline_items, context)?;
                    continue;
                }

                return Err(format!("parsing crate `{}`: external mod found in expanded source", crate_name));
            }
            syn::ItemKind::ExternCrate(_) => {
                parse_crate(&item.ident.to_string(), context)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn parse_mod<F>(crate_name: &str,
                mod_path: &Path,
                context: &mut ParseLibContext<F>) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    let mod_parsed = {
        let owned_mod_path = mod_path.to_path_buf();

        if !context.cache_src.contains_key(&owned_mod_path) {
            let mut s = String::new();
            let mut f = File::open(mod_path).map_err(|_| format!("parsing crate `{}`: cannot open file `{:?}`", crate_name, mod_path))?;
            f.read_to_string(&mut s).map_err(|_| format!("parsing crate `{}`: cannot open file `{:?}`", crate_name, mod_path))?;
            let i = syn::parse_crate(&s).map_err(|msg| format!("parsing crate `{}`:\n{}", crate_name, msg))?;
            context.cache_src.insert(owned_mod_path.clone(), i.items);
        }

        context.cache_src.get(&owned_mod_path).unwrap().clone()
    };

    let mod_dir = mod_path.parent().unwrap();

    process_mod(crate_name,
                mod_dir,
                &mod_parsed,
                context)
}

fn process_mod<F>(crate_name: &str,
                  mod_dir: &Path,
                  items: &Vec<syn::Item>,
                  context: &mut ParseLibContext<F>) -> ParseResult
    where F: FnMut(&str, &Vec<syn::Item>)
{
    (context.items_callback)(crate_name, items);

    for item in items {
        match item.node {
            syn::ItemKind::Mod(ref inline_items) => {
                let next_mod_name = item.ident.to_string();

                if let &Some(ref inline_items) = inline_items {
                    process_mod(crate_name,
                                mod_dir,
                                inline_items,
                                context)?;
                    continue;
                }

                let next_mod_path1 = mod_dir.join(next_mod_name.clone() + ".rs");
                let next_mod_path2 = mod_dir.join(next_mod_name.clone()).join("mod.rs");

                if next_mod_path1.exists() {
                    parse_mod(crate_name,
                              next_mod_path1.as_path(),
                              context)?;
                } else if next_mod_path2.exists() {
                    parse_mod(crate_name,
                              next_mod_path2.as_path(),
                              context)?;
                } else {
                    // This should be an error, but is common enough to just elicit a warning
                    warn!("parsing crate `{}`: can't find mod {}`", crate_name, next_mod_name);
                }
            }
            syn::ItemKind::ExternCrate(_) => {
                parse_crate(&item.ident.to_string(), context)?;
            }
            _ => {}
        }
    }

    Ok(())
}
