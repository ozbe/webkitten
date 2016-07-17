
pub trait ApplicationUI: Sized {

    /// Create a new UI
    fn new(engine: super::Engine) -> Option<Self>;

    /// UI event handler
    fn event_handler(&self) -> &super::Engine;

    /// Initialize all needed UI functions
    fn run(&self);

    /// Copy text to the system clipboard
    fn copy(&self, text: &str);


    /// The index of the focused window
    fn focused_window_index(&self) -> u8;

    /// Number of open windows
    fn window_count(&self) -> u8;

    /// Open a new window
    fn open_window(&self, uri: Option<&str>);

    /// Close a window
    fn close_window(&self, index: u8);

    /// Focus window at index
    fn focus_window(&self, index: u8);

    /// Set window visibility
    fn toggle_window(&self, index: u8, visible: bool);

    /// Change the dimensions of a specified window
    fn resize_window(&self, window_index: u8, width: u32, height: u32);

    /// Text in the command bar of a specified window
    fn command_field_text(&self, window_index: u8) -> String;

    /// Set the text in the command bar of a specified window
    fn set_command_field_text(&self, window_index: u8, text: &str);

    /// Title of a specified window
    fn window_title(&self, window_index: u8) -> String;

    /// Set the title of a specified window
    fn set_window_title(&self, window_index: u8, title: &str);


    /// Index of the webview currently visible in a specified window
    fn focused_webview_index(&self, window_index: u8) -> u8;

    /// Number of webviews in a window
    fn webview_count(&self, window_index: u8) -> u8;

    /// Open a new webview in a specified window
    fn open_webview(&self, window_index: u8, uri: &str);

    /// Close a webview in a specified window
    fn close_webview(&self, window_index: u8, webview_index: u8);

    /// Focus a webview in a specified window, hiding the current webview
    fn focus_webview(&self, window_index: u8, webview_index: u8);

    /// Reload a webview in a specified window
    fn reload_webview(&self, window_index: u8, webview_index: u8, disable_filters: bool);

    /// Load a URI in a webview
    fn set_uri(&self, window_index: u8, webview_index: u8, uri: &str);

    /// Go back to the previously loaded resource in a webview
    fn go_back(&self, window_index: u8, webview_index: u8) -> bool;

    /// Go forward to the next loaded resource in a webview
    fn go_forward(&self, window_index: u8, webview_index: u8) -> bool;

    /// Get the currently loaded URI or empty string
    fn uri(&self, window_index: u8, webview_index: u8) -> String;

    /// Find a string within the selected web view
    fn find_string(&self, window_index: u8, webview_index: u8, query: &str);

    /// Hide results from a previous find invocation (if applicable)
    fn hide_find_results(&self, window_index: u8, webview_index: u8);

    /// Get the title of the currently loaded URI or empty string
    fn webview_title(&self, window_index: u8, webview_index: u8) -> String;

    /// Run a JavaScript snippet in a webview
    fn run_javascript(&self, window_index: u8, webview_index: u8, script: &str);

    /// Apply a stylesheet to a webview
    fn apply_styles(&self, window_index: u8, webview_index: u8, styles: &str);
}

pub enum CommandError {
    /// No command matches the given text
    CommandNotFound,
    /// Command execution halted with an error
    ErrorDuringExecution,
    /// The provided arguments were invalid in the context of the given command
    InvalidArguments,
    /// There was no command text specified
    NoCommandSpecified,
}

pub struct CommandOutput {
    pub error: Option<CommandError>,
    pub message: Option<String>,
}

#[derive(Debug,Copy,Clone)]
pub enum URIEvent {
    Fail,
    Load,
    Request,
}

pub trait EventHandler {

    /// Handle a Return key press within the command bar
    fn execute_command<T: ApplicationUI>(&self, ui: &T, window_index: u8, text: &str)
        -> CommandOutput;

    /// Close the application
    fn close<T: ApplicationUI>(&self, ui: &T);

    /// Get available commands and/or arguments given a prefix
    fn command_completions<T: ApplicationUI>(&self, ui: &T, prefix: &str) -> Vec<String>;

    /// Handle a document load event in a webview.
    ///
    /// ## Events
    ///
    /// * `URIEvent::Request`: Invoke before document begins loading
    /// * `URIEvent::Load`: Invoke after document finishes loading but not
    ///   necessarily after subresources load
    /// * `URIEvent::Fail`: Invoke after a document fails to load
    fn on_uri_event<T: ApplicationUI>(&self,
                                      ui: &T,
                                      window_index: u8,
                                      webview_index: u8,
                                      uri: &str,
                                      event: URIEvent);
}

pub trait BrowserConfiguration: Sized {

    /// Parse a string literal into a `BrowserConfiguration`
    fn parse(raw_input: &str) -> Option<Self>;

