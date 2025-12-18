use clap::{Parser, Subcommand};
use colored::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use std::process::{Command, Output};
use std::path::PathBuf;
use chrono;
use indicatif::{ProgressBar, ProgressStyle};
use console::{style, Emoji};
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

/// TRAE-CLI: Ejecutor de comandos Rust que reporta a JARVIXSERVER
#[derive(Parser)]
#[command(name = "trae")]
#[command(about = "Ejecuta comandos Rust (cargo) y reporta resultados a JARVIXSERVER")]
#[command(version = "0.2.0")]
#[command(author = "TRAE Team")]
struct Args {
    #[command(subcommand)]
    command: Option<CargoCommand>,

    /// URL del servidor JARVIXSERVER
    #[arg(long, global = true, default_value = "http://localhost:8080", env = "JARVIX_URL")]
    jarvix: String,

    /// Ruta del proyecto Rust a ejecutar
    #[arg(long, global = true, default_value = ".", value_parser = validate_path)]
    project: PathBuf,

    /// No reportar resultado a JARVIXSERVER
    #[arg(long, global = true)]
    no_report: bool,

    /// Mostrar output detallado
    #[arg(short, long, global = true)]
    verbose: bool,
}

/// Información de código muerto detectado
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeadCodeItem {
    item_type: String, // function, struct, enum, const, static
    name: String,
    file: String,
    line: usize,
    is_pub: bool,
}

/// Información de módulo
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModuleInfo {
    name: String,
    path: String,
    used: bool,
    sub_modules: Vec<String>,
    file_count: usize,
}

/// Información de mock generado
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MockInfo {
    trait_name: String,
    methods: Vec<String>,
    generated_code: String,
}

