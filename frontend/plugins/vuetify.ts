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

import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as labs from "vuetify/labs/components";
import * as directives from "vuetify/directives";
import { defineNuxtPlugin } from "#imports";
import { de } from "vuetify/locale";

export default defineNuxtPlugin((nuxtApp) => {
  const vuetify = createVuetify({
    components: {
      ...components,
      ...labs,
    },
    directives,
    locale: {
      locale: "de",
      messages: { de },
    },
    theme: {
      defaultTheme: "dark",
      themes: {
        dark: {
          dark: true,
          colors: {
            primary: "#181c1f",
            secondary: "#eaf2f7",
            accent: "#8c9eff",
            error: "#d33d3d",
            warning: "#ffc107",
            info: "#346ddb",
            success: "#4caf50",
          },
        },
        light: {
          dark: false,
          colors: {
            primary: "#ffffff",
            secondary: "#000000",
            accent: "#8c9eff",
            error: "#d33d3d",
            warning: "#ffc107",
            info: "#346ddb",
            success: "#4caf50",
          },
        },
      },
    },
  });

  nuxtApp.vueApp.use(vuetify);
});
