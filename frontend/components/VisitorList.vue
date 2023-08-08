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
  <FormDialog v-model="dialog">
    <template #activator="{ activate }">
      <slot name="activator" :activate="activate" />
    </template>

    <v-card-title>
      {{ $t("visitor.list.title") }}
    </v-card-title>

    <v-card-text class="overflow-visible">
      <v-table id="visitors">
        <thead>
          <tr>
            <th>{{ $t("visitor.list.name") }}</th>
            <th>{{ $t("visitor.list.voted") }}</th>
            <th class="d-flex justify-end align-center">
              {{ $t("visitor.list.actions") }}
            </th>
          </tr>
        </thead>
        <tbody>
          <template v-if="visitors">
            <tr v-for="visitor in visitors" :key="visitor.id">
              <td>{{ visitor.username }}</td>
              <td>
                <v-icon
                  v-if="visitor.visitable.length > 0"
                  color="green"
                  icon="mdi-check"
                />
                <v-icon v-else color="red" icon="mdi-close" />
              </td>
              <td class="d-flex justify-end align-center">
                <v-icon
                  @click="remove(visitor)"
                  size="small"
                  color="grey"
                  icon="mdi-delete-outline"
                />
                <v-icon
                  @click="copy(visitor)"
                  size="small"
                  color="grey"
                  icon="mdi-content-copy"
                />
              </td>
            </tr>
          </template>

          <tr>
            <td colspan="3">
              <VisitorEditor :event="event" @refresh="fetch">
                <template #activator="{ activate }">
                  <v-btn
                    color="green"
                    variant="tonal"
                    style="width: 100%"
                    @click="activate"
                  >
                    {{ $t("visitor.list.add") }}
                  </v-btn>
                </template>
              </VisitorEditor>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card-text>
  </FormDialog>
</template>

<script setup lang="ts">
import { watch, ref, useLocalePath, useNuxtApp } from "#imports";
import { Visitor } from "~/composables/types";
import clipboard from "clipboardy";
import { notificationEmitter } from "~/stores/notifications";

const { $surrealdb } = useNuxtApp();
const localePath = useLocalePath();
const notifications = notificationEmitter();

const dialog = ref(false);
const visitors = ref([] as Visitor[]);
const props = defineProps({
  event: { type: String, required: true },
});

watch(dialog, async (value: boolean) => {
  if (value) await fetch();
});

async function fetch() {
  visitors.value = await $surrealdb.connection
    .query("SELECT * FROM visitor WHERE type::string(event) = $event", {
      event: props.event,
    })
    .then((value: any) => value.map(Object.fromEntries));
}

async function copy(visitor: Visitor) {
  // copy the voting link
  await clipboard.write(
    `https://${document.location.hostname}${localePath("/vote")}?visitor=${
      visitor.id.split(":")[1]
    }`
  );

  notifications.attachNotification({
    icon: "mdi-check",
    color: "success",
    title: "visitor.create.response.success.title",
    content: "visitor.create.response.success.description",
    timeout: 1000,
  });
}

async function remove(visitor: Visitor) {
  try {
    await $surrealdb.connection.delete(visitor.id);

    notifications.attachNotification({
      icon: "mdi-check",
      color: "success",
      title: "visitor.remove.response.success.title",
      content: "visitor.remove.response.success.description",
    });
    await fetch();
  } catch {
    notifications.attachNotification({
      icon: "mdi-alert",
      color: "error",
      title: "visitor.remove.response.error.title",
      content: "visitor.remove.response.error.description",
    });
  }
}
</script>

<style lang="sass">
#visitors
  .v-table__wrapper
    overflow: visible !important
</style>
