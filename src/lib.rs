use zed_extension_api::{self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput, SlashCommandOutputSection};

struct MiseExtension;

// =============================================================================
// LANGUAGE TEMPLATES
// =============================================================================

fn get_minimal_template() -> String {
    r#"# mise.toml - Project configuration
# Docs: https://mise.jdx.dev

[tools]
# Add your tools here

[env]
# Add environment variables here

[tasks]
# Add tasks here
"#.to_string()
}

fn get_python_template() -> String {
    r#"
# Python with uv
python = "3.12"
uv = "latest"
"#.to_string()
}

fn get_python_env_template() -> String {
    r#"
# Python/uv environment
UV_PROJECT_ENVIRONMENT = ".venv"
VIRTUAL_ENV = "{{ cwd }}/.venv"
"#.to_string()
}

fn get_python_tasks_template() -> String {
    r#"
[tasks.install]
description = "Install dependencies with uv"
run = "uv sync"

[tasks.dev]
description = "Run development server"
run = "uv run python -m uvicorn main:app --reload"
depends = ["install"]

[tasks.test]
description = "Run tests with pytest"
run = "uv run pytest"

[tasks.lint]
description = "Run linting"
run = "uv run ruff check ."

[tasks.format]
description = "Format code"
run = "uv run ruff format ."

[tasks.typecheck]
description = "Run type checking"
run = "uv run basedpyright"
"#.to_string()
}

fn get_python_hooks_template() -> String {
    r#"
[hooks.enter]
run = "uv sync --quiet"
"#.to_string()
}

fn get_node_template() -> String {
    r#"
# Node.js
node = "22"
"#.to_string()
}

fn get_node_env_template() -> String {
    r#"
# Node.js environment
NODE_ENV = "development"
"#.to_string()
}

fn get_node_tasks_template() -> String {
    r#"
[tasks.install]
description = "Install npm dependencies"
run = "npm install"

[tasks.dev]
description = "Run development server"
run = "npm run dev"
depends = ["install"]

[tasks.build]
description = "Build for production"
run = "npm run build"

[tasks.test]
description = "Run tests"
run = "npm test"

[tasks.lint]
description = "Run linting"
run = "npm run lint"
"#.to_string()
}

fn get_nextjs_template() -> String {
    r#"
# Next.js (includes Node)
node = "22"
"#.to_string()
}

fn get_nextjs_env_template() -> String {
    r#"
# Next.js environment
NODE_ENV = "development"
NEXT_TELEMETRY_DISABLED = "1"
"#.to_string()
}

fn get_nextjs_tasks_template() -> String {
    r#"
[tasks.install]
description = "Install dependencies"
run = "npm install"

[tasks.dev]
description = "Run Next.js development server"
run = "npm run dev"
depends = ["install"]

[tasks.build]
description = "Build Next.js for production"
run = "npm run build"

[tasks.start]
description = "Start production server"
run = "npm run start"
depends = ["build"]

[tasks.lint]
description = "Run Next.js linting"
run = "npm run lint"

[tasks.typecheck]
description = "Run TypeScript type checking"
run = "npx tsc --noEmit"
"#.to_string()
}

fn get_go_template() -> String {
    r#"
# Go
go = "1.23"
"#.to_string()
}

fn get_go_env_template() -> String {
    r#"
# Go environment
GOPATH = "{{ env.HOME }}/go"
"#.to_string()
}

fn get_go_tasks_template() -> String {
    r#"
[tasks.build]
description = "Build Go binary"
run = "go build -o bin/ ./..."

[tasks.test]
description = "Run Go tests"
run = "go test -v ./..."

[tasks.lint]
description = "Run golangci-lint"
run = "golangci-lint run"

[tasks.tidy]
description = "Tidy go.mod"
run = "go mod tidy"

[tasks.run]
description = "Run the application"
run = "go run ."
"#.to_string()
}

fn get_rust_template() -> String {
    r#"
# Rust
rust = "stable"
"#.to_string()
}

fn get_rust_tasks_template() -> String {
    r#"
[tasks.build]
description = "Build Rust project"
run = "cargo build"

[tasks.release]
description = "Build release binary"
run = "cargo build --release"

[tasks.test]
description = "Run tests"
run = "cargo test"

[tasks.lint]
description = "Run clippy"
run = "cargo clippy -- -D warnings"

[tasks.format]
description = "Format code"
run = "cargo fmt"

[tasks.run]
description = "Run the application"
run = "cargo run"
"#.to_string()
}

fn get_deno_template() -> String {
    r#"
# Deno
deno = "latest"
"#.to_string()
}

