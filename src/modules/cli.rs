/*
Jirai by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "App"
/// structure to create
/// a CLI app.
use yue::App;

/// Importing the data
/// structure to catch
/// and handle errors.
use super::err::JiraiErr;

/// Importing the function to
/// lint a string obtained from
/// a file containing Jirai source.
use super::compiler::lint_file;

/// Importing the function
/// to compile a Jirai string
/// into a HTML string.
use super::compiler::compile_file;

/// The function containing the Jirai compiler's
/// very light CLI. If the CLI operations are
/// successful, a string is returned. If any
/// operations fail, an error is returned.
pub fn cli() -> Result<String, JiraiErr>{
    let mut app: App = App::new(
        "Jirai Compiler",
        "0.3.0"
    );
    app.add_arg("lint", &true, "lints a Jirai file")?;
    app.add_arg("build", &true, "compiles a Jirai file")?;
    app.add_arg("out", &true, "the name of the output file")?;
    app.add_arg("help", &false, "displays usage information")?;
    app.add_arg("version", &false, "displays version information")?;
    let mut arguments: Vec<String> = std::env::args()
        .collect::<Vec<String>>();
    if arguments.len() > 1{
        arguments.remove(0);
        let _p: () = app.parse_args(&arguments)?;
        if app.arg_used("build"){
            let jirai_file: String = app.get_arg_data("build")?;
            if app.arg_used("out"){
                let out_file: String = app.get_arg_data("out")?;
                let _c: () = compile_file(&jirai_file, &Some(&out_file))?;
                let msg: String = format!(
                    "Compiled file \"{}\" into file \"{}\"!", 
                    jirai_file,
                    out_file
                );
                Ok(msg)
            }
            else {
                let _c: () = compile_file(&jirai_file, &None)?;
                let msg: String = format!(
                    "Compiled file \"{}\"!", 
                    jirai_file
                );
                Ok(msg)
            }
        }
        else if app.arg_used("lint"){
            let jirai_file: String = app.get_arg_data("lint")?;
            let is_ok: bool = lint_file(&jirai_file)?;
            if is_ok{
                Ok(format!("The file \"{}\" passed all checks.", jirai_file))
            }
            else {
                Ok(format!("The file \"{}\" did not pass all checks.", jirai_file))
            }
        }
        else if app.arg_used("version"){
            Ok(app.version_info())
        }
        else if app.arg_used("help"){
            Ok(app.help_info()?)
        }
        else {
            Err::<String, JiraiErr>(
                JiraiErr::new("Unrecognized arguments.")
            )       
        }
    }
    else {
        Err::<String, JiraiErr>(
            JiraiErr::new("Unrecognized arguments.")
        )
    }
}
