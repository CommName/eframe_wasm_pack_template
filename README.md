# eframe_wasm_pack_template
This is a minimal template for eframe, a framework for writing apps using [egui](https://github.com/emilk/egui), built with wasm_pack using egui.

# Why wasmpack
The official repository documentation specifies the use of Trunk for developing egui applications. However, as of the current update, there is no straightforward method available for passing arguments to your application. To overcome this limitation, a workaround has been implemented by creating a library that supports argument passing. Given that Trunk does not facilitate building libraries, wasm-pack was chosen as an alternative solution.

# Prerequirments

Project is complied using [wasm-pack](https://rustwasm.github.io/wasm-pack/) please make sure you download the latest version

Other dependenices:
On linux you will have to install the following dependenices:
```
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```
On Fedora Rawhide you need to run:
```
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

# Getting Started

Change the name of the crate: Choose a good name for your project, and change the name to it in:
* `Cargo.toml`
    * Change the `package.name` from `eframe_wasm_pack_template` to `your_crate`.
    * Change the `package.authors`
* `index.html`
    * Change the name of wasm and js files in head:
        * ` href="./pkg/eframe_wasm_pack_template.js"` to ` href="./pkg/your_crate.js"`.
        * ` href="./pkg/eframe_wasm_pack_template_bg.wasm"` to ` href="../pkg/your_crate_bg.wasm"`. 
    * Change the name of loaded files in script
        * ` ... from './pkg/eframe_wasm_pack_template.js'` to ` ... from './pkg/your_crate.js'"`.
        * `  init('./pkg/eframe_wasm_pack_template_bg.wasm')` to `  init('./pkg/your_crate_bg.wasm')`. 


# Web deployment

Run the following command to comiple to wasm module
```
wasm-pack build --no-typescript -t web
```
Explanation of command:
- `--no-typescript` Since we are using pure HTML and JS we do not need ts file
- `-t web` Web will generate wasm module that can be run directly from HTML, feel free to explore other targets if they suite your project more

After building make sure that all the files are correctly linked inside `index.html` file, and that is it. All files can be served on web server out of the box




## Credits
This repo was inspired by offical [eframe_template](https://github.com/emilk/eframe_template/) go check it out