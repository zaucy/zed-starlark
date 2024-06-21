let version = open cargo.toml | get package.version;
rm -rf extensions;
gh repo clone zed-industries/extensions;
open extensions/extensions.toml --raw
    | str replace --multiline '(submodule\s*=\s*"extensions/starlark"\s+version\s*=\s*)(".*")' $'${1}"($version)"'
    | save extensions/extensions.toml --force;
cd extensions;
git submodule update --init extensions/starlark;
git checkout
cd extensions/starlark;
git checkout $version;
cd ...;
git add -A;
git checkout -b $"chore/update-starlark-($version)";
git commit -m $"Update starlark extension to ($version)";
gh repo set-default zed-industries/extensions;
gh pr create --fill;
gh pr view --web;
cd ..;
