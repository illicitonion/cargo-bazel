"""Common utilities useful for unifying the behavior of different parts of `cargo-bazel`."""

load(
    "//private:rust_utils.bzl",
    _get_host_triple = "get_host_triple",
    _rust_get_rust_tools = "get_rust_tools",
)

get_host_triple = _get_host_triple

_EXECUTE_ERROR_MESSAGE = """\
Command {args} failed with exit code {exit_code}.
STDOUT ------------------------------------------------------------------------
{stdout}
STDERR ------------------------------------------------------------------------
{stderr}
"""

def execute(repository_ctx, args, env = {}):
    """A heler macro for executing some arguments and displaying nicely formatted errors

    Args:
        repository_ctx (repository_ctx): The rule's context object.
        args (list): A list of strings which act as `argv` for execution.
        env (dict, optional): Environment variables to set in the execution environment.

    Returns:
        struct: The results of `repository_ctx.execute`
    """
    result = repository_ctx.execute(
        args,
        environment = env,
        quiet = repository_ctx.attr.quiet,
    )

    if result.return_code:
        fail(_EXECUTE_ERROR_MESSAGE.format(
            args = args,
            exit_code = result.return_code,
            stdout = result.stdout,
            stderr = result.stderr,
        ))

    return result

def get_rust_tools(repository_ctx, host_triple):
    """Retrieve a cargo and rustc binary based on the host triple.

    Args:
        repository_ctx (repository_ctx): The rule's context object.
        host_triple (struct): A `@rules_rust//rust:triple.bzl%triple` object.

    Returns:
        struct: A struct containing the expected rust tools
    """

    # This is a bit hidden but to ensure Cargo behaves consistently based
    # on the user provided config file, the config file is installed into
    # the `CARGO_HOME` path. This is done so here since fetching tools
    # is expected to always occur before any subcommands are run.
    if repository_ctx.attr.isolated and repository_ctx.attr.cargo_config:
        cargo_home = cargo_home_path(repository_ctx)
        cargo_home_config = repository_ctx.path("{}/config.toml".format(cargo_home))
        cargo_config = repository_ctx.path(repository_ctx.attr.cargo_config)
        repository_ctx.symlink(cargo_config, cargo_home_config)

    return _rust_get_rust_tools(
        repository_ctx = repository_ctx,
        cargo_template = repository_ctx.attr.rust_toolchain_cargo_template,
        rustc_template = repository_ctx.attr.rust_toolchain_rustc_template,
        host_triple = host_triple,
        version = repository_ctx.attr.rust_version,
    )

def cargo_home_path(repository_ctx):
    """Define a path within the repository to use in place of `CARGO_HOME`

    Args:
        repository_ctx (repository_ctx): The rules context object

    Returns:
        path: The path to a directory to use as `CARGO_HOME`
    """
    return repository_ctx.path(".cargo_home")