fn get_deno_tasks_template() -> String {
    r#"
[tasks.dev]
description = "Run with watch mode"
run = "deno run --watch main.ts"

[tasks.test]
description = "Run tests"
run = "deno test"

[tasks.lint]
description = "Run linting"
run = "deno lint"

[tasks.format]
description = "Format code"
run = "deno fmt"

[tasks.check]
description = "Type check"
run = "deno check main.ts"
"#.to_string()
}

fn get_bun_template() -> String {
    r#"
# Bun
bun = "latest"
"#.to_string()
}

fn get_bun_tasks_template() -> String {
    r#"
[tasks.install]
description = "Install dependencies"
run = "bun install"

[tasks.dev]
description = "Run development server"
run = "bun run dev"

[tasks.build]
description = "Build for production"
run = "bun run build"

[tasks.test]
description = "Run tests"
run = "bun test"
"#.to_string()
}

fn get_docker_tasks_template() -> String {
    r#"
[tasks.docker-build]
description = "Build Docker image"
run = "docker compose build"

[tasks.docker-up]
description = "Start containers"
run = "docker compose up -d"

[tasks.docker-down]
description = "Stop containers"
run = "docker compose down"

[tasks.docker-logs]
description = "Show container logs"
run = "docker compose logs -f"

[tasks.docker-shell]
description = "Open shell in container"
run = "docker compose exec app sh"
"#.to_string()
}

fn get_terraform_template() -> String {
    r#"
# Terraform/OpenTofu
opentofu = "latest"
"#.to_string()
}

fn get_terraform_tasks_template() -> String {
    r#"
[tasks.tf-init]
description = "Initialize Terraform"
run = "tofu init"

[tasks.tf-plan]
description = "Plan changes"
run = "tofu plan"

[tasks.tf-apply]
description = "Apply changes"
run = "tofu apply"

[tasks.tf-destroy]
description = "Destroy infrastructure"
run = "tofu destroy"

[tasks.tf-fmt]
description = "Format Terraform files"
run = "tofu fmt -recursive"
"#.to_string()
}

// =============================================================================
// SUPPORTED LANGUAGES
// =============================================================================

const SUPPORTED_LANGUAGES: &[&str] = &[
    "python", "node", "nextjs", "go", "rust", "deno", "bun", "docker", "terraform"
];