    /// The page opened with each new window or empty buffer based on
    /// `window.start-page`
    fn start_page(&self) -> Option<String> {
        self.lookup_str("window.start-page")
    }

    /// The directory to replace instances of CONFIG_DIR in the configuration
    /// file
    fn config_dir(&self) -> Option<String> {
        self.lookup_raw_str("general.config-dir")
    }

    /// The name of a command resolving any matching alias in `commands.aliases`
    fn resolved_command_name(&self, name: &str) -> Option<String> {
        let command = self.lookup_str(&format!("commands.aliases.{}", name))
            .unwrap_or(String::from(name));
        if self.command_disabled(&command) { None } else { Some(command) }
    }

    fn command_matching_prefix(&self, text: &str) -> Option<String> {
        if text.len() > 0 {
            let key = format!("commands.on-text-change.\"{}\"", &text[.. 1]);
            if let Some(script) = self.lookup_str(&key) {
                return Some(format!("{} {}", script, &text[1 ..]))
            }
        }
        None
    }

    /// Whether a command is disabled based on `commands.disabled`
    fn command_disabled(&self, name: &str) -> bool {
        if let Some(disabled) = self.lookup_str_vec("commands.disabled") {
            return disabled.contains(&String::from(name));
        }
        false
    }

    /// The path to the content filter used in buffers based on
    /// `general.content-filter`
    fn content_filter_path(&self) -> Option<String> {
        self.lookup_str("general.content-filter")
    }

    /// Whether to skip content filtering based on the site-specific option
    /// `sites."[HOST]".skip-content-filter`.
    fn skip_content_filter(&self, uri: &str) -> bool {
        if self.content_filter_path().is_some() {
            self.lookup_site_bool(uri, "skip-content-filter").unwrap_or(false)
        } else {
            true
        }
    }

    /// Whether to enable private browsing based on the global option
    /// `general.private-browsing` and site-specific option
    /// `sites."[HOST]".private-browsing`. Defaults to `false`.
    fn use_private_browsing(&self, uri: &str) -> bool {
        if let Some(value) = self.lookup_site_bool(uri, "private-browsing") {
            return value;
        } else if let Some(mode) = self.lookup_bool("general.private-browsing") {
            return mode;
        }
        false
    }

    /// Whether to allow browser plugins to run in a buffer based on the global
    /// option `general.allow-plugins` and site-specific option
    /// `sites."[HOST]".allow-plugins`. Defaults to `false`.
    fn use_plugins(&self, uri: &str) -> bool {
        if let Some(value) = self.lookup_site_bool(uri, "allow-plugins") {
            return value;
        } else if let Some(mode) = self.lookup_bool("general.allow-plugins") {
            return mode;
        }
        false
    }

    /// Paths to search for command scripts using configuration option
    /// `command.search-paths`
    fn command_search_paths(&self) -> Vec<String> {
        self.lookup_str_vec("commands.search-paths").unwrap_or(vec![])
    }

    /// Command to run when no other commands are matched using configuration
    /// option `commands.default`
    fn default_command(&self) -> Option<String> {
        self.lookup_str("commands.default")
    }

    /// Commands triggered by a URI load event
    ///
    /// ## Events
    ///
    /// * `Load`: invokes all commands listed in `commands.on-load-uri`
    /// * `Request`: invokes all commands listed in `commands.on-request-uri`
    /// * `Fail`: invokes all commands listed in `commands.on-fail-uri`
    fn on_uri_event_commands(&self, event: URIEvent) -> Vec<String> {
        let key = match event {
            URIEvent::Load => "commands.on-load-uri",
            URIEvent::Request => "commands.on-request-uri",
            URIEvent::Fail => "commands.on-fail-uri",
        };
        self.lookup_str_vec(key).unwrap_or(vec![])
    }

    /// Look up the bool value of a configuration option matching key
    fn lookup_bool<'a>(&'a self, key: &'a str) -> Option<bool>;

    /// Look up the string value of a configuration option matching key,
    /// replacing string variables where possible
    fn lookup_str<'a>(&'a self, key: &'a str) -> Option<String>;

    /// Look up the string value of a configuration option without any
    /// substitutions
    fn lookup_raw_str<'a>(&'a self, key: &'a str) -> Option<String>;

    /// Look up the string vector value of a configuration option matching key
    fn lookup_str_vec(&self, key: &str) -> Option<Vec<String>>;

    /// Look up the bool value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`
    fn lookup_site_bool<'a>(&'a self, uri: &str, key: &'a str) -> Option<bool>;

    /// Look up the string value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`
    fn lookup_site_str<'a>(&'a self, uri: &str, key: &'a str) -> Option<String>;

    /// Look up the string vector value of a configuration option matching key
    /// formatted as `sites."[HOST]".[key]`
    fn lookup_site_str_vec<'a>(&'a self, uri: &str, key: &'a str) -> Option<Vec<String>>;
}