/// Información extraída por el crawler
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CrawledInfo {
    dependencies: Vec<String>,
    functions: Vec<FunctionInfo>,
    structs: Vec<StructInfo>,
    traits: Vec<TraitInfo>,
    tests: Vec<TestInfo>,
    todos: Vec<TodoItem>,
    metrics: ProjectMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FunctionInfo {
    name: String,
    file: String,
    line: usize,
    is_pub: bool,
    params: Vec<String>,
    return_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StructInfo {
    name: String,
    file: String,
    fields: Vec<String>,
    is_pub: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TraitInfo {
    name: String,
    file: String,
    methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestInfo {
    name: String,
    file: String,
    line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoItem {
    text: String,
    file: String,
    line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectMetrics {
    total_lines: usize,
    total_functions: usize,
    total_structs: usize,
    total_traits: usize,
    total_tests: usize,
    code_files: usize,
    test_coverage_estimate: f64,
}

#[derive(Subcommand)]
enum CargoCommand {
    /// cargo check - Verificar código sin compilar
    #[command(visible_alias = "c")]
    Check {
        /// Verificar ejemplos
        #[arg(long)]
        examples: bool,

        /// Verificar tests
        #[arg(long)]
        tests: bool,

        /// Verificar todo el workspace
        #[arg(long)]
        workspace: bool,

        /// Activar todas las features
        #[arg(long)]
        all_features: bool,

        /// Número de threads paralelos
        #[arg(long, short = 'j')]
        jobs: Option<u32>,

        /// Target específico
        #[arg(long)]
        target: Option<String>,

        /// Mostrar warnings como errores
        #[arg(long)]
        deny_warnings: bool,
    },

    /// cargo build - Compilar proyecto
    #[command(visible_alias = "b")]
    Build {
        /// Compilar con optimizaciones para producción
        #[arg(long, short = 'r')]
        release: bool,

        /// Modo debug con información de depuración
        #[arg(long)]
        debug: bool,

        /// Compilar todo el workspace
        #[arg(long)]
        workspace: bool,

        /// Activar todas las features
        #[arg(long)]
        all_features: bool,

        /// Target específico
        #[arg(long)]
        target: Option<String>,

        /// Mostrar timings de compilación
        #[arg(long)]
        timings: bool,

        /// Continuar tras errores
        #[arg(long)]
        keep_going: bool,

        /// Número de threads paralelos
        #[arg(long, short = 'j')]
        jobs: Option<u32>,
    },

    /// cargo test - Ejecutar tests
    #[command(visible_alias = "t")]
    Test {
        /// Ejecutar tests específicos (nombre)
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Testear todo el workspace
        #[arg(long)]
        workspace: bool,

        /// Ejecutar en modo release
        #[arg(long)]
        release: bool,

        /// Ejecutar tests documentados
        #[arg(long)]
        doc: bool,

        /// Ejecutar tests sin capturar output
        #[arg(long)]
        nocapture: bool,

        /// Ejecutar tests en single-threaded
        #[arg(long)]
        single_threaded: bool,
    },

    /// cargo run - Ejecutar binario
    #[command(visible_alias = "r")]
    Run {
        /// Argumentos para pasar al binario
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Compilar en release antes de correr
        #[arg(long, short = 'r')]
        release: bool,

        /// Compilar ejemplos específicos
        #[arg(long)]
        example: Option<String>,

        /// Bin específico
        #[arg(long)]
        bin: Option<String>,

        /// Manifest
        #[arg(long)]
        manifest_path: Option<String>,
    },

    /// cargo new - Crear nuevo proyecto
    New {
        /// Ruta del proyecto
        path: String,
        /// Crear librería en lugar de binario
        #[arg(long)]
        lib: bool,
    },

    /// cargo init - Inicializar proyecto en directorio actual
    Init {
        /// Ruta (opcional)
        path: Option<String>,
        /// Crear librería en lugar de binario
        #[arg(long)]
        lib: bool,
    },

    /// cargo add - Agregar dependencias
    Add {
        /// Dependencias a agregar
        #[arg(required = true)]
        crates: Vec<String>,

        /// Agregar como dev-dependency
        #[arg(long, short = 'D')]
        dev: bool,

        /// Agregar como build-dependency
        #[arg(long, short = 'B')]
        build: bool,

        /// Features a activar
        #[arg(long, short = 'F')]
        features: Vec<String>,

        /// Versión específica
        #[arg(long)]
        version: Option<String>,

        /// Desde git
        #[arg(long)]
        git: Option<String>,

        /// Desde ruta local
        #[arg(long)]
        path: Option<String>,

        /// Branch específico (con git)
        #[arg(long)]
        branch: Option<String>,
    },

    /// cargo remove - Remover dependencias
    Remove {
        /// Dependencias a remover
        #[arg(required = true)]
        crates: Vec<String>,
    },

    /// cargo bench - Ejecutar benchmarks
    Bench {
        /// Argumentos extra
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Benchmark específico
        #[arg(long)]
        bench: Option<String>,

        /// Mostrar output
        #[arg(long)]
        verbose: bool,

        /// No ejecutar, solo compilar
        #[arg(long)]
        no_run: bool,
    },

    /// cargo search - Buscar crates en crates.io
    Search {
        /// Query de búsqueda
        query: String,

        /// Límite de resultados
        #[arg(long, default_value = "10")]
        limit: u32,

        /// Mostrar detalles completos
        #[arg(long)]
        verbose: bool,

        /// Formato: json, table
        #[arg(long)]
        format: Option<String>,
    },

    /// cargo install - Instalar binario
    Install {
        /// Crate a instalar
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// cargo uninstall - Desinstalar binario
    Uninstall {
        /// Crate a desinstalar
        package: String,
    },

    /// cargo fmt - Formatear código
    Fmt {
        /// Verificar si el código está formateado
        #[arg(long, short = 'c')]
        check: bool,
    },

    /// cargo clippy - Análisis estático de código
    #[command(visible_alias = "lint")]
    Clippy {
        /// Modo estricto
        #[arg(long)]
        strict: bool,

        /// Fix automático
        #[arg(long)]
        fix: bool,

        /// Workspace
        #[arg(long)]
        workspace: bool,

        /// Mostrar todos los lints
        #[arg(long)]
        all_targets: bool,

        /// Modo pedantic
        #[arg(long)]
        pedantic: bool,

        /// Desactivar lints específicos
        #[arg(long)]
        allow: Option<String>,

        /// Número de threads
        #[arg(long, short = 'j')]
        jobs: Option<u32>,
    },

    /// cargo clean - Limpiar artifacts compilados
    Clean,

    /// cargo doc - Generar documentación
    Doc {
        /// Abrir documentación en navegador
        #[arg(long)]
        open: bool,

        /// Documentar dependencias privadas
        #[arg(long)]
        document_private_items: bool,

        /// No documentar dependencias
        #[arg(long)]
        no_deps: bool,

        /// Workspace
        #[arg(long)]
        workspace: bool,

        /// Threads paralelos
        #[arg(long, short = 'j')]
        jobs: Option<u32>,
    },

    /// cargo tree - Visualizar árbol de dependencias
    Tree {
        /// Profundidad máxima
        #[arg(long, short = 'd')]
        depth: Option<usize>,
    },

    /// cargo update - Actualizar dependencias
    Update {
        /// Actualizar dependencia específica
        #[arg(short = 'p', long)]
        package: Option<String>,
    },

    /// Comando cargo personalizado
    Custom {
        /// Comando y argumentos personalizados
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// 🪦 Detectar dead code no utilizado
    Deadcode {
        /// Mostrar detalles completos
        #[arg(long)]
        verbose: bool,

        /// Workspace
        #[arg(long)]
        workspace: bool,

        /// Mostrar solo funciones
        #[arg(long)]
        functions: bool,

        /// Mostrar solo structs
        #[arg(long)]
        structs: bool,

        /// Mostrar solo enums
        #[arg(long)]
        enums: bool,
    },

    /// 🎭 Generar mocks y test doubles
    Mock {
        /// Trait a mockear
        #[arg(long)]
        trait_name: Option<String>,

        /// Archivo de salida
        #[arg(long, short = 'o')]
        output: Option<String>,

        /// Usar mockito
        #[arg(long)]
        mockito: bool,

        /// Usar mock_derive
        #[arg(long)]
        derive: bool,
    },

    /// 📦 Analizar módulos no utilizados
    Modules {
        /// Mostrar solo módulos sin usar
        #[arg(long)]
        unused_only: bool,

        /// Incluir dependencias
        #[arg(long)]
        with_deps: bool,

        /// Mostrar en formato árbol
        #[arg(long)]
        tree: bool,

        /// Profundidad máxima
        #[arg(long, short = 'd')]
        depth: Option<usize>,
    },

    /// 🚀 SUPER COMANDO: Verifica todo antes de subir cambios (fmt + clippy + test + build)
    Preflight,

    /// 🔧 SUPER COMANDO: Intenta arreglar todo automáticamente (fix + fmt + clippy fix)
    Repair,

    /// 🌐 Buscar información en internet usando JARVIXSERVER
    WebSearch {
        /// Consulta de búsqueda
        query: String,
        /// Número máximo de resultados
        #[arg(short = 'n', long, default_value = "5")]
        limit: usize,
        /// Incluir código fuente en resultados
        #[arg(long)]
        include_code: bool,
        /// Buscar específicamente en documentación de Rust
        #[arg(long)]
        rust_docs: bool,
        /// Buscar en crates.io
        #[arg(long)]
        crates: bool,
    },
}

/// Resultado de ejecutar un comando cargo
#[derive(Serialize, Deserialize, Clone, Debug)]
struct CommandResult {
    /// Comando ejecutado (ej: "cargo build")
    command: String,
    /// Ruta del proyecto
    project: String,
    /// Si fue exitoso
    success: bool,
    /// Output estándar
    stdout: String,
    /// Output de error
    stderr: String,
    /// Código de salida del proceso
    exit_code: i32,
    /// Timestamp en formato RFC3339
    timestamp: String,
    /// Tiempo de ejecución en ms
    duration_ms: u128,
}

/// Valida que la ruta del proyecto existe
fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if path.exists() {
        Ok(path)
    } else {
        Err(format!("La ruta '{}' no existe", s))
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    print_header(&args);

    // Validar que cargo existe
    if !check_cargo_installed() {
        eprintln!("{} Cargo no está instalado o no está en el PATH", "✗".red().bold());
        std::process::exit(1);
    }

    // Ejecutar comando
    let start = std::time::Instant::now();
    let (cmd_name, output) = execute_command(&args).await;
    let duration = start.elapsed().as_millis();

    // Procesar resultado
    let success = output.status.success();
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    // Mostrar resultados
    display_output(&stdout, &stderr);

    // Crear resultado
    let result = CommandResult {
        command: format!("cargo {}", cmd_name),
        project: args.project.display().to_string(),
        success,
        stdout,
        stderr,
        exit_code,
        timestamp: chrono::Local::now().to_rfc3339(),
        duration_ms: duration,
    };

    // Reportar a JARVIXSERVER
    if !args.no_report {
        report_to_jarvix(&args, &result).await;
    }

    // Mostrar resumen
    print_summary(&result);

    // Salir con código apropiado
    if !success {
        std::process::exit(exit_code);
    }
}

/// Imprime el encabezado de la aplicación
fn print_header(args: &Args) {
    println!("{}", "╔════════════════════════════════════════════════════════╗".cyan());
    println!("{}", "║        ▶ TRAE-CLI v0.2.0 - Ejecutor de Rust            ║".cyan().bold());
    println!("{}", "║     Compilación, Testing & Reporting Integrado         ║".bright_cyan());
    println!("{}", "╚════════════════════════════════════════════════════════╝".cyan());
    println!("  {} {}", style("JARVIXSERVER:").cyan().bold(), args.jarvix.green());
    println!("  {} {}", style("Proyecto:").cyan().bold(), args.project.display().to_string().green());
    if args.verbose {
        println!("  {} ACTIVADO", style("Verbose:").cyan().bold());
    }
    println!();
}

/// Verifica que cargo está instalado
fn check_cargo_installed() -> bool {
    match Command::new("cargo").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Ejecuta el comando cargo
async fn execute_command(args: &Args) -> (&'static str, Output) {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&args.project);

    // MEJORA: Cargar variables de entorno desde .env automáticamente para todos los comandos
    // Esto es especialmente útil para 'run' y 'test', pero no hace daño en otros.
    if let Ok(env_vars) = load_env_file(&args.project) {
        if !env_vars.is_empty() {
             println!("{} Cargadas {} variables desde .env", "ℹ".blue(), env_vars.len());
             cmd.envs(env_vars);
        }
    }

    let cmd_name = match &args.command {
        Some(CargoCommand::Check { examples, tests, workspace, all_features, jobs, target, deny_warnings }) => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner} {msg}").unwrap()
            );
            spinner.set_message("Verificando estilo de código...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            let fmt_status = Command::new("cargo")
                .arg("fmt")
                .arg("--check")
                .current_dir(&args.project)
                .output();

            if let Ok(output) = fmt_status {
                if !output.status.success() {
                    spinner.finish_with_message("❌ Formato incorrecto (ejecuta 'trae fmt')");
                } else {
                    spinner.finish_with_message("✓ Formato verificado");
                }
            } else {
                spinner.finish_with_message("⚠ No se pudo verificar formato");
            }

            cmd.arg("check");
            if *examples { cmd.arg("--examples"); }
            if *tests { cmd.arg("--tests"); }
            if *workspace { cmd.arg("--workspace"); }
            if *all_features { cmd.arg("--all-features"); }
            if let Some(j) = jobs { cmd.args(&["--jobs", &j.to_string()]); }
            if let Some(t) = target { cmd.args(&["--target", t]); }
            if *deny_warnings { cmd.args(&["--", "-D", "warnings"]); }
            "check"
        }
        Some(CargoCommand::Build { release, debug, workspace, all_features, target, timings, keep_going, jobs }) => {
            cmd.arg("build");
            if *release { cmd.arg("--release"); }
            if *debug { cmd.arg("--debug"); }
            if *workspace { cmd.arg("--workspace"); }
            if *all_features { cmd.arg("--all-features"); }
            if let Some(t) = target { cmd.args(&["--target", t]); }
            if *timings { cmd.arg("--timings"); }
            if *keep_going { cmd.arg("--keep-going"); }
            if let Some(j) = jobs { cmd.args(&["--jobs", &j.to_string()]); }
            "build"
        }
        Some(CargoCommand::Test { args: test_args, workspace, release, doc, nocapture, single_threaded }) => {
            cmd.arg("test");
            if *workspace { cmd.arg("--workspace"); }
            if *release { cmd.arg("--release"); }
            if *doc { cmd.arg("--doc"); }
            cmd.arg("--");
            if *nocapture { cmd.arg("--nocapture"); }
            if *single_threaded { cmd.arg("--test-threads=1"); }
            for arg in test_args {
                cmd.arg(arg);
            }
            "test"
        }
        Some(CargoCommand::Run { args: run_args, release, example, bin, manifest_path }) => {
            cmd.arg("run");
            if *release { cmd.arg("--release"); }
            if let Some(e) = example { cmd.args(&["--example", e]); }
            if let Some(b) = bin { cmd.args(&["--bin", b]); }
            if let Some(m) = manifest_path { cmd.args(&["--manifest-path", m]); }
            cmd.arg("--");
            for arg in run_args {
                cmd.arg(arg);
            }
            "run"
        }
        Some(CargoCommand::New { path, lib }) => {
            cmd.arg("new");
            cmd.arg(path);
            if *lib { cmd.arg("--lib"); }
            "new"
        }
        Some(CargoCommand::Init { path, lib }) => {
            cmd.arg("init");
            if let Some(p) = path { cmd.arg(p); }
            if *lib { cmd.arg("--lib"); }
            "init"
        }
        Some(CargoCommand::Add { crates, dev, build, features, version, git, path, branch }) => {
            cmd.arg("add");
            for krate in crates { cmd.arg(krate); }
            if *dev { cmd.arg("--dev"); }
            if *build { cmd.arg("--build"); }
            for feature in features {
                cmd.args(&["--features", feature]);
            }
            if let Some(v) = version { cmd.args(&["--version", v]); }
            if let Some(g) = git { cmd.args(&["--git", g]); }
            if let Some(p) = path { cmd.args(&["--path", p]); }
            if let Some(b) = branch { cmd.args(&["--branch", b]); }
            "add"
        }
        Some(CargoCommand::Remove { crates }) => {
            cmd.arg("remove");
            for krate in crates { cmd.arg(krate); }
            "remove"
        }
        Some(CargoCommand::Bench { args: bench_args, bench, verbose, no_run }) => {
            cmd.arg("bench");
            if let Some(b) = bench { cmd.arg(b); }
            if *verbose { cmd.arg("--verbose"); }
            if *no_run { cmd.arg("--no-run"); }
            cmd.arg("--");
            for arg in bench_args { cmd.arg(arg); }
            "bench"
        }
        Some(CargoCommand::Search { query, limit, verbose, format }) => {
            cmd.arg("search");
            cmd.arg(query);
            cmd.args(&["--limit", &limit.to_string()]);
            if *verbose { cmd.arg("--verbose"); }
            if let Some(f) = format { cmd.args(&["--format", f]); }
            "search"
        }
        Some(CargoCommand::Install { args: install_args }) => {
            cmd.arg("install");
            for arg in install_args { cmd.arg(arg); }
            "install"
        }
        Some(CargoCommand::Uninstall { package }) => {
            cmd.arg("uninstall");
            cmd.arg(package);
            "uninstall"
        }
        Some(CargoCommand::Fmt { check }) => {
            cmd.arg("fmt");
            if *check { cmd.arg("--check"); }
            "fmt"
        }
        Some(CargoCommand::Clippy { strict, fix, workspace, all_targets, pedantic, allow, jobs }) => {
            cmd.arg("clippy");
            if *fix { cmd.arg("--fix"); }
            if *workspace { cmd.arg("--workspace"); }
            if *all_targets { cmd.arg("--all-targets"); }
            if let Some(j) = jobs { cmd.args(&["--jobs", &j.to_string()]); }

            cmd.arg("--");
            if *strict { cmd.args(&["-D", "warnings"]); }
            if *pedantic { cmd.arg("-W"); cmd.arg("clippy::pedantic"); }
            if let Some(a) = allow { cmd.arg(format!("-A {}", a)); }
            "clippy"
        }
        Some(CargoCommand::Clean) => {
            cmd.arg("clean");
            "clean"
        }
        Some(CargoCommand::Doc { open, document_private_items, no_deps, workspace, jobs }) => {
            cmd.arg("doc");
            if *open { cmd.arg("--open"); }
            if *document_private_items { cmd.arg("--document-private-items"); }
            if *no_deps { cmd.arg("--no-deps"); }
            if *workspace { cmd.arg("--workspace"); }
            if let Some(j) = jobs { cmd.args(&["--jobs", &j.to_string()]); }
            "doc"
        }
        Some(CargoCommand::Tree { depth }) => {
            cmd.arg("tree");
            if let Some(d) = depth {
                cmd.args(&["--depth", &d.to_string()]);
            }
            "tree"
        }
        Some(CargoCommand::Update { package }) => {
            cmd.arg("update");
            if let Some(pkg) = package {
                cmd.arg("-p");
                cmd.arg(pkg);
            }
            "update"
        }
        Some(CargoCommand::Custom { args: custom_args }) => {
            for arg in custom_args {
                cmd.arg(arg);
            }
            "custom"
        }
        Some(CargoCommand::Deadcode { verbose, workspace: _workspace, functions, structs, enums }) => {
            println!("{} {} Analizando dead code y extrayendo información del proyecto...", "→".blue().bold(), Emoji("🪦", ""));
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner} {msg}").unwrap()
            );
            spinner.set_message("Ejecutando crawling avanzado del proyecto...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            // Análisis avanzado: Crawling semántico
            let crawled = advanced_project_crawler(&args.project);

            spinner.finish_with_message(format!(
                "✓ Crawling completado: {} funciones, {} structs, {} traits, {} tests"
            , crawled.functions.len(), crawled.structs.len(), crawled.traits.len(), crawled.tests.len()));
            println!();

            // Mostrar estadísticas del proyecto
            if *verbose {
                println!("{}", "┌─ MÉTRICAS DEL PROYECTO ─────────────────────┐".cyan().bold());
                println!("  {} líneas de código", crawled.metrics.total_lines);
                println!("  {} archivos Rust", crawled.metrics.code_files);
                println!("  {} funciones totales", crawled.metrics.total_functions);
                println!("  {} structs", crawled.metrics.total_structs);
                println!("  {} traits", crawled.metrics.total_traits);
                println!("  {} tests (cobertura estimada: {:.1}%)", crawled.metrics.total_tests, crawled.metrics.test_coverage_estimate);
                println!("  {} dependencias", crawled.dependencies.len());
                println!("{}", "└─────────────────────────────────────────────┘".cyan().bold());
                println!();

                // Mostrar dependencias
                if !crawled.dependencies.is_empty() {
                    println!("{}", "┌─ DEPENDENCIAS ──────────────────────────────┐".yellow().bold());
                    for (i, dep) in crawled.dependencies.iter().take(10).enumerate() {
                        println!("  {} {}", format!("{}.", i+1).bright_black(), dep);
                    }
                    if crawled.dependencies.len() > 10 {
                        println!("  ... y {} más", crawled.dependencies.len() - 10);
                    }
                    println!("{}", "└─────────────────────────────────────────────┘".yellow().bold());
                    println!();
                }
            }

            // Mostrar funciones encontradas
            if !crawled.functions.is_empty() {
                println!("{}", "┌─ FUNCIONES DETECTADAS ──────────────────────┐".green().bold());
                for func in crawled.functions.iter().take(20) {
                    let pub_marker = if func.is_pub { "pub " } else { "" };
                    println!("  {} {}{}({})",
                        "→".green(),
                        pub_marker,
                        func.name.cyan(),
                        func.params.join(", ").bright_black()
                    );
                }
                if crawled.functions.len() > 20 {
                    println!("  ... y {} más", crawled.functions.len() - 20);
                }
                println!("{}", "└─────────────────────────────────────────────┘".green().bold());
                println!();
            }

            // Mostrar structs
            if !crawled.structs.is_empty() {
                println!("{}", "┌─ STRUCTS DEFINIDAS ─────────────────────────┐".magenta().bold());
                for st in crawled.structs.iter().take(15) {
                    let pub_marker = if st.is_pub { "pub " } else { "" };
                    println!("  {} {}{} {{ {} }}",
                        "⚙".magenta(),
                        pub_marker,
                        st.name.cyan(),
                        st.fields.join(", ").bright_black()
                    );
                }
                if crawled.structs.len() > 15 {
                    println!("  ... y {} más", crawled.structs.len() - 15);
                }
                println!("{}", "└─────────────────────────────────────────────┘".magenta().bold());
                println!();
            }

            // Mostrar traits
            if !crawled.traits.is_empty() {
                println!("{}", "┌─ TRAITS DEFINIDAS ──────────────────────────┐".cyan().bold());
                for tr in crawled.traits.iter().take(15) {
                    println!("  {} {} with {} methods",
                        "╬".cyan(),
                        tr.name.yellow(),
                        tr.methods.len()
                    );
                }
                if crawled.traits.len() > 15 {
                    println!("  ... y {} más", crawled.traits.len() - 15);
                }
                println!("{}", "└─────────────────────────────────────────────┘".cyan().bold());
                println!();
            }

            // Mostrar TODOs y FIXMEs
            if !crawled.todos.is_empty() {
                println!("{}", "┌─ TAREAS PENDIENTES (TODO/FIXME) ────────────┐".yellow().bold());
                for todo in crawled.todos.iter().take(15) {
                    println!("  {} {} ({}:{})",
                        "⚠".yellow(),
                        todo.text.yellow(),
                        todo.file.bright_black(),
                        todo.line
                    );
                }
                if crawled.todos.len() > 15 {
                    println!("  ... y {} más", crawled.todos.len() - 15);
                }
                println!("{}", "└─────────────────────────────────────────────┘".yellow().bold());
                println!();
            }

            // Análisis de dead code
            let dead_items = scan_deadcode(&args.project);

            let mut filtered = dead_items.clone();
            if *functions {
                filtered.retain(|item| item.item_type == "function");
            } else if *structs {
                filtered.retain(|item| item.item_type == "struct");
            } else if *enums {
                filtered.retain(|item| item.item_type == "enum");
            }

            if !filtered.is_empty() {
                println!("{}", "┌─ CÓDIGO POTENCIALMENTE MUERTO ──────────────┐".red().bold());
                for item in filtered.iter().take(20) {
                    let pub_marker = if item.is_pub { "pub " } else { "" };
                    println!("{} {} {} ({}:{})",
                        "  ✗".red(),
                        item.item_type.red().bold(),
                        format!("{}{}", pub_marker, item.name).bright_red(),
                        item.file.bright_black(),
                        item.line
                    );
                }
                if filtered.len() > 20 {
                    println!("  ... y {} más", filtered.len() - 20);
                }
                println!("{}", "└─────────────────────────────────────────────┘".red().bold());
            }

            // Usar cargo check como fallback
            cmd.arg("check");
            cmd.arg("--all");
            "deadcode"
        }
        Some(CargoCommand::Mock { trait_name, output, mockito: _mockito, derive: _derive }) => {
            println!("{} {} Generando mocks y analizando traits...", "→".blue().bold(), Emoji("🎭", ""));
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner} {msg}").unwrap()
            );
            spinner.set_message("Extrayendo traits del proyecto...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            // Extraer todos los traits del proyecto
            let crawled = advanced_project_crawler(&args.project);

            spinner.finish_with_message(format!(
                "✓ Encontrados {} traits en el proyecto",
                crawled.traits.len()
            ));
            println!();

            // Mostrar todos los traits disponibles
            if !crawled.traits.is_empty() {
                println!("{}", "┌─ TRAITS DISPONIBLES PARA MOCK ──────────────┐".cyan().bold());
                for (i, tr) in crawled.traits.iter().enumerate() {
                    println!("  {} {} ({}:{})",
                        format!("{}.", i+1).bright_black().bold(),
                        tr.name.yellow().bold(),
                        tr.file.bright_black(),
                        tr.methods.len()
                    );
                    for method in &tr.methods {
                        println!("    {} {}", "→".cyan(), method);
                    }
                }
                println!("{}", "└─────────────────────────────────────────────┘".cyan().bold());
                println!();
            }

            if let Some(trait_n) = trait_name {
                let mocks = scan_and_generate_mocks(&args.project, trait_n);

                if !mocks.is_empty() {
                    println!("{}", "┌─ MOCK GENERADO ─────────────────────────────┐".green().bold());
                    for mock in &mocks {
                        println!("  {} {}", "Trait:".green(), mock.trait_name.cyan().bold());
                        println!("  {} {}", "Métodos:".green(), mock.methods.join(", ").bright_green());

                        if let Some(out) = output {
                            println!("  {} {}", "Guardado en:".yellow(), out.bright_yellow());
                            let _ = fs::write(out, &mock.generated_code);
                        } else {
                            println!("  {} {} bytes de código generado", "Tamaño:".blue(), mock.generated_code.len());
                        }
                    }
                    println!("{}", "└─────────────────────────────────────────────┘".green().bold());
                } else {
                    println!("{} {}", "⚠ No se encontró trait:".yellow().bold(), trait_n);
                }
            } else {
                println!("{}", "⚠ Especifica un trait con --trait-name".yellow().bold());
            }
            println!();

            cmd.arg("check");
            cmd.arg("--tests");
            "mock"
        }
        Some(CargoCommand::Modules { unused_only, with_deps: _with_deps, tree, depth }) => {
            println!("{} {} Analizando módulos...", "→".blue().bold(), Emoji("📦", ""));
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner} {msg}").unwrap()
            );
            spinner.set_message("Escaneando estructura...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            let modules = scan_modules(&args.project);

            spinner.finish_with_message(format!(
                "✓ Encontrados {} módulos",
                modules.len()
            ));
            println!();

            if *tree {
                println!("{}", "src/".bright_cyan().bold());
                for (idx, module) in modules.iter().enumerate() {
                    let is_last = idx == modules.len() - 1;
                    let prefix = if is_last { "└──" } else { "├──" };
                    let status = if module.used { "✓".green() } else { "✗".red() };
                    println!("{} {} {} ({} files)",
                        prefix.bright_black(),
                        status,
                        module.name.cyan(),
                        module.file_count
                    );
                }
            } else {
                for module in &modules {
                    let status = if module.used { "✓".green() } else { "✗".red() };
                    if !*unused_only || !module.used {
                        println!("{} {} - {} archivos",
                            status,
                            module.name.cyan(),
                            module.file_count
                        );
                    }
                }
            }
            println!();

            cmd.arg("tree");
            if let Some(d) = depth {
                cmd.args(&["--depth", &d.to_string()]);
            }
            "modules"
        }
        Some(CargoCommand::Preflight) => {
            println!("{} {} Iniciando secuencia de PREFLIGHT", "→".cyan().bold(), Emoji("🚀", ""));
            println!();

            let steps = vec![
                ("Verificando formato", "fmt"),
                ("Analizando con Clippy", "clippy"),
                ("Ejecutando tests", "test"),
                ("Compilando release", "build"),
            ];

            let pb = ProgressBar::new(4);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{bar:30.cyan/blue}] {pos}/4 {msg}").unwrap()
                    .progress_chars("█▓░")
            );

            // 1. Check Format
            pb.set_message(steps[0].0);
            match Command::new("cargo").args(&["fmt", "--check"]).current_dir(&args.project).status() {
                Ok(status) if !status.success() => {
                    pb.finish_with_message("❌ Formato incorrecto");
                    eprintln!("{} Ejecuta 'trae fmt' para corregir", "!".red());
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"Formato incorrecto".to_vec(),
                        stderr: b"".to_vec(),
                    };
                    return ("preflight-fmt-failed", output);
                }
                Err(e) => {
                    pb.finish_with_message("❌ Error");
                    eprintln!("{} Error ejecutando cargo fmt: {}", "✗".red(), e);
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"".to_vec(),
                        stderr: format!("Error: {}", e).into_bytes(),
                    };
                    return ("preflight-error", output);
                }
                _ => {}
            }
            pb.inc(1);

            // 2. Clippy
            pb.set_message(steps[1].0);
            match Command::new("cargo").args(&["clippy", "--", "-D", "warnings"]).current_dir(&args.project).status() {
                Ok(status) if !status.success() => {
                    pb.finish_with_message("❌ Clippy detectó problemas");
                    eprintln!("{} Clippy encontró problemas de código", "!".red());
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"Clippy detected problems".to_vec(),
                        stderr: b"".to_vec(),
                    };
                    return ("preflight-clippy-failed", output);
                }
                Err(e) => {
                    pb.finish_with_message("❌ Error");
                    eprintln!("{} Error ejecutando clippy: {}", "✗".red(), e);
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"".to_vec(),
                        stderr: format!("Error: {}", e).into_bytes(),
                    };
                    return ("preflight-error", output);
                }
                _ => {}
            }
            pb.inc(1);

            // 3. Tests
            pb.set_message(steps[2].0);
            match Command::new("cargo").arg("test").current_dir(&args.project).status() {
                Ok(status) if !status.success() => {
                    pb.finish_with_message("❌ Tests fallaron");
                    eprintln!("{} Los tests no pasaron", "!".red());
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"Tests failed".to_vec(),
                        stderr: b"".to_vec(),
                    };
                    return ("preflight-test-failed", output);
                }
                Err(e) => {
                    pb.finish_with_message("❌ Error");
                    eprintln!("{} Error ejecutando tests: {}", "✗".red(), e);
                    let output = Output {
                        status: std::process::ExitStatus::default(),
                        stdout: b"".to_vec(),
                        stderr: format!("Error: {}", e).into_bytes(),
                    };
                    return ("preflight-error", output);
                }
                _ => {}
            }
            pb.inc(1);

            // 4. Build Release
            pb.set_message(steps[3].0);
            pb.inc(1);
            pb.finish_with_message("✓ Preflight completado - procediendo a build");
            println!();

            cmd.arg("build");
            cmd.arg("--release");
            "preflight"
        }
        Some(CargoCommand::Repair) => {
            println!("{} {} Iniciando secuencia de REPARACIÓN", "→".cyan().bold(), Emoji("🔧", ""));
            println!();

            let steps = vec!["Aplicando cargo fix", "Aplicando formato", "Aplicando clippy fix"];

            for (idx, step) in steps.iter().enumerate() {
                let spinner = ProgressBar::new_spinner();
                spinner.set_style(
                    ProgressStyle::default_spinner()
                        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                        .template("{spinner} {msg}").unwrap()
                );
                spinner.set_message(format!("{}/{} {}", idx + 1, 3, step));
                spinner.enable_steady_tick(std::time::Duration::from_millis(100));

                match idx {
                    0 => {
                        if let Err(e) = Command::new("cargo").args(&["fix", "--allow-dirty", "--allow-staged"]).current_dir(&args.project).status() {
                            spinner.finish_with_message(format!("⚠ {}: {}", step, e));
                        } else {
                            spinner.finish_with_message(format!("✓ {}", step));
                        }
                    }
                    1 => {
                        if let Err(e) = Command::new("cargo").arg("fmt").current_dir(&args.project).status() {
                            spinner.finish_with_message(format!("⚠ {}: {}", step, e));
                        } else {
                            spinner.finish_with_message(format!("✓ {}", step));
                        }
                    }
                    2 => {
                        if let Err(e) = Command::new("cargo").args(&["clippy", "--fix", "--allow-dirty", "--allow-staged"]).current_dir(&args.project).status() {
                            spinner.finish_with_message(format!("⚠ {}: {}", step, e));
                        } else {
                            spinner.finish_with_message(format!("✓ {}", step));
                        }
                    }
                    _ => {}
                }
            }
            println!();

            cmd.arg("build");
            "repair"
        }
        Some(CargoCommand::WebSearch { query, limit, include_code, rust_docs, crates }) => {
            println!("{} {} Buscando '{}' en internet...", "→".blue().bold(), Emoji("🌐", ""), query.cyan().bold());
            println!();

            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::default_spinner()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                    .template("{spinner} {msg}").unwrap()
            );
            spinner.set_message("Consultando JARVIXSERVER...");
            spinner.enable_steady_tick(std::time::Duration::from_millis(100));

            // Construir la consulta de búsqueda
            let mut search_query = query.clone();
            if *rust_docs {
                search_query = format!("rust {} site:docs.rs OR site:doc.rust-lang.org", query);
            } else if *crates {
                search_query = format!("{} site:crates.io", query);
            }

            // Hacer petición a JARVIXSERVER para búsqueda web
            let client = Client::new();
            let endpoint = format!("{}/search/web", args.jarvix);

            let search_request = serde_json::json!({
                "query": search_query,
                "limit": limit,
                "include_code": include_code,
                "source": if *rust_docs { "rust_docs" } else if *crates { "crates" } else { "web" }
            });

            match client
                .post(&endpoint)
                .json(&search_request)
                .header("Content-Type", "application/json")
                .header("X-TRAE-Version", "0.2.0")
                .timeout(std::time::Duration::from_secs(30))
                .send()
                .await
            {
                Ok(resp) => {
                    match resp.status().as_u16() {
                        200..=299 => {
                            match resp.json::<serde_json::Value>().await {
                                Ok(json_response) => {
                                    spinner.finish_with_message("✓ Búsqueda completada".green().to_string());

                                    // Procesar y mostrar resultados
                                    if let Some(results) = json_response.get("search_results").and_then(|r| r.as_array()) {
                                        println!();
                                        println!("{}", "┌─ RESULTADOS DE BÚSQUEDA ─────────────────────┐".cyan().bold());

                                        for (i, result) in results.iter().enumerate() {
                                            if i >= *limit { break; }

                                            let title = result.get("title").and_then(|t| t.as_str()).unwrap_or("Sin título");
                                            let url = result.get("url").and_then(|u| u.as_str()).unwrap_or("");
                                            let snippet = result.get("snippet").and_then(|s| s.as_str()).unwrap_or("");

                                            println!("  {}. {} {}", (i+1).to_string().bright_yellow().bold(), title.cyan().bold(), format!("({})", url).bright_black());
                                            if !snippet.is_empty() {
                                                println!("     {}", snippet.bright_white());
                                            }

                                            if *include_code {
                                                if let Some(code) = result.get("code").and_then(|c| c.as_str()) {
                                                    println!("     {} {}", "💻".green(), code.bright_green());
                                                }
                                            }
                                            println!();
                                        }

                                        println!("{}", "└─────────────────────────────────────────────┘".cyan().bold());
                                        println!("{} {} resultados encontrados", "ℹ".blue(), results.len());
                                    } else {
                                        println!("{} No se encontraron resultados", "⚠".yellow());
                                    }
                                }
                                Err(e) => {
                                    spinner.finish_with_message("✗ Error procesando respuesta JSON".red().to_string());
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                        404 => {
                            spinner.finish_with_message("✗ BrowserMCP no disponible (404)".red().to_string());
                            eprintln!("{} El servicio BrowserMCP no está disponible en JARVIXSERVER", "!".red());
                            eprintln!("{} Verifica que BrowserMCP esté ejecutándose en el puerto 3000", "💡".blue());
                        }
                        500..=599 => {
                            spinner.finish_with_message(format!("✗ Error del servidor: {}", resp.status()).red().to_string());
                        }
                        _ => {
                            spinner.finish_with_message(format!("✗ Error inesperado: {}", resp.status()).red().to_string());
                        }
                    }
                }
                Err(e) => {
                    spinner.finish_with_message("✗ Error de conexión".red().to_string());
                    eprintln!("{} No se pudo conectar a JARVIXSERVER: {}", "✗".red(), e);
                    eprintln!("{} Verifica que JARVIXSERVER esté ejecutándose en {}", "💡".blue(), args.jarvix);
                }
            }

            // No ejecutar comando cargo para este caso
            let output = Output {
                status: std::process::ExitStatus::default(),
                stdout: b"Web search completed".to_vec(),
                stderr: b"".to_vec(),
            };
            return ("websearch", output);
        }
        None => {
            eprintln!("{} Especifica un comando", "✗".red().bold());
            std::process::exit(1);
        }
    };

    println!("{} Ejecutando: {}", "→".yellow(), format!("cargo {}", cmd_name).bright_white());
    println!();

    let output = match cmd.output() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{} Error al ejecutar cargo: {}", "✗".red(), e);
            std::process::exit(1);
        }
    };

    (cmd_name, output)
}

