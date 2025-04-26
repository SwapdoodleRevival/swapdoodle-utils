import { mount } from 'svelte'
import "./main.css"
import "./font.css"
import App from './App.svelte'
import { init } from './lib/parsing/parsing.svelte'

(async () => {
  await init()

  mount(App, {
    target: document.body!,
  });
})()