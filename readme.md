# 🖼️ pixIT — Terminal Image Editor in Rust

**pixIT** is a fast, flexible command-line image editor built with Rust 🦀. It allows you to perform a wide range of image transformations—right from your terminal!

> Navigate like a shell. Select your image. Transform it. All through the CLI.

---

## ✨ Features

- 🔍 **Blur** images with customizable intensity
- 🔁 **Rotate** by 90°, 180°, or 270°
- 🖤 Convert to **Grayscale**
- 🌞 Adjust **Brightness**
- 🎚️ Tune **Contrast**
- 🌈 Modify **Hue**
- 🔄 **Flip** horizontally or vertically
- 📏 **Resize** with custom width & height
- 🗂️ Use familiar shell-like navigation (`cd`, `ls`, `select <image>`)

---

## ⚙️ Requirements

- Rust & Cargo installed — [Install Rust](https://www.rust-lang.org/tools/install)

---

## 📦 Installation

Clone this repo and enter the directory:

```bash
git clone https://github.com/your-username/pixit.git
cd pixit
```

---

## 🚀 Usage

Run any image operation using:

```bash
cargo run -- <command> --para <value>
```

This drops you into an interactive terminal for selecting your image.

You can now use:

- `ls` – view current directory files
- `cd <folder>` – change directory
- `select <filename>` – select the image you want to edit

---

## 🔧 Supported Commands

| Command | Parameter(Ex) | Description |
|---------|-----------|---------|
| blur | 0.5 | Apply blur (0.1 to 1.0) |
| rotate | 90 / 180 / 270 | Rotate image |
| grayscale | 0 | Convert to grayscale (just pass 0) |
| brighten | -10 / 10 | Brightness adjustment |
| contrast | -5.0 / 10.0 | Contrast adjustment (float) |
| hue_rotation | 30 / -45 | Hue shift in degrees |
| flip | hor / ver | Flip horizontally or vertically |
| resize | 300/300 | New width/height (format: width/height) |

---

## 🧪 Examples

```bash
cargo run -- blur --para 0.5
cargo run -- rotate --para 180
cargo run -- grayscale --para 0
cargo run -- brighten --para 20
cargo run -- contrast --para -10.5
cargo run -- hue_rotation --para 60
cargo run -- flip --para ver
cargo run -- resize --para 800/600
```

---

## 🖱️ Image Selection in Terminal

After running any command, you'll see:

```bash
=> Now Select image (use `ls`, `cd <dir>`, and `select <image>`):
--> 
```

Use these commands to navigate:

- `ls` – list files and directories
- `cd foldername` – change directory
- `select image.jpg` – choose an image for editing

You can even use regular terminal commands like `clear`, `echo`, etc.

---

## 💾 Output

Once the operation completes, the edited image is saved with a modified name:

Examples:
- cat.jpg + rotate → cat_rotated.jpg
- dog.jpeg + blur → dog_blured.jpg
- sample.jpg + flip → sample_flipped.jpg

All edited images are saved in the same directory as the original.

---

## 📂 Folder Structure

```
pixit/
│
├── src/
│   ├── main.rs
│   └── Image_fns.rs
├── Cargo.toml
└── README.md
```

---

## 🤝 Contributing

Pull requests are welcome! Feel free to fork and improve the tool.

Suggestions, issues, and improvements are appreciated ✨

---

## 📄 License

Licensed under the MIT License.

---

## 👤 Author

Made with 💙 by Yash