/// Muestra el output de stdout y stderr
fn display_output(stdout: &str, stderr: &str) {
    if !stdout.is_empty() {
        println!("{}", "STDOUT:".cyan().bold());
        println!("{}", stdout);
    }

    if !stderr.is_empty() {
        println!("{}", "STDERR:".red().bold());
        println!("{}", stderr);
    }
}

/// Reporta el resultado a JARVIXSERVER con reintentos
async fn report_to_jarvix(args: &Args, result: &CommandResult) {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner} {msg}").unwrap()
    );
    spinner.set_message("Reportando a JARVIXSERVER...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let max_retries = 3;
    let mut attempt = 1;

    loop {
        let client = Client::new();
        let endpoint = format!("{}/commands/execute", args.jarvix);

        match client
            .post(&endpoint)
            .json(result)
            .header("Content-Type", "application/json")
            .header("X-TRAE-Version", "0.2.0")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(resp) => {
                match resp.status().as_u16() {
                    200..=299 => {
                        spinner.finish_with_message("✓ Reportado exitosamente".green().to_string());
                        return;
                    }
                    400 => {
                        spinner.finish_with_message("✗ Error: Solicitud inválida (400)".red().to_string());
                        return;
                    }
                    401 => {
                        spinner.finish_with_message("✗ Error: No autorizado (401)".red().to_string());
                        return;
                    }
                    404 => {
                        spinner.finish_with_message("✗ Error: Endpoint no encontrado (404)".red().to_string());
                        return;
                    }
                    500..=599 => {
                        if attempt < max_retries {
                            spinner.set_message(format!(
                                "⟳ Reintentando... {}/{} (Error {})",
                                attempt, max_retries, resp.status()
                            ));
                            attempt += 1;
                            tokio::time::sleep(std::time::Duration::from_secs(attempt as u64)).await;
                            continue;
                        } else {
                            spinner.finish_with_message(format!(
                                "✗ Error servidor después de {} intentos",
                                max_retries
                            ).red().to_string());
                            return;
                        }
                    }
                    _ => {
                        spinner.finish_with_message(format!(
                            "✗ Error inesperado: {}",
                            resp.status()
                        ).red().to_string());
                        return;
                    }
                }
            }
            Err(e) => {
                if attempt < max_retries {
                    spinner.set_message(format!(
                        "⟳ Reintentando... {}/{} ({})",
                        attempt, max_retries, e
                    ));
                    attempt += 1;
                    tokio::time::sleep(std::time::Duration::from_secs(attempt as u64)).await;
                    continue;
                } else {
                    spinner.finish_with_message(format!(
                        "⚠ No se pudo conectar a JARVIXSERVER después de {} intentos",
                        max_retries
                    ).yellow().to_string());
                    return;
                }
            }
        }
    }
}

