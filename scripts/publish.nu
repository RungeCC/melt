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

print $"Typst packages: (pwd)"

let branch = $"($self)-($version)"
let dir = $"packages/preview/($self)/($version)"

print $dir

git branch $branch
git checkout $branch
mkdir $dir
cp -r ($"($package_dir)/*" | into glob) $dir
git add $dir
git commit -m $"($self):($version)"
print "input YES to push"

if (input) == "YES" {
  git push --force --set-upstream origin $branch
} else {
  print "Cancelled"
}

cd $cwd
}

def main [] {
  print "no effect, use `build` or `publish`"
}
