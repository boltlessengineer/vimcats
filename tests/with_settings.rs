use vimcats::{vimdoc::VimDoc, FromLuaCATS, VimCats, Settings};

const CODE: &str = r#"
local U = {}

---@alias ID string

---@class User
---@field name string
---@field email string
---@field id ID

---A Pi
---@type number
U.Pi = 3.14

---Creates a PI
---@return number
---@usage `require('Pi'):create()`
function U:create()
    return self.Pi
end

return U
"#;

#[test]
fn rename_with_return() {
    let mut vimcats = VimCats::new();
    let s = Settings {
        prefix_func: true,
        prefix_alias: true,
        prefix_class: true,
        prefix_type: true,
        ..Default::default()
    };

    vimcats.for_help(CODE, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&vimcats, &s).to_string(),
        "\
ID                                                                        *U.ID*

    Type: ~
        string


User                                                                    *U.User*

    Fields: ~
        {name}   (string)
        {email}  (string)
        {id}     (ID)


U.Pi                                                                      *U.Pi*
    A Pi

    Type: ~
        (number)


U:create()                                                            *U:create*
    Creates a PI

    Returns: ~
        (number)

    Usage: ~
>lua
        require('Pi'):create()
<


"
    );
}

#[test]
fn rename_with_mod() {
    let src = format!("---@mod awesome This is working {CODE}");

    let mut vimcats = VimCats::new();
    let s = Settings {
        prefix_func: true,
        prefix_alias: true,
        prefix_class: true,
        prefix_type: true,
        ..Default::default()
    };

    vimcats.for_help(&src, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&vimcats, &s).to_string(),
        "\
==============================================================================
This is working                                                        *awesome*

ID                                                                  *awesome.ID*

    Type: ~
        string


User                                                              *awesome.User*

    Fields: ~
        {name}   (string)
        {email}  (string)
        {id}     (ID)


U.Pi                                                                *awesome.Pi*
    A Pi

    Type: ~
        (number)


U:create()                                                      *awesome:create*
    Creates a PI

    Returns: ~
        (number)

    Usage: ~
>lua
        require('Pi'):create()
<


"
    );
}

#[test]
fn expand_opt() {
    let src = "
local M = {}

---@class HelloWorld
---@field message? string First message to the world
---@field private opts? table
---@field secret table Sauce

---Prints given value
---@param message string
---@param opts? table
function M.echo(message, opts)
    return print(message)
end

return M
";

    let mut vimcats = VimCats::new();
    let s = Settings {
        expand_opt: true,
        ..Default::default()
    };

    vimcats.for_help(src, &s).unwrap();

    assert_eq!(
        VimDoc::from_emmy(&vimcats, &s).to_string(),
        "\
HelloWorld                                                          *HelloWorld*

    Fields: ~
        {message}  (nil|string)  First message to the world
        {secret}   (table)       Sauce


M.echo({message}, {opts?})                                              *M.echo*
    Prints given value

    Parameters: ~
        {message}  (string)
        {opts}     (nil|table)


"
    );
}