/// Imprime el resumen final
fn print_summary(result: &CommandResult) {
    println!();
    println!("{}", "═".repeat(70));

    if result.success {
        println!("{} {} Éxito en {} ms",
            Emoji("✓", "✓").to_string().green().bold(),
            "Comando ejecutado correctamente".green().bold(),
            result.duration_ms
        );
    } else {
        println!("{} {} Falló con código {} en {} ms",
            Emoji("✗", "✗").to_string().red().bold(),
            "Comando fallido".red().bold(),
            result.exit_code,
            result.duration_ms
        );
    }
    println!("{}", "═".repeat(70));
}

/// Crawling Avanzado: Análisis semántico profundo del proyecto Rust
fn advanced_project_crawler(project_path: &PathBuf) -> CrawledInfo {
    let info = CrawledInfo {
        dependencies: extract_dependencies(project_path),
        functions: extract_functions(project_path),
        structs: extract_structs(project_path),
        traits: extract_traits(project_path),
        tests: extract_tests(project_path),
        todos: extract_todos(project_path),
        metrics: calculate_metrics(project_path),
    };
    info
}

/// Extrae dependencias del Cargo.toml
fn extract_dependencies(project_path: &PathBuf) -> Vec<String> {
    let cargo_path = project_path.join("Cargo.toml");
    let mut deps = Vec::new();

    if let Ok(content) = fs::read_to_string(&cargo_path) {
        let in_deps = Regex::new(r#"\[dependencies\]"#).unwrap();
        let dep_pattern = Regex::new(r#"^([a-z0-9_-]+)\s*="#).unwrap();
        let mut is_deps_section = false;

        for line in content.lines() {
            if in_deps.is_match(line) {
                is_deps_section = true;
                continue;
            }

            if line.starts_with('[') {
                is_deps_section = false;
                continue;
            }

            if is_deps_section {
                if let Some(caps) = dep_pattern.captures(line) {
                    deps.push(caps.get(1).unwrap().as_str().to_string());
                }
            }
        }
    }

    deps
}

/// Extrae funciones definidas en el proyecto (parse semántico)
fn extract_functions(project_path: &PathBuf) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return functions;
    }

    let fn_pattern = Regex::new(r#"(?m)^\s*(pub\s+)?(?:async\s+)?(?:unsafe\s+)?(?:extern\s+"[^"]*"\s+)?fn\s+([a-z_]\w*)\s*\(([^)]*)\)\s*(?:->?\s*([^{]+?))?\s*\{"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_path = entry.path().display().to_string();

            for (line_num, line) in content.lines().enumerate() {
                if let Some(caps) = fn_pattern.captures(line) {
                    let is_pub = caps.get(1).is_some();
                    let name = caps.get(2).unwrap().as_str().to_string();
                    let params_str = caps.get(3).unwrap().as_str();
                    let return_type = caps.get(4).map(|m| m.as_str().trim().to_string()).unwrap_or_else(|| "()".to_string());

                    let params: Vec<String> = params_str.split(',')
                        .map(|p| p.trim().to_string())
                        .filter(|p| !p.is_empty())
                        .collect();

                    functions.push(FunctionInfo {
                        name,
                        file: file_path.clone(),
                        line: line_num + 1,
                        is_pub,
                        params,
                        return_type,
                    });
                }
            }
        }
    }

    functions
}

