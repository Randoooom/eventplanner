/*
 *     Copyright (C) 2023 Fritz Ochsmann
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published
 *     by the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const production = process.env.NODE_ENV === "production";
const generate = process.env.BUILD === "static";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
 devtools: { enabled: false },

 app: {
   head: {
     title: "EP",
     charset: "utf-8",
     viewport: "width=device-width, initial-scale=1",
     meta: [{ name: "format-detection", content: "telephone=no" }],
     style: [],
     script: [],
   },
 },

 i18n: {
   locales: [
     { code: "de", iso: "de-DE", file: "de.json" },
     { code: "en", iso: "en-US", file: "en.json" },
   ],
   lazy: true,
   langDir: "./locales",
   defaultLocale: "en",
   detectBrowserLanguage: {
     cookieKey: "lang",
     useCookie: true,
     redirectOn: "root",
     alwaysRedirect: true,
   },
 },

 css: [
   "vuetify/styles",
   "@mdi/font/css/materialdesignicons.min.css",
   "@/assets/main.sass",
 ],

 build: {
   transpile: ["vuetify", "lodash"],
 },

 modules: ["@nuxtjs/i18n", "@pinia/nuxt", "nuxt-lodash"],

 vite: {
   plugins: production ? [wasm(), topLevelAwait()] : [],
   optimizeDeps: {
     exclude: production ? [] : ["@surrealdb/wasm"],
   },
 },

 runtimeConfig: {
   public: {
     surrealdbEndpoint: "NUXT_PUBLIC_SURREALDB_ENDPOINT",
   },
 },

 ssr: !generate,
 compatibilityDate: "2024-12-29",
});
