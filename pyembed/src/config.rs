// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Data structures for configuring a Python interpreter.

use {python3_sys as pyffi, std::ffi::CString};

/// Defines which allocator to use for the raw domain.
#[derive(Clone, Debug)]
pub enum PythonRawAllocator {
    /// Use jemalloc.
    Jemalloc,
    /// Use the Rust global allocator.
    Rust,
    /// Use the system allocator.
    System,
}

/// Defines Python code to run.
#[derive(Clone, Debug)]
pub enum PythonRunMode {
    /// No-op.
    None,
    /// Run a Python REPL.
    Repl,
    /// Run a Python module as the main module.
    Module { module: String },
    /// Evaluate Python code from a string.
    Eval { code: String },
    /// Execute Python code in a file.
    ///
    /// We define this as a CString because the underlying API wants
    /// a char* and we want the constructor of this type to worry about
    /// the type coercion.
    File { path: CString },
}

/// Defines `terminfo`` database resolution semantics.
#[derive(Clone, Debug)]
pub enum TerminfoResolution {
    /// Resolve `terminfo` database using appropriate behavior for current OS.
    Dynamic,
    /// Do not attempt to resolve the `terminfo` database. Basically a no-op.
    None,
    /// Use a specified string as the `TERMINFO_DIRS` value.
    Static(String),
}

/// Defines an extra extension module to load.
#[derive(Clone, Debug)]
pub struct ExtensionModule {
    /// Name of the extension module.
    pub name: CString,

    /// Extension module initialization function.
    pub init_func: unsafe extern "C" fn() -> *mut pyffi::PyObject,
}

/// Holds the configuration of an embedded Python interpreter.
///
/// Instances of this struct can be used to construct Python interpreters.
///
/// Each instance contains the total state to define the run-time behavior of
/// a Python interpreter.
#[derive(Clone, Debug)]
pub struct PythonConfig {
    /// Name of encoding for stdio handles.
    pub standard_io_encoding: Option<String>,

    /// Name of encoding error mode for stdio handles.
    pub standard_io_errors: Option<String>,

    /// Python optimization level.
    pub opt_level: i32,

    /// Whether to load our custom frozen importlib bootstrap modules.
    pub use_custom_importlib: bool,

    /// Whether to load the filesystem-based sys.meta_path finder.
    pub filesystem_importer: bool,

    /// Filesystem paths to add to sys.path.
    ///
    /// ``$ORIGIN`` will resolve to the directory of the application at
    /// run-time.
    pub sys_paths: Vec<String>,

    /// Controls whether to detect comparing bytes/bytearray with str.
    ///
    /// If 1, issues a warning. If 2 or greater, raises a BytesWarning
    /// exception.
    pub bytes_warning: i32,

    /// Whether to load the site.py module at initialization time.
    pub import_site: bool,

    /// Whether to load a user-specific site module at initialization time.
    pub import_user_site: bool,

    /// Whether to ignore various PYTHON* environment variables.
    pub ignore_python_env: bool,

    /// Whether to enter interactive mode after executing a script or a command.
    pub inspect: bool,

    /// Whether to put interpreter in interactive mode.
    pub interactive: bool,

    /// Whether to enable isolated mode.
    pub isolated: bool,

    /// If set, set the Windows filesystem encoding to mbcs and the filesystem
    /// error handler to replace.
    pub legacy_windows_fs_encoding: bool,

    /// Whether io.File instead of io.WindowsConsoleIO for sys.stdin, sys.stdout,
    /// and sys.stderr.
    pub legacy_windows_stdio: bool,

    /// Whether to suppress writing of ``.pyc`` files when importing ``.py``
    /// files from the filesystem. This is typically irrelevant since modules
    /// are imported from memory.
    pub write_bytecode: bool,

    /// Whether stdout and stderr streams should be unbuffered.
    pub unbuffered_stdio: bool,

    /// Whether to enable parser debugging output.
    pub parser_debug: bool,

    /// Whether to enable quiet mode.
    pub quiet: bool,

    /// Whether to use the PYTHONHASHSEED environment variable to initialize the
    /// hash seed.
    pub use_hash_seed: bool,

    /// Controls the level of the verbose mode for the interpreter.
    pub verbose: i32,

    /// Reference to packed resources data.
    ///
    /// The referenced data contains Python module data. It likely comes from an
    /// `include_bytes!(...)` of a file generated by PyOxidizer.
    ///
    /// The format of the data is defined by the ``python-packed-resources``
    /// crate. The data will be parsed as part of initializing the custom
    /// meta path importer during interpreter initialization.
    pub packed_resources: &'static [u8],

    /// Extra extension modules to make available to the interpreter.
    ///
    /// The values will effectively be passed to ``PyImport_ExtendInitTab()``.
    pub extra_extension_modules: Vec<ExtensionModule>,

    /// Whether to set sys.argvb with bytes versions of process arguments.
    ///
    /// On Windows, bytes will be UTF-16. On POSIX, bytes will be raw char*
    /// values passed to `int main()`.
    pub argvb: bool,

    /// Whether to set sys.frozen=True.
    ///
    /// Setting this will enable Python to emulate "frozen" binaries, such as
    /// those used by PyInstaller.
    pub sys_frozen: bool,

    /// Whether to set sys._MEIPASS to the directory of the executable.
    ///
    /// Setting this will enable Python to emulate PyInstaller's behavior
    /// of setting this attribute.
    pub sys_meipass: bool,

    /// Which memory allocator to use for the raw domain.
    pub raw_allocator: PythonRawAllocator,

    /// How to resolve the `terminfo` database.
    pub terminfo_resolution: TerminfoResolution,

    /// Environment variable holding the directory to write a loaded modules file.
    ///
    /// If this value is set and the environment it refers to is set,
    /// on interpreter shutdown, we will write a ``modules-<random>`` file to
    /// the directory specified containing a ``\n`` delimited list of modules
    /// loaded in ``sys.modules``.
    pub write_modules_directory_env: Option<String>,

    /// Defines what code to run by default.
    ///
    pub run: PythonRunMode,
}

impl Default for PythonConfig {
    /// Create a new instance using defaults.
    #[allow(unused)]
    fn default() -> Self {
        PythonConfig {
            standard_io_encoding: None,
            standard_io_errors: None,
            opt_level: 0,
            use_custom_importlib: false,
            filesystem_importer: false,
            sys_paths: vec![],
            bytes_warning: 0,
            import_site: false,
            import_user_site: false,
            ignore_python_env: true,
            inspect: false,
            interactive: false,
            isolated: false,
            legacy_windows_fs_encoding: false,
            legacy_windows_stdio: false,
            write_bytecode: false,
            unbuffered_stdio: false,
            parser_debug: false,
            quiet: false,
            use_hash_seed: false,
            verbose: 0,
            packed_resources: &[],
            extra_extension_modules: vec![],
            argvb: false,
            sys_frozen: false,
            sys_meipass: false,
            raw_allocator: if cfg!(windows) {
                PythonRawAllocator::System
            } else {
                PythonRawAllocator::Jemalloc
            },
            terminfo_resolution: TerminfoResolution::Dynamic,
            write_modules_directory_env: None,
            run: PythonRunMode::None,
        }
    }
}
