# rusty-blog

A note-taking web app based on the Yew framework that is based on Rust.

## Pre-requisites
Ensure the following services and tools are setup before deploying the app.
- Rust
- Trunk
- WebAssembly Target: wasm32-unknown-unknown 
- Go Backend
- PostgreSQL Database
- Node.js
- TailwindCSS

## Yew Setup
Follow the official [Yew guide](https://yew.rs/docs/getting-started/introduction) to setup the project and install the pre-requisites.

## TailwindCSS Setup
The TailwindCSS framework is used in this project, and hence, there is a need to setup and ensure the following is installed:
- TailwindCSS
- Autoprefixer 

<br>
This can be done by running the `npm install` command.

## Project Directory Structure
You may only need to focus on the following folders for dev and deployment.

|Directory|Description|
|---|---|
|./src|Contains the project's source code.|
|./src/components|Reusable Yew components.|

The `./src/components` directory contains all the components used to run the Yew app, such as API, and index components. <br>
The `router.rs` file contains all routes within the Yew web application. <br>
You may also see multiple `mod.rs` files which are used to allow importing of various `.rs` components across the project scope.

## Development Deployment 
Before deploying, ensure that the backend is being deployed in a seperate port to avoid collision.
<br>
Serve the Yew app via the command `trunk serve`.
<br>
To auto-populate the CSS file with styles, simply run `npx tailwindcss -i ./input.css -o ./src/output.css --watch`, as TailwindCSS
will only include the CSS classes that are used in the project.

