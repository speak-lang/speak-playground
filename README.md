# Speak Playground

The playground is hosted in Vercel. You can start interacting with this playground [here](https://deploy--leafy-piroshki-ccc0e1.netlify.app/).

## How to set up locally

1. You need the rust toolchain installed, you can find the instructions [here](https://www.rust-lang.org/learn/get-started) 
2. You will need to install the Webassembly target and the trunk tool for building the application locally, instructions can be found [here](https://yew.rs/docs/getting-started/introduction)
3. Building locally:

```sh
trunk serve --port 8089
```

The port flag can be changed to your liking, here I'm making an assumption that the port `8089` is free.

Adding classes to a node:

```html
 <div class={classes!("input", "output")}>
```