/// Extrae structs definidas en el proyecto
fn extract_structs(project_path: &PathBuf) -> Vec<StructInfo> {
    let mut structs = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return structs;
    }

    let struct_pattern = Regex::new(r#"(?m)^\s*(pub\s+)?struct\s+([A-Z]\w*)\s*(?:\{([^}]*)\})?"#).unwrap();
    let field_pattern = Regex::new(r#"(\w+)\s*:\s*([^,}]+)"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_path = entry.path().display().to_string();

            for line in content.lines() {
                if let Some(caps) = struct_pattern.captures(line) {
                    let is_pub = caps.get(1).is_some();
                    let name = caps.get(2).unwrap().as_str().to_string();
                    let fields_str = caps.get(3).map(|m| m.as_str()).unwrap_or("");

                    let mut fields = Vec::new();
                    for field_cap in field_pattern.captures_iter(fields_str) {
                        let field_name = field_cap.get(1).unwrap().as_str().to_string();
                        fields.push(field_name);
                    }

                    structs.push(StructInfo {
                        name,
                        file: file_path.clone(),
                        fields,
                        is_pub,
                    });
                }
            }
        }
    }

    structs
}

/// Extrae traits definidas en el proyecto
fn extract_traits(project_path: &PathBuf) -> Vec<TraitInfo> {
    let mut traits = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return traits;
    }

    let trait_pattern = Regex::new(r#"(?m)^\s*pub\s+trait\s+([A-Z]\w*)\s*(?:\{([^}]*)\})?"#).unwrap();
    let method_pattern = Regex::new(r#"fn\s+([a-z_]\w*)"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_path = entry.path().display().to_string();

            for _ in content.lines() {
                if let Some(caps) = trait_pattern.captures(&content[..]) {
                    let name = caps.get(1).unwrap().as_str().to_string();
                    let trait_body = caps.get(2).map(|m| m.as_str()).unwrap_or("");

                    let mut methods = Vec::new();
                    for method_cap in method_pattern.captures_iter(trait_body) {
                        methods.push(method_cap.get(1).unwrap().as_str().to_string());
                    }

                    traits.push(TraitInfo {
                        name,
                        file: file_path.clone(),
                        methods,
                    });
                }
            }
        }
    }

    traits
}

