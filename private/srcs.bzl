"""A generate file containing all source files used to produce `cargo-bazel`"""

# Each source file is tracked as a target so the `cargo_bootstrap_repository`
# rule will know to automatically rebuild if any of the sources changed.
CARGO_BAZEL_SRCS = [
    "@cargo_bazel//:tools/cargo_bazel/src/annotation.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/annotation/dependency.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/cli.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/cli/generate.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/cli/query.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/cli/splice.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/config.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/context.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/context/crate_context.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/context/platforms.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/lib.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/lockfile.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/main.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/metadata.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/template_engine.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/crate_build_file.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/module_build_file.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/module_bzl.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/aliases.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/binary.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/build_script.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/common_attrs.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/deps.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/library.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/crate/proc_macro.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/header.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/module/aliases_map.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/module/deps_map.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/module/repo_git.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/module/repo_http.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/starlark/glob.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/starlark/selectable_dict.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/rendering/templates/partials/starlark/selectable_list.j2",
    "@cargo_bazel//:tools/cargo_bazel/src/splicing.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/splicing/cargo_config.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/splicing/splicer.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/test.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/utils.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/utils/starlark.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/utils/starlark/glob.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/utils/starlark/label.rs",
    "@cargo_bazel//:tools/cargo_bazel/src/utils/starlark/select.rs",
]
