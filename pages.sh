cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=raptors\">" > target/doc/raptors/index.html
cp -r target/doc/raptors ./docs