/// Extrae tests del proyecto
fn extract_tests(project_path: &PathBuf) -> Vec<TestInfo> {
    let mut tests = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return tests;
    }

    let test_pattern = Regex::new(r#"#\[test\]|#\[tokio::test\]|#\[actix_rt::test\]"#).unwrap();
    let fn_pattern = Regex::new(r#"fn\s+([a-z_]\w*)"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_path = entry.path().display().to_string();

            for (line_num, line) in content.lines().enumerate() {
                if test_pattern.is_match(line) {
                    // La siguiente línea debe ser la función de test
                    let lines: Vec<&str> = content.lines().collect();
                    if line_num + 1 < lines.len() {
                        if let Some(caps) = fn_pattern.captures(lines[line_num + 1]) {
                            let name = caps.get(1).unwrap().as_str().to_string();
                            tests.push(TestInfo {
                                name,
                                file: file_path.clone(),
                                line: line_num + 2,
                            });
                        }
                    }
                }
            }
        }
    }

    tests
}

/// Extrae TODOs y FIXMEs del código
fn extract_todos(project_path: &PathBuf) -> Vec<TodoItem> {
    let mut todos = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return todos;
    }

    let todo_pattern = Regex::new(r#"//\s*(TODO|FIXME|BUG|HACK):\s*(.+)"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_path = entry.path().display().to_string();

            for (line_num, line) in content.lines().enumerate() {
                if let Some(caps) = todo_pattern.captures(line) {
                    let text = format!("[{}] {}", caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str());
                    todos.push(TodoItem {
                        text,
                        file: file_path.clone(),
                        line: line_num + 1,
                    });
                }
            }
        }
    }

    todos
}

