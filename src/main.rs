use mlua::{ExternalResult, Lua, Result};

fn main() -> Result<()> {
    let lua = Lua::new();

    setup(&lua)?;

    print_version(&lua)?;

    repl(&lua)?;

    Ok(())
}

fn print_version(lua: &Lua) -> Result<()> {
    let version: String = lua.load("return _VERSION").eval()?;
    println!("Lua version: {}", version);
    Ok(())
}

fn repl(lua: &Lua) -> Result<()> {
    let mut rl = rustyline::DefaultEditor::new().to_lua_err()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match line {
                    line if line.starts_with(":") => {
                        match line.as_str() {
                            ":q" => {
                                break;
                            }
                            _ => {}
                        }
                        continue;
                    }
                    _ => {}
                }

                rl.add_history_entry(line.as_str()).to_lua_err()?;
                let result: mlua::Result<mlua::Value> = lua.load(&line).eval();

                match result {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                continue;
            }
        }
    }
    Ok(())
}

fn setup(lua: &Lua) -> Result<()> {
    // Synchronous HTTP
    let http_get = lua.create_function(|_, url: String| {
        let resp = reqwest::blocking::get(&url).to_lua_err()?;
        let body = resp.text().to_lua_err()?;
        Ok(body)
    })?;

    let lua_http_table = lua.create_table()?;
    lua_http_table.set("get", http_get)?;

    let globals = lua.globals();
    globals.set("http", lua_http_table)?;

    Ok(())
}
