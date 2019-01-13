// LIFETIME SUBTYPING
pub struct Context<'s>(&'s str);

pub struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    pub fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

pub fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

// LIFETIME BOUNDS

pub struct Ref<'a, T: 'a>(&'a T); // Any references that T holds should live as long as 'a

pub struct StaticRef<T: 'static>(&'static T); // All references T holds must have the 'static lifetime


// INFERENCE OF TRAIT OBJECT LIFETIMES

// The default lifetime of a trait object is 'static.
// With &'a Trait or &'a mut Trait, the default lifetime of the trait object is 'a.
// With a single T: 'a clause, the default lifetime of the trait object is 'a.
// With multiple clauses like T: 'a, there is no default lifetime; we must be explicit.

trait Red { }

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> { }

fn n() {
    let num = 5;

    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

fn m<'a>(num: &'a i32) {
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red + 'a>;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

// ANONYMOUS LIFETIME

struct StrWrap<'a>(&'a str);

fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}

fn foo_alt(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

/* opt1
impl<'a> std::fmt::Debug for StrWrap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
*/

// opt2 = same as opt1
impl std::fmt::Debug for StrWrap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