/// Calcula métricas del proyecto
fn calculate_metrics(project_path: &PathBuf) -> ProjectMetrics {
    let src_path = project_path.join("src");
    let mut metrics = ProjectMetrics {
        total_lines: 0,
        total_functions: 0,
        total_structs: 0,
        total_traits: 0,
        total_tests: 0,
        code_files: 0,
        test_coverage_estimate: 0.0,
    };

    if !src_path.exists() {
        return metrics;
    }

    let fn_pattern = Regex::new(r#"fn\s+\w+"#).unwrap();
    let struct_pattern = Regex::new(r#"struct\s+\w+"#).unwrap();
    let trait_pattern = Regex::new(r#"trait\s+\w+"#).unwrap();
    let test_pattern = Regex::new(r#"#\[test\]|#\[tokio::test\]"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        metrics.code_files += 1;

        if let Ok(content) = fs::read_to_string(entry.path()) {
            metrics.total_lines += content.lines().count();
            metrics.total_functions += fn_pattern.find_iter(&content).count();
            metrics.total_structs += struct_pattern.find_iter(&content).count();
            metrics.total_traits += trait_pattern.find_iter(&content).count();
            metrics.total_tests += test_pattern.find_iter(&content).count();
        }
    }

    if metrics.total_functions > 0 {
        metrics.test_coverage_estimate = (metrics.total_tests as f64 / metrics.total_functions as f64) * 100.0;
    }

    metrics
}

/// Scanner: Detecta código muerto analizando los archivos .rs
fn scan_deadcode(project_path: &PathBuf) -> Vec<DeadCodeItem> {
    let mut dead_items = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return dead_items;
    }

    // Patrones para detectar código potencialmente muerto
    let fn_pattern = Regex::new(r#"^\s*(?:pub\s+)?fn\s+([a-z_]\w*)"#).unwrap();
    let struct_pattern = Regex::new(r#"^\s*(?:pub\s+)?struct\s+([A-Z]\w*)"#).unwrap();
    let enum_pattern = Regex::new(r#"^\s*(?:pub\s+)?enum\s+([A-Z]\w*)"#).unwrap();
    let const_pattern = Regex::new(r#"^\s*(?:pub\s+)?const\s+([A-Z_]\w+)"#).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            for (line_num, line) in content.lines().enumerate() {
                if line.contains("#[allow(dead_code)]") || line.contains("#[test]") {
                    continue;
                }

                if let Some(caps) = fn_pattern.captures(line) {
                    dead_items.push(DeadCodeItem {
                        item_type: "function".to_string(),
                        name: caps.get(1).unwrap().as_str().to_string(),
                        file: entry.path().display().to_string(),
                        line: line_num + 1,
                        is_pub: line.contains("pub"),
                    });
                }
                if let Some(caps) = struct_pattern.captures(line) {
                    dead_items.push(DeadCodeItem {
                        item_type: "struct".to_string(),
                        name: caps.get(1).unwrap().as_str().to_string(),
                        file: entry.path().display().to_string(),
                        line: line_num + 1,
                        is_pub: line.contains("pub"),
                    });
                }
                if let Some(caps) = enum_pattern.captures(line) {
                    dead_items.push(DeadCodeItem {
                        item_type: "enum".to_string(),
                        name: caps.get(1).unwrap().as_str().to_string(),
                        file: entry.path().display().to_string(),
                        line: line_num + 1,
                        is_pub: line.contains("pub"),
                    });
                }
                if let Some(caps) = const_pattern.captures(line) {
                    dead_items.push(DeadCodeItem {
                        item_type: "const".to_string(),
                        name: caps.get(1).unwrap().as_str().to_string(),
                        file: entry.path().display().to_string(),
                        line: line_num + 1,
                        is_pub: line.contains("pub"),
                    });
                }
            }
        }
    }

    dead_items
}

/// Scanner: Analiza módulos del proyecto
fn scan_modules(project_path: &PathBuf) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return modules;
    }

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
    {
        let mod_path = entry.path().to_string_lossy().to_string();
        let mod_name = entry.file_name().to_string_lossy().to_string();

        let file_count = WalkDir::new(entry.path())
            .into_iter()
            .filter(|e| e.as_ref().map_or(false, |f| f.path().extension().map_or(false, |ext| ext == "rs")))
            .count();

        if file_count > 0 {
            modules.push(ModuleInfo {
                name: mod_name,
                path: mod_path,
                used: true, // Simplificado
                sub_modules: Vec::new(),
                file_count,
            });
        }
    }

    modules
}

/// Scanner: Genera mocks a partir de traits en el código
fn scan_and_generate_mocks(project_path: &PathBuf, trait_name: &str) -> Vec<MockInfo> {
    let mut mocks = Vec::new();
    let src_path = project_path.join("src");

    if !src_path.exists() {
        return mocks;
    }

    let trait_pattern = Regex::new(&format!(r#"pub\s+trait\s+{}\s*\{{([^}}]*)}}"#, trait_name)).unwrap();

    for entry in WalkDir::new(&src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Some(caps) = trait_pattern.captures(&content) {
                let trait_body = caps.get(1).unwrap().as_str();
                let fn_pattern = Regex::new(r#"fn\s+(\w+)"#).unwrap();

                let mut methods = Vec::new();
                for cap in fn_pattern.captures_iter(trait_body) {
                    methods.push(cap.get(1).unwrap().as_str().to_string());
                }

                let mock_code = format!(
                    "#[derive(Mock)]\npub struct Mock{} {{\n{}\n}}\n",
                    trait_name,
                    methods.iter().map(|m| format!("    {}: MockFn,", m)).collect::<Vec<_>>().join("\n")
                );

                mocks.push(MockInfo {
                    trait_name: trait_name.to_string(),
                    methods,
                    generated_code: mock_code,
                });
            }
        }
    }

    mocks
}

/// Carga variables de entorno desde un archivo .env simple
fn load_env_file(project_path: &PathBuf) -> std::io::Result<std::collections::HashMap<String, String>> {
    let env_path = project_path.join(".env");
    if !env_path.exists() {
        return Ok(std::collections::HashMap::new());
    }

    let content = std::fs::read_to_string(env_path)?;
    let mut vars = std::collections::HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().trim_matches('"').trim_matches('\'').to_string();
            vars.insert(key, value);
        }
    }

    Ok(vars)
}