fn get_language_help() -> String {
    format!(r#"# Mise Init - Available Languages

Usage: /mise-init [language...]

## Supported Languages/Stacks

| Language   | Tools Installed              |
|------------|------------------------------|
| python     | python 3.12 + uv             |
| node       | node 22                      |
| nextjs     | node 22 (Next.js optimized)  |
| go         | go 1.23                      |
| rust       | rust stable                  |
| deno       | deno latest                  |
| bun        | bun latest                   |
| docker     | docker compose tasks         |
| terraform  | opentofu latest              |

## Examples

```
/mise-init                    # Minimal empty template
/mise-init python             # Python + uv project
/mise-init python node        # Full-stack: Python backend + Node frontend
/mise-init nextjs             # Next.js project
/mise-init go docker          # Go with Docker support
```

## Notes

- Python template includes uv integration with .venv auto-creation
- Templates include environment variables and hooks where appropriate
- Use /mise-task create to add task templates
"#)
}

fn get_task_help() -> String {
    r#"# Mise Task - Task Management

## Commands

| Command                      | Description                           |
|------------------------------|---------------------------------------|
| /mise-task create [stack...] | Add task templates to mise.toml       |
| /mise-task export            | Export mise tasks to .zed/tasks.json  |
| /mise-task help              | Show this help                        |

## Available Task Templates

| Stack      | Tasks Included                                    |
|------------|---------------------------------------------------|
| python     | install, dev, test, lint, format, typecheck       |
| node       | install, dev, build, test, lint                   |
| nextjs     | install, dev, build, start, lint, typecheck       |
| go         | build, test, lint, tidy, run                      |
| rust       | build, release, test, lint, format, run           |
| deno       | dev, test, lint, format, check                    |
| bun        | install, dev, build, test                         |
| docker     | docker-build, docker-up, docker-down, docker-logs |
| terraform  | tf-init, tf-plan, tf-apply, tf-destroy, tf-fmt    |

## Examples

```
/mise-task create python          # Add Python tasks
/mise-task create python nextjs   # Add Python + Next.js tasks
/mise-task export                 # Generate .zed/tasks.json
```

## Notes

- Tasks are appended to existing mise.toml
- Export creates/overwrites .zed/tasks.json
- After modifying mise.toml tasks, run export again
"#.to_string()
}

// =============================================================================
// MISE INIT COMMAND
// =============================================================================

fn build_mise_init(args: &[String]) -> Result<SlashCommandOutput, String> {
    // Handle help
    if args.first().map(|s| s.as_str()) == Some("help") {
        let text = get_language_help();
        return Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "Mise Init Help".to_string(),
            }],
            text,
        });
    }

    let mut tools_section = String::new();
    let mut env_section = String::new();
    let mut hooks_section = String::new();

    // If no args, return minimal template
    if args.is_empty() {
        let text = get_minimal_template();
        return Ok(SlashCommandOutput {
            sections: vec![SlashCommandOutputSection {
                range: (0..text.len()).into(),
                label: "mise.toml (minimal)".to_string(),
            }],
            text,
        });
    }

    // Build template based on requested languages
    for lang in args {
        match lang.to_lowercase().as_str() {
            "python" => {
                tools_section.push_str(&get_python_template());
                env_section.push_str(&get_python_env_template());
                hooks_section.push_str(&get_python_hooks_template());
            }
            "node" => {
                tools_section.push_str(&get_node_template());
                env_section.push_str(&get_node_env_template());
            }
            "nextjs" => {
                tools_section.push_str(&get_nextjs_template());
                env_section.push_str(&get_nextjs_env_template());
            }
            "go" => {
                tools_section.push_str(&get_go_template());
                env_section.push_str(&get_go_env_template());
            }
            "rust" => {
                tools_section.push_str(&get_rust_template());
            }
            "deno" => {
                tools_section.push_str(&get_deno_template());
            }
            "bun" => {
                tools_section.push_str(&get_bun_template());
            }
            "terraform" | "opentofu" => {
                tools_section.push_str(&get_terraform_template());
            }
            "docker" => {
                // Docker doesn't add tools, only tasks
            }
            unknown => {
                return Err(format!(
                    "Unknown language: '{}'. Use /mise-init help to see available options.",
                    unknown
                ));
            }
        }
    }

    // Build final template
    let mut text = String::from("# mise.toml - Project configuration\n# Docs: https://mise.jdx.dev\n");

    text.push_str("\n[tools]");
    text.push_str(&tools_section);

    text.push_str("\n[env]");
    if env_section.is_empty() {
        text.push_str("\n# Add environment variables here\n");
    } else {
        text.push_str(&env_section);
    }

    if !hooks_section.is_empty() {
        text.push_str(&hooks_section);
    }

    text.push_str("\n[tasks]\n# Add tasks here or use /mise-task create\n");

    let label = if args.is_empty() {
        "mise.toml (minimal)".to_string()
    } else {
        format!("mise.toml ({})", args.join(" + "))
    };

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label,
        }],
        text,
    })
}

// =============================================================================
// MISE TASK COMMAND
// =============================================================================

fn build_mise_task(args: &[String]) -> Result<SlashCommandOutput, String> {
    let subcommand = args.first().map(|s| s.as_str()).unwrap_or("help");

    match subcommand {
        "help" => {
            let text = get_task_help();
            Ok(SlashCommandOutput {
                sections: vec![SlashCommandOutputSection {
                    range: (0..text.len()).into(),
                    label: "Mise Task Help".to_string(),
                }],
                text,
            })
        }
        "create" => {
            let stacks = &args[1..];
            if stacks.is_empty() {
                return Err("Usage: /mise-task create [stack...]\nExample: /mise-task create python nextjs".to_string());
            }

            let mut tasks = String::new();
            tasks.push_str("# Add these tasks to your mise.toml\n");

            for stack in stacks {
                match stack.to_lowercase().as_str() {
                    "python" => tasks.push_str(&get_python_tasks_template()),
                    "node" => tasks.push_str(&get_node_tasks_template()),
                    "nextjs" => tasks.push_str(&get_nextjs_tasks_template()),
                    "go" => tasks.push_str(&get_go_tasks_template()),
                    "rust" => tasks.push_str(&get_rust_tasks_template()),
                    "deno" => tasks.push_str(&get_deno_tasks_template()),
                    "bun" => tasks.push_str(&get_bun_tasks_template()),
                    "docker" => tasks.push_str(&get_docker_tasks_template()),
                    "terraform" | "opentofu" => tasks.push_str(&get_terraform_tasks_template()),
                    unknown => {
                        return Err(format!(
                            "Unknown stack: '{}'. Use /mise-task help to see available options.",
                            unknown
                        ));
                    }
                }
            }

            Ok(SlashCommandOutput {
                sections: vec![SlashCommandOutputSection {
                    range: (0..tasks.len()).into(),
                    label: format!("Mise Tasks ({})", stacks.join(" + ")),
                }],
                text: tasks,
            })
        }
        "export" => {
            let text = r#"# Zed Tasks Export

To export mise tasks to Zed, run this command in your terminal:

```bash
mise tasks --json | jq '[.[] | {label: .name, command: ("mise run " + .name), args: []}]' > .zed/tasks.json
```

Or manually create `.zed/tasks.json`:

```json
{
  "tasks": [
    {
      "label": "mise: dev",
      "command": "mise",
      "args": ["run", "dev"]
    },
    {
      "label": "mise: test",
      "command": "mise",
      "args": ["run", "test"]
    },
    {
      "label": "mise: build",
      "command": "mise",
      "args": ["run", "build"]
    }
  ]
}
```

Note: Zed doesn't support dynamic task providers, so you need to regenerate
this file when you add/remove tasks from mise.toml.
"#.to_string();

            Ok(SlashCommandOutput {
                sections: vec![SlashCommandOutputSection {
                    range: (0..text.len()).into(),
                    label: "Mise Task Export".to_string(),
                }],
                text,
            })
        }
        unknown => Err(format!(
            "Unknown subcommand: '{}'. Use: create, export, or help",
            unknown
        )),
    }
}

