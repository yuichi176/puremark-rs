# puremark-rs
puremark-rs is a super-fast and lightweight Markdown parser built with Rust.

It is provided as an npm package using WebAssembly (WASM), and works seamlessly in both browser and Node.js environments.

## ✨ Features
🚀 High performance powered by Rust

📦 Lightweight and minimal

🌐 Supports both Browser and Node.js

📜 Standard Markdown syntax support

🔧 Simple and easy-to-use API

## 📦 Installation
```bash
npm install puremark-rs
```
or
```bash
yarn add puremark-rs
```

## 🚀 Usage
```javascript
import { parseMarkdown } from 'puremark-rs';

const markdownText = `
# Hello World

This is puremark-rs!
`;

const html = parseMarkdown(markdownText);
console.log(html);
```
Output:

```html
<h1>Hello World</h1>
<p>This is puremark-rs!</p>
```

## 📚 API
`parseMarkdown(markdown: string): string`

- markdown: The Markdown text to parse 
- Returns: A string of HTML

## 📄 License
MIT License
