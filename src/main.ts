import { createApp } from "vue";
import { appWindow } from '@tauri-apps/api/window';
import "./styles.scss";
import App from "./App.vue";

createApp(App).mount("#app");
