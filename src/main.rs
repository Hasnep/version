use std::collections::HashMap;
use std::process::Command;

// fn list_tools(tools: &HashMap<&str, &str>) {
//     for (tool_name, _) in tools {
//         println!("{}", tool_name)
//     }
// }

fn get_version_of_tool(tool_name: &str, argument: &str) -> String {
    println!(
        "Getting the version of {} by running `{} {}`",
        tool_name, tool_name, argument
    );
    return match Command::new(tool_name).arg(argument).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(s) => s,
            Err(_) => return "Oh no something went wrong with decoding output.".to_owned(),
        },
        Err(_) => return "Oh no something went wrong with running the command.".to_owned(),
    };
}

fn check_does_tool_exist(tool_name: &str) -> bool {
    println!(
        "Checking if {} exists by running `sh -c command -v {}`",
        tool_name, tool_name
    );
    match Command::new("sh").arg("-c").arg("command -v snap").output() {
        Ok(output) => {
            if output.status.success() {
                println!("{} does exist.", tool_name);
                return true;
            } else {
                println!("{} does not exist.", tool_name);
                return false;
            }
        }
        Err(_) => {
            println!(
                "Got an error when trying to find if {} exists, assuming it doesn't.",
                tool_name
            );
            return false;
        }
    };
}

fn check_does_snap_exist() -> bool {
    return check_does_tool_exist("snap");
}

// fn extract_version_from_snap_output(stdout: &str) -> Option<String> {
//     regex::Regex.new("")
// }

fn check_snap(tool_name: &str) -> Option<String> {
    println!("Running `snap list {}`", tool_name);
    let output = Command::new("snap")
        .arg("list")
        .arg(tool_name)
        .output()
        .ok()?;
    if output.status.success() {
        return String::from_utf8(output.stdout).ok();
    } else {
        return None;
    }
}

