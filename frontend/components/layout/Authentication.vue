<!--
  -     Copyright (C) 2023 Fritz Ochsmann
  -
  -     This program is free software: you can redistribute it and/or modify
  -     it under the terms of the GNU Affero General Public License as published
  -     by the Free Software Foundation, either version 3 of the License, or
  -     (at your option) any later version.
  -
  -     This program is distributed in the hope that it will be useful,
  -     but WITHOUT ANY WARRANTY; without even the implied warranty of
  -     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
  -     GNU Affero General Public License for more details.
  -
  -     You should have received a copy of the GNU Affero General Public License
  -     along with this program.  If not, see <https://www.gnu.org/licenses/>.
  -->

<template>
  <FormFloatingDialog
    :tooltip="tooltip"
    :icon="icon"
    @click="onClick"
    v-model="dialog"
  >
    <v-card-title>
      {{ $t("layout.login.title") }}
    </v-card-title>

    <v-card-text class="mt-3">
      <v-row>
        <v-col cols="12">
          <v-text-field
            color="accent"
            :label="$t('form.username')"
            v-model="username"
            :rules="[required($t)]"
          />
        </v-col>

        <v-col cols="12">
          <FormPasswordInput
            @keydown.enter="login"
            color="accent"
            v-model="password"
          />
        </v-col>
      </v-row>
    </v-card-text>

    <v-card-actions>
      <v-btn
        variant="text"
        color="accent"
        :disabled="disabled($t, username, password)"
        @click="login"
      >
        {{ $t("layout.login.submit") }}
      </v-btn>

      <v-spacer />

      <v-btn
        @click="dialog = false"
        nuxt
        :to="localePath('/signup')"
        variant="text"
        color="grey"
      >
        {{ $t("layout.login.signup") }}
      </v-btn>
    </v-card-actions>
  </FormFloatingDialog>

  <v-btn
    class="ml-5"
    nuxt
    :to="localePath('/dashboard')"
    variant="tonal"
    color="accent"
    v-if="
      $surrealdb.loggedIn.value &&
      $surrealdb.scope.value &&
      $surrealdb.scope.value === 'account'
    "
  >
    Dashboard
  </v-btn>
</template>

<script lang="ts" setup>
import {
  computed,
  ref,
  disabled,
  required,
  useLocalePath,
  watch,
} from "#imports";
import { notificationEmitter } from "~/stores/notifications";

const { $surrealdb } = useNuxtApp();
const localePath = useLocalePath();
const emitter = notificationEmitter();

const tooltip = computed(() =>
  $surrealdb.loggedIn.value ? "layout.nav.logout" : "layout.nav.login"
);
const icon = computed(() =>
  $surrealdb.loggedIn.value ? "mdi-logout" : "mdi-login"
);

const username = ref("");
const password = ref("");
const dialog = ref(false);

watch(dialog, (value: boolean) => {
  if (value) {
    username.value = "";
    password.value = "";
  }
});

async function login() {
  try {
    await $surrealdb.login(username.value, password.value);

    emitter.attachNotification({
      color: "success",
      content: "layout.login.success.description",
      icon: "mdi-check",
      title: "layout.login.success.title",
    });

    dialog.value = false;
    username.value = "";
    password.value = "";
    await useRouter().push(localePath("/dashboard"));
  } catch {
    emitter.attachNotification({
      color: "error",
      content: "layout.login.error.description",
      icon: "mdi-alert",
      title: "layout.login.error.title",
    });
  }
}

async function onClick() {
  if ($surrealdb.loggedIn.value) {
    try {
      await $surrealdb.logout();

      emitter.attachNotification({
        color: "success",
        content: "layout.logout.success.description",
        icon: "mdi-check",
        title: "layout.logout.success.title",
      });
    } catch {}
  }
}
</script>
