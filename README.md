# Create Vite RS

## Description
Create Vite RS is a Rust-based reimplementation of the popular `create-vite` tool. It's a command-line interface (CLI) application designed to generate various template projects for Vite, a next-generation frontend build tool.

Key features:
- Faithfully reproduces the functionality of `create-vite` in Rust
- Generates project templates for multiple frontend frameworks (Vue, React, Svelte, etc.)
- Offers both TypeScript and JavaScript variants for each framework
- Provides additional customization options (e.g., SWC for React, create-vue for Vue)
- Implements an interactive CLI using the `inquire` crate for improved user experience
- Maintains an extensible architecture for easy addition of new frameworks and variants

By leveraging Rust's performance and safety features, Create Vite RS aims to offer a robust and efficient alternative to the original Node.js-based `create-vite`, while maintaining full compatibility with Vite's ecosystem.

Create Vite RS 是流行的 `create-vite` 工具的 Rust 重新实现版本。它是一个命令行界面 (CLI) 应用程序，旨在为 Vite（一个新一代前端构建工具）生成各种模板项目。

主要特点：
- 用 Rust 忠实重现 `create-vite` 的功能
- 为多个前端框架（Vue、React、Svelte 等）生成项目模板
- 为每个框架提供 TypeScript 和 JavaScript 变体
- 提供额外的自定义选项（如 React 的 SWC、Vue 的 create-vue）
- 使用 `inquire` crate 实现交互式 CLI，提供更好的用户体验
- 保持可扩展的架构，方便添加新的框架和变体

通过利用 Rust 的性能和安全特性，Create Vite RS 旨在为原始的基于 Node.js 的 `create-vite` 提供一个强大而高效的替代方案，同时保持与 Vite 生态系统的完全兼容性。

## Run

```bash
cargo run
```

## Build

```bash
cargo build --release
```
