import { mount } from 'svelte'
import "./main.css"
import "./font.css"
import App from './App.svelte'
import { init } from './lib/libdoodle/libdoodle.svelte'

(async () => {
  await init()

  mount(App, {
    target: document.body!,
  });
})()