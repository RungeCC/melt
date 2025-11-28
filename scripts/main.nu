def "main release" [--output: path, --yes] {
  cargo build --target wasm32-unknown-unknown --release
  let release_path = if $output == null {
    "release/"
  } else {
    $output
  }
  print $"Write to ($release_path)\n"
  if not $yes and (input "Type YES to continue: ") != "YES" {
    print "Write cancelled.\n"
    exit -1
  }
  mkdir $release_path
  cp typst_package/* $release_path -r
  cp target/wasm32-unknown-unknown/release/melt.wasm $release_path
  cp LICENSE $release_path
  $"(open --raw README.md)\n(open --raw CHANGELOGS.md)" | save --force ($release_path | path join "README.md")
}

def "main clean" [--output: path, --cargo, --yes] {
  let release_path = if $output == null {
    "release/"
  } else {
    $output
  }
  print $"Clean ($release_path).\n"
  if not $yes and (input "Type YES to continue: ") != "YES" {
    print "Clean cancelled.\n"
    exit -1
  }
  if $cargo {
    cargo clean
  }
  rm -r $release_path
}

def "main publish" [
  version?: string,
  --release (-r),
  --local-typst-packages: path,
  --yes
] {
  print $"Starting publish.\n"
  if not $yes and (input "Type YES to continue: ") != "YES" {
    print "Cancelled.\n"
    exit -1
  }

  let cwd = (pwd)
  let package_dir = $"($cwd)/typst_package"

  let version = if $version == null {
    (open $"($package_dir)/typst.toml").package.version
  }
  let self = "melt"

  if $release {
    main release --yes=$yes
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

  if not $yes and (input "Type YES to continue: ") != "YES" {
    print "Cancelled.\n"
    exit -1
  }

  cd $cwd
}

def main [] {
  print "no effect, use `build` or `publish`"
}

def "main help" [] {
  print "release  [--output path] [--yes]"
  print "publish  [version] [--release] [--local-typst-packages path] [--yes]"
  print "clean    [--output path] [--cargo] [--yes]"
}
