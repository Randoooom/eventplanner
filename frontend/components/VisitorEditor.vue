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
  <FormFloatingDialog icon="mdi-plus-thick" v-model="dialog">
    <template #activator="{ activate }">
      <slot name="activator" :activate="activate" />
    </template>

    <v-card-title>
      {{ $t("visitor.add") }}
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
      </v-row>
    </v-card-text>

    <v-card-actions>
      <v-btn
        variant="text"
        color="accent"
        :disabled="disabled(username)"
        @click="save"
      >
        {{ $t("form.confirm") }}
      </v-btn>
    </v-card-actions>
  </FormFloatingDialog>
</template>

<script setup lang="ts">
import {
  required,
  ref,
  watch,
  disabled,
  useNuxtApp,
  useLocalePath,
} from "#imports";
import { notificationEmitter } from "~/stores/notifications";
import clipboard from "clipboardy";

const localePath = useLocalePath();
const { $surrealdb } = useNuxtApp();
const notifications = notificationEmitter();
const emit = defineEmits(["refresh"]);
const props = defineProps({
  event: { type: String, required: true },
});
const dialog = ref(false);
const username = ref("");

watch(dialog, (value: boolean) => {
  if (value) username.value = "";
});

async function save() {
  try {
    let visitor = await $surrealdb.connection
      .query(
        'CREATE visitor CONTENT { "username": $username, "event": type::thing("event", $event) }',
        {
          username: username.value,
          event: props.event.split(":")[1],
        }
      )
      .then((value: any) => value[0]);
    visitor = Object.fromEntries(visitor);

    // copy the voting link
    await clipboard.write(
      `https://${document.location.hostname}${localePath("/vote")}?visitor=${
        visitor.id.split(":")[1]
      }`
    );

    notifications.attachNotification({
      icon: "mdi-check",
      color: "success",
      title: "visitor.success.title",
      content: "visitor.success.description",
    });
    emit("refresh");
  } catch (e) {
    console.log(e);
    notifications.attachNotification({
      icon: "mdi-alert",
      color: "error",
      title: "form.save.error",
    });
  }

  dialog.value = false;
}
</script>
