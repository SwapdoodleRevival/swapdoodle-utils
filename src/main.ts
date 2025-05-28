import { mount } from 'svelte'
import "./main.css"
import "./font.css"
import App from './App.svelte'

mount(App, {
  target: document.body!,
});