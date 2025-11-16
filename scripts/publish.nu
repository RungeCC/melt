def "main build" [] {
  cargo build --target wasm32-unknown-unknown --release
  cp target/wasm32-unknown-unknown/release/melt.wasm typst_package/
  cp README.md typst_package\
}

def "main publish" [
  version?: string,
  --build (-b),
  --local-typst-packages: path
] {

let cwd = (pwd)
let package_dir = $"($cwd)/typst_package"

let version = if $version == null {
  (open $"($package_dir)/typst.toml").package.version
}
let self = "melt"

if $build {
  main build
}

print $"Current version: ($version)"

if $local_typst_packages == null {
  let tmp = (mktemp -d)
  cd $tmp
  gh repo fork https://github.com/typst/packages --clone --fork-name "typst-packages" -- --depth 1 --single-branch
  cd typst-packages
} else {
  cd $local_typst_packages
}

let branch = $"($self)-($version)"
let dir = $"($self)/($version)"

git branch $branch
git checkout $branch
mkdir $dir
cp -r $"($package_dir)/.*" $dir
git add $dir
git commit -m $"($self):($version)"
git push --set-upstream origin $branch

cd $cwd
}

def main [] {
  print "no effect, use `build` or `publish`"
}
