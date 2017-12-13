use syn;

#[derive(Debug, Default)]
pub struct Attributes<'a> {
    pub array_length: Option<&'a syn::Lit>,
    pub parameters: Option<&'a syn::Lit>,
}

impl<'a> Attributes<'a> {
    fn with_params(&mut self, lit: &'a syn::Lit) {
        if self.parameters.is_some() {
            panic!("multiple parameters declared");
        }

        self.parameters = Some(lit);
    }

    fn array_length(&mut self, lit: &'a syn::Lit) {
        if self.array_length.is_some() {
            panic!("multiple array declaration detected");
        }

        self.array_length = Some(lit);
    }
}

pub fn decl_field_attributes(attrs: &[syn::Attribute]) -> Attributes {
    use syn::MetaItem;
    use syn::Ident;

    // here we filter through declarative attributes, extracting the inner values.
    let attrs = attrs
        .iter()
        .filter_map(|attr| {
            use syn::MetaItem;
            if let MetaItem::List(ref ident, ref items) = attr.value {
                if ident == "declarative" {
                    Some(&items[..])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flat_map(|items| items);

    let mut parsed = Attributes::default();
    for attr in attrs {
        match *attr {
            syn::NestedMetaItem::MetaItem(syn::MetaItem::NameValue(ref ident, ref value)) => {
                match ident.as_ref() {
                    "length" => parsed.array_length(value),
                    "arguments" => parsed.with_params(value),
                    _ => panic!("unrecognized field attribute"),
                }
            }
            _ => panic!("unrecognized field attribute"),
        }
    }

    parsed
}
