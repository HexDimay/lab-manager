import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from "element-plus";
import 'element-plus/dist/index.css';
import 'element-plus/theme-chalk/dark/css-vars.css'
import './assets/main.css';

createApp(App).use(ElementPlus, { size: "default",  }).mount("#app");
