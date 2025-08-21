# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Клонирование репозитория

```bash
git clone -b rust/5.7.x https://github.com/GravitLauncher/LauncherPrestarter.git
```
### Сборка модуля
- Перейдите в папку репозитория LauncherPrestarter
```bash
cd LauncherPrestarter
```
- Создать рабочее дерево
  - Папка с веткой модуля будет находится на уровне папки текущего репозитория
- Необходима 21-24 JDK версия Java
```bash
git worktree add ../LauncherPrestarterModule origin/modules/5.7.x
```
```bash
cd ../LauncherPrestarterModule
```
```
./gradlew build
```

### Установка на LaunchServer
Для использования Prestarter выполните следующие действия:
- Установите модуль `Prestarter_module.jar` на лаунчсервер в папку `modules`
- Соберите проект с помощью Cargo/yarn
- Поместите собранный файл в корень лаунчсервера с названием `Prestarter.exe`
