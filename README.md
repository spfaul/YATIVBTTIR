# Yet Another Terminal Image Viewer But This Time In Rust (YATIVBTTIR)

![demo](https://user-images.githubusercontent.com/69741305/142181682-7d2b8ce1-22f9-4635-af94-b9dbab16e071.gif)

Because there totally aren't hundreds of repos that do the same thing but better.
Displays RGB images in the terminal (terminal must have truecolor support).

## Building

```
git clone https://github.com/t0a5ted/YATIVBTTIR.git
cd YATIVBTTIR
cargo build --release
./target/release/yativbttir
```

#### Optional (BASH): Create Alias for easy access.
Add the following line to your `.bashrc` or `.bash_aliases`
```
alias tiv=./installdir/target/release/yativbttir
```
> change `installdir` to the root of your YATIVBTTR folder

Run YATIVBTTR with `tiv --help`

