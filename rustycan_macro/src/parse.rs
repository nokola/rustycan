use syn::{
    parenthesized,
    parse::{Parse, ParseStream, Result},
    token, Expr, Ident, LitStr, Token,
};

/// Syntax of rustycan_ui! macro:
///
/// ```ignore
/// rustycan_ui! {
///    $Elem
/// }
/// ```
///
/// Syntax of $Elem:
/// ```ignore
/// $name "text"? (
///    $( $propertyName:Ident = $value:PropertyValue )*
///    $( $child:Elem )*
/// )
/// ```
///
/// TODO:
/// Syntax for $PropertyValue:
/// ```ignore
///     $Size       // examples: 40 auto 1x _ 20.._  _..60 10.._..70 10..40..70
///     "string"    // "hello"
///     $Range      // (1..10)
///     $List       // (10 1x 2 auto)
///     $Object     // {sizes=(50 2x 1x) between=40 before_first=10 after_last=20}
/// ```
///
/// Syntax for $Size:
/// ```ignore
///     $number     // absolute value in logical pixels
///     auto        // fit to content
///     $(number)x  // fractional sizing
///     $(number)%  // percent sizing
///     _           // default
///     20.._       // min
///     _..60       // max
///     10..50      // min and max
///     10..1x..50  // min and max with a scale unit
///     10..auto..50 // min and max with auto size to content
/// ```
///
/// Syntax for $Range:
/// ```ignore
///     (number..number)
/// ```
///
/// /// Syntax for $List:
/// ```ignore,
///     ( $(Size,)* )
/// ```
///
/// /// Syntax for $Object:
/// ```ignore
///     {
///          $( $propertyName:Ident = $value:PropertyValue )*
///     }
/// ```
/// TODO: TEST: "Ok" after elem
/// TODO: .ok in various places
/// TODO: TEST: empty Elem
/// TODO: // comments

pub struct UiInfo {
    /// Name of identifier to use for ui calls, usually "ui". For egui, the calls may look like `ui.label(...)`
    pub ui_name: Ident,
    pub root: Elem,
}

impl Parse for UiInfo {
    fn parse(input: ParseStream) -> Result<Self> {
        let ui_name: Ident = input.parse()?;
        let root = input.parse::<Elem>()?;

        Ok(UiInfo { ui_name, root })
    }
}

pub struct Elem {
    pub name: Ident,
    pub text: Option<LitStr>,
    pub params: Vec<ElemParam>,
}

impl Parse for Elem {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        // check for "text" case, example: Button "Ok"
        let text: Option<LitStr> = if input.peek(LitStr) {
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };

        // if no (, this must be the end of the element, for example: Button "Ok"
        if !input.peek(token::Paren) {
            return Ok(Elem {
                name,
                text,
                params: Vec::new(),
            });
        }

        let content;
        parenthesized!(content in input); // parse content inside "(" and ")"
        let params: ElemParamList = content.parse()?;
        Ok(Elem {
            name,
            text,
            params: params.0,
        })
    }
}

pub enum ElemParam {
    Property(ElemProperty),
    ChildElem(Elem),
}

pub struct ElemParamList(Vec<ElemParam>);

impl Parse for ElemParamList {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut params: Vec<ElemParam> = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }
            let param = ElemParam::parse(input)?;
            params.push(param);
        }

        Ok(ElemParamList(params))
    }
}

impl Parse for ElemParam {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Token![=]) {
            input.parse().map(ElemParam::Property)
        } else if input.peek2(token::Paren) || input.peek2(LitStr) {
            input.parse().map(ElemParam::ChildElem)
        } else {
            let name = input.parse::<Ident>()?; // advance to show error at correct location

            // if just identifier, then its a single empty child (e.g. HorizontalLine)
            if input.is_empty() {
                return Ok(ElemParam::ChildElem(Elem {
                    name,
                    text: None,
                    params: Vec::new(),
                }));
            }

            Err(input.error("Expected element or property"))
        }
    }
}

pub struct ElemProperty {
    pub name: Ident,
    pub value: Expr,
}

impl Parse for ElemProperty {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;
        Ok(ElemProperty { name, value })
    }
}
