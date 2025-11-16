/// Common utilities used across the application

/// Check if an extension is for a text-based file (whitelist approach)
pub fn is_text_extension(extension: &str) -> bool {
    match extension.to_lowercase().as_str() {
        // Text-based programming languages
        "rs" | "py" | "js" | "ts" | "gleam" | "java" | "cpp" | "c" | "h" | "cs" | "go" | "rb" | "php" |
        "swift" | "kt" | "groovy" | "scala" | "jsx" | "tsx" | "ps1" | "rsbuild" | "cson" | "coffee" |
        "ue4game" | "uex" | "graphql" | "gql" | "pug" | "jade" | "handlebars" | "hbs" | "mustache" | "hjson" |
        "elm" | "clj" | "cljs" | "cljc" | "mjml" | "wxml" | "swig" | "twig" | "jinja" => true,

        // Shell scripts
        "sh" | "bash" | "zsh" | "fish" | "M" | "bat" | "cmd" | "btn" | "vbs" => true,

        // Configuration files
        "cfg" | "config" | "ini" | "toml" | "env" | "conf" | "rc" | "vim" | "bashrc" | "zshrc" |
        "fishconfig" | "gitconfig" | "gitignore" => true,

        // Documentation and markup
        "rst" | "adoc" | "asciidoc" => true,
        "md" | "markdown" | "mdown" | "mkdown" | "mkdn" | "mdwn" | "mmd" | "mdbase" | "mdtext" | "mdoc" => true,

        // Web files
        "html" | "htm" | "xml" | "xhtml" | "xsd" | "xsl" | "css" | "sass" | "scss" | "less" => true,

        // Other text formats
        "csv" | "tsv" | "log" | "sql" | "pl" | "pm" => true,

        // Metadata and other
        "yaml" | "yml" | "json" | "txt" | "text" => true,

        // Other languages
        "d" | "glam" => true,

        _ => false,
    }
}