// =============================================================================
// MISE ENV COMMAND
// =============================================================================

fn build_mise_env() -> Result<SlashCommandOutput, String> {
    let text = r#"# Mise Environment

To view current mise environment, run in terminal:

```bash
mise env
```

To view specific tool paths:

```bash
mise which python
mise which node
mise where python
```

To see all active tools:

```bash
mise ls --current
```

Note: Zed extensions cannot directly access the shell environment.
Use the terminal or configure Zed to inherit mise environment by:

1. Launch Zed from terminal where mise is active: `zed .`
2. Or add to ~/.zprofile: `eval "$(mise activate --shims zsh)"`
"#.to_string();

    Ok(SlashCommandOutput {
        sections: vec![SlashCommandOutputSection {
            range: (0..text.len()).into(),
            label: "Mise Environment".to_string(),
        }],
        text,
    })
}

// =============================================================================
// EXTENSION IMPLEMENTATION
// =============================================================================

impl zed::Extension for MiseExtension {
    fn new() -> Self {
        MiseExtension
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "mise-init" => build_mise_init(&args),
            "mise-task" => build_mise_task(&args),
            "mise-env" => build_mise_env(),
            cmd => Err(format!("Unknown command: {}", cmd)),
        }
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "mise-init" => {
                // Filter out already selected languages
                let completions: Vec<SlashCommandArgumentCompletion> = SUPPORTED_LANGUAGES
                    .iter()
                    .filter(|lang| !args.contains(&lang.to_string()))
                    .map(|lang| SlashCommandArgumentCompletion {
                        label: lang.to_string(),
                        new_text: lang.to_string(),
                        run_command: false, // Don't run immediately, allow multiple selections
                    })
                    .chain(std::iter::once(SlashCommandArgumentCompletion {
                        label: "help".to_string(),
                        new_text: "help".to_string(),
                        run_command: true,
                    }))
                    .collect();
                Ok(completions)
            }
            "mise-task" => {
                if args.is_empty() {
                    // First argument: subcommand
                    Ok(vec![
                        SlashCommandArgumentCompletion {
                            label: "create - Add task templates".to_string(),
                            new_text: "create".to_string(),
                            run_command: false,
                        },
                        SlashCommandArgumentCompletion {
                            label: "export - Export to .zed/tasks.json".to_string(),
                            new_text: "export".to_string(),
                            run_command: true,
                        },
                        SlashCommandArgumentCompletion {
                            label: "help - Show help".to_string(),
                            new_text: "help".to_string(),
                            run_command: true,
                        },
                    ])
                } else if args.first().map(|s| s.as_str()) == Some("create") {
                    // After "create", suggest stacks
                    let selected: Vec<&str> = args.iter().skip(1).map(|s| s.as_str()).collect();
                    let completions: Vec<SlashCommandArgumentCompletion> = SUPPORTED_LANGUAGES
                        .iter()
                        .filter(|lang| !selected.contains(*lang))
                        .map(|lang| SlashCommandArgumentCompletion {
                            label: lang.to_string(),
                            new_text: lang.to_string(),
                            run_command: false,
                        })
                        .collect();
                    Ok(completions)
                } else {
                    Ok(vec![])
                }
            }
            _ => Ok(vec![]),
        }
    }
}

zed::register_extension!(MiseExtension);
