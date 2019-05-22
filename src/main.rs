use clipboard::{ClipboardContext, ClipboardProvider};
use failure::{format_err, Error};
use rlua::Lua;
use std::{fs, time::Duration};

fn execute(lua: &Lua, clipboard_text: &str) -> Result<String, Error> {
    let lua_script = fs::read_to_string("main.lua")?;
    lua.context::<_, rlua::Result<_>>(|lua_ctx| {
        let globals = lua_ctx.globals();
        globals.set("clipboard", clipboard_text.to_owned())?;
        lua_ctx.load(&lua_script).eval::<String>()
    })
    .map_err(|e| format_err!("{:?}", e))
}

fn main() -> Result<(), Error> {
    let lua = Lua::new();
    let mut clipboard_ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to create clipboard provider");
    let mut prev_clipboard = String::new();

    loop {
        match clipboard_ctx.get_contents() {
            Ok(ref clipboard_text) if clipboard_text != &prev_clipboard => {
                prev_clipboard = clipboard_text.clone();
                match execute(&lua, clipboard_text) {
                    Ok(s) => {
                        println!("{} => {}", clipboard_text, s);
                        if clipboard_ctx.set_contents(s.clone()).is_ok() {
                            prev_clipboard = s;
                        }
                    }
                    Err(e) => println!("{}, {:?}", clipboard_text, e),
                }
            }
            _ => {}
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
