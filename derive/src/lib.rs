extern crate proc_macro;

use proc_macro::TokenStream;

///
/// Generates methods for struct members of type Option<String>,
/// supporting multi-level nesting.
///
/// ```compile_fail
/// use zepub_mini_derive::option_string_method;
///
/// // Accessing member: self.info.k
/// option_string_method!(info, k);
///
/// // Accessing member: self.k
/// option_string_method!(k);
/// ```
#[proc_macro]
pub fn option_string_method(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let v: Vec<&str> = s.split(',').collect();

    let m = r#"pub fn set_{method}<T:  AsRef<str>>(&mut self, {method}: T) {
        if let Some( c) = &mut self.{prefix}{method} {
            c.clear();
            c.push_str({method}.as_ref());
        } else {
            self.{prefix}{method} = Some(String::from({method}.as_ref()));
        }
    }
    pub fn with_{method}<T:  AsRef<str>>(mut self, {method}: T) ->Self {
        self.set_{method}({method}.as_ref());
        self
    }
    pub fn {method}(&self) -> Option<&str> {
        self.{prefix}{method}.as_ref().map(|x|x.as_str())
    }"#;
    if v.len() == 2 {
        let r = m
            .replace("{prefix}", format!("{}.", v[0].trim()).as_str().trim())
            .replace("{method}", v[1].trim());
        return r.parse().unwrap();
    } else if v.len() == 1 {
        return m
            .replace("{prefix}", "")
            .replace("{method}", v[0].trim())
            .parse()
            .unwrap();
    }
    "".parse().unwrap()
}