fn main() {
    let tools: HashMap<&str, &str> = [
        ("alacritty", "-V"),
        ("apt", "-v"),
        ("arduino", "--version"),
        ("audacious", "-v"),
        ("autoconf", "--version"),
        ("automake", "--version"),
        ("awk", "--version"),
        ("baobab", "--version"),
        ("bash", "--version"),
        ("bibtex", "-version"),
        ("borg", "-V"),
        ("brew", "--version"),
        ("bundle", "-v"),
        ("bundler", "-v"),
        ("bzip2", "--version"),
        ("calibre", "--version"),
        ("cargo", "--version"),
        ("carthage", "version"),
        ("catfish", "--version"),
        ("ccmake", "--version"),
        ("cheese", "--version"),
        ("chromium", "--version"),
        ("clang", "--version"),
        ("clang_format", "--version"),
        ("clementine", "--version"),
        ("clisp", "--version"),
        ("cmake", "--version"),
        ("code", "--version"),
        ("coffee", "-v"),
        ("conky", "-v"),
        ("convert", "--version"),
        ("crystal", "-v"),
        ("ctags", "--version"),
        ("curl", "--version"),
        ("darcs", "-v"),
        ("deluge", "-V"),
        ("direnv", "version"),
        ("docker", "-v"),
        ("dosbox", "--version"),
        ("dotnet", "--version"),
        ("ed", "-V"),
        ("egrep", "--version"),
        ("electron", "-v"),
        ("elixir", "-v"),
        ("elm", "--version"),
        ("engrampa", "--version"),
        ("eog", "--version"),
        ("exa", "-v"),
        ("expac", "--version"),
        ("eyeD3", "--version"),
        ("ffmpeg", "-version"),
        ("flac", "-v"),
        ("flameshot", "-v"),
        ("flatpak", "--version"),
        ("firefox", "-v"),
        ("fish", "-v"),
        ("fossil", "version"),
        ("fzf", "--version"),
        ("gcc", "--version"),
        ("gdb", "--version"),
        ("gedit", "--version"),
        ("gem", "-v"),
        ("ghc", "--version"),
        ("gimp", "-version"),
        ("git", "--version"),
        ("gitkraken", "--version"),
        ("glade", "--version"),
        ("gnome_boxes", "--version"),
        ("go", "version"),
        ("google_chrome_stable", "--version"),
        ("gpodder", "--version"),
        ("gradle", "--version"),
        ("grep", "-V"),
        ("guake", "--version"),
        ("gzip", "-V"),
        ("HandBrakeCLI", "--version"),
        ("haxe", "-version"),
        ("haxelib", "version"),
        ("heroku", "-v"),
        ("htop", "-v"),
        ("inkscape", "--version"),
        ("java", "-version"),
        ("javac", "-version"),
        ("julia", "-v"),
        ("kodi", "-v"),
        ("kotlin", "-version"),
        ("ksh", "--version"),
        ("lame", "--version"),
        ("latex", "-version"),
        ("latexmk", "-version"),
        ("libreoffice", "--version"),
        ("love", "--version"),
        ("lua", "-v"),
        ("magick", "--version"),
        ("make", "-v"),
        ("man", "--version"),
        ("meld", "--version"),
        ("mousepad", "-v"),
        ("multipass", "version"),
        ("mpv", "--version"),
        ("nano", "--version"),
        ("nautilus", "--version"),
        ("ncdu", "-v"),
        ("neofetch", "--version"),
        ("ninja", "--version"),
        ("node", "-v"),
        ("nokogiri", "-v"),
        ("nordvpn", "-v"),
        ("npm", "-v"),
        ("npx", "-v"),
        ("nvim", "-version"),
        ("pacman", "--version"),
        ("pamac", "--version"),
        ("pdflatex", "-version"),
        ("perl", "-v"),
        ("php", "-v"),
        ("pip3", "-V"),
        ("pip", "-V"),
        ("pkg_config", "--version"),
        ("pyenv", "-v"),
        ("python2", "-V"),
        ("python3", "-V"),
        ("python", "-V"),
        ("rake", "--version"),
        ("ranger", "--version"),
        ("rbenv", "-v"),
        ("restic", "version"),
        ("rg", "-V"),
        ("rofi", "-v"),
        ("rome", "--version"),
        ("rsync", "--version"),
        ("rubocop", "-v"),
        ("ruby", "-v"),
        ("rustc", "--version"),
        ("samba", "--version"),
        ("sbatch", "-V"),
        ("scalac", "-version"),
        ("sccs", "-V"),
        ("screen", "-v"),
        ("sed", "--version"),
        ("sidekiq", "--version"),
        ("sinfo", "-V"),
        ("slack", "-v"),
        ("snap", "--version"),
        ("snapcraft", "version"),
        ("solaar", "--version"),
        ("spotify", "--version"),
        ("squeue", "-V"),
        ("ssh", "-V"),
        ("swift", "-version"),
        ("syncthing", "-version"),
        ("tcsh", "--version"),
        ("tex", "-version"),
        ("thefuck", "-v"),
        ("thunar", "--version"),
        ("thunderbird", "--version"),
        ("tldr", "-v"),
        ("tlmgr", "version"),
        ("tmux", "-V"),
        ("tsc", "-v"),
        ("ufw", "version"),
        ("valgrind", "--version"),
        ("version", "-v"),
        ("vi", "-version"),
        ("viewnior", "--version"),
        ("vim", "-version"),
        ("virt_manager", "--version"),
        ("vlc", "--version"),
        ("w3m", "-version"),
        ("wine", "--version"),
        ("wget", "--version"),
        ("wmctrl", "--version"),
        ("xdotool", "--version"),
        ("xelatex", "-version"),
        ("xetex", "-version"),
        ("xfce4_terminal", "--version"),
        ("xrandr", "-v"),
        ("xsel", "--version"),
        ("yarn", "--version"),
        ("yay", "--version"),
        ("youtube_dl", "--version"),
        ("zathura", "--version"),
        ("zenity", "--version"),
        ("zsh", "--version"),
    ]
    .iter()
    .cloned()
    .collect();

    let cli_args = std::env::args();

    println!("{:#?}", cli_args);

    if cli_args.len() <= 1 {
        // TODO: Add a help page here
        println!("No tool specified.");
    } else {
        let does_snap_exist = check_does_snap_exist();
        for tool_name in cli_args.skip(1) {
            println!("Checking tool `{}`", &tool_name);
            let tool_exists = check_does_tool_exist(&tool_name);
            if tool_exists {
                let version_argument = tools.get(&tool_name as &str);
                match version_argument {
                    Some(version_argument) => {
                        let tool_version = get_version_of_tool(&tool_name, &version_argument);
                        println!("{}", tool_version)
                    }
                    None => {
                        println!(
                            "I don't know how to get the version of the tool `{}`",
                            tool_name
                        )
                        // TODO: Continue
                    }
                }
            } else {
                println!("Tool doesn't exist, checking snaps.");
                if does_snap_exist {
                    let tool_version = check_snap(&tool_name);
                    match tool_version {
                        Some(v) => println!("{}", v),
                        None => println!("Tool {} not found as a snap", tool_name),
                    }
                }
            }
        }
    }
}
