def "main build" [] {
  cargo build --target wasm32-unknown-unknown --release
  cp target/wasm32-unknown-unknown/release/melt.wasm typst_package/
  cp README.md typst_package\
}

def "main publish" [version?: string, --build (-b)] {
let self = "melt"
let fork =  $"typst-packages-($self)-($version)"

let cwd = (pwd)
let package_dir = $"($cwd)/typst_package"

let version = if $version == null {
  (open $"($package_dir)/typst.toml").package.version
}

if $build {
  main build
}

print $"Current version: ($version)"

cd (mktemp -d)
gh repo fork https://github.com/typst/packages --clone --fork-name $fork
cd $fork

let dir = $"($self)/($version)"

mkdir $dir
cp -r $"($package_dir)/.*" $dir
git add $dir
git commit -m $"($self):($version)"
git push

cd $cwd
}

def main [] {
  print "no effect, use `build` or `publish`"
}
